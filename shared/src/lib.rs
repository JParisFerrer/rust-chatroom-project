use anyhow::Result;
use async_stream::stream;
use pb_jelly;
use std::convert::TryInto;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::sync::{broadcast, mpsc};
use tokio_stream::Stream;
//use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, Stream, io::BufReader};

pub struct Shutdown {
    is_shutdown: bool,
    // We listen to this for when to indicate we should shutdown
    shutdown_notify: broadcast::Receiver<()>,
    // We need this so that we can correctly clone Shutdown; Shutdown will never send on this
    // (and too much time was spent on trying to get a Fn() -> broadcast::Receiver to work)
    shutdown_notify_gen: broadcast::Sender<()>,
    // We close this (on drop) once we have acked the shutdown
    shutdown_ack: mpsc::Sender<()>,
}

impl Shutdown {
    pub fn new(shutdown_notify_gen: broadcast::Sender<()>, shutdown_ack: mpsc::Sender<()>) -> Self {
        Shutdown {
            is_shutdown: false,
            shutdown_notify: shutdown_notify_gen.subscribe(),
            shutdown_notify_gen,
            shutdown_ack,
        }
    }

    pub async fn wait_for_shutdown(&mut self) {
        if self.is_shutdown {
            return;
        }

        let _ = self.shutdown_notify.recv().await;
        self.is_shutdown = true;
    }

    pub fn is_shutdown(&self) -> bool {
        self.is_shutdown
    }
}

impl Clone for Shutdown {
    fn clone(&self) -> Self {
        Self {
            is_shutdown: self.is_shutdown,
            shutdown_notify: self.shutdown_notify_gen.subscribe(),
            shutdown_notify_gen: self.shutdown_notify_gen.clone(),
            shutdown_ack: self.shutdown_ack.clone(),
        }
    }
}

pub fn into_proto_stream<R, T>(reader: R) -> impl Stream<Item = T>
where
    R: AsyncRead,
    T: pb_jelly::Message,
{
    stream! {
        // Convert raw reader into a Buffered Reader & Pin it
        let mut reader = Box::pin(BufReader::new(reader));
        loop {
            match read_next_message(&mut reader).await {
                Ok(msg) => yield msg,
                Err(err) => {
                    eprintln!("stream err: {}", err);
                    break;
                }
            }
        }
    }
}

/// Send the given proto over the wire to be read by sync_read_next_message.
pub async fn write_message<W, T>(writer: &mut W, proto: &T) -> Result<()>
where
    W: AsyncWrite + Unpin,
    T: pb_jelly::Message,
{
    let ser_proto = proto.serialize_to_vec();
    let msg_size_usize = ser_proto.len();
    let msg_size: u32 = msg_size_usize.try_into().expect("<32 bit system?");

    writer.write_all(&msg_size.to_be_bytes()).await?;
    writer.write_all(&ser_proto).await.map_err(anyhow::Error::new)
}

/// Reads the next seriaized proto from the given stream and deserializes it. Assumes this was sent
/// by sync_write_message.
pub async fn read_next_message<R, T>(reader: &mut R) -> Result<T>
where
    R: AsyncRead + Unpin,
    T: pb_jelly::Message,
{
    let mut size_slice = [0u8; 4];
    reader.read_exact(&mut size_slice).await?;

    let msg_size: usize = u32::from_be_bytes(size_slice)
        .try_into()
        .expect("<32 byte system?");

    let mut read_vec = vec![0u8; msg_size];

    reader.read_exact(&mut read_vec).await?;

    T::deserialize_from_slice(read_vec.as_slice()).map_err(anyhow::Error::new)
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::io::Cursor;
    use proto_chatroom::chatroom::*;
    use tokio;
    use tokio_stream::{self, StreamExt};

    #[tokio::test]
    async fn round_trip() {
        let initial_proto = ServerMessageWrapper {
            inner_message: Some(ServerMessageWrapper_InnerMessage::ChatMsg(RecvChat {
                msg: String::from("msg"),
                username: String::from("username"),
            })),
        };

        let mut vec_buff: Cursor<Vec<u8>> = Cursor::new(vec![]);
        write_message(&mut vec_buff, &initial_proto)
            .await
            .expect("serialize failed");

        vec_buff.set_position(0);
        let parsed_msg: ServerMessageWrapper = read_next_message(&mut vec_buff)
            .await
            .expect("deserialize failed");

        assert_eq!(parsed_msg, initial_proto);
    }

    #[tokio::test]
    async fn stream_wrapper() {
        let initial_proto = ServerMessageWrapper {
            inner_message: Some(ServerMessageWrapper_InnerMessage::ChatMsg(RecvChat {
                msg: String::from("msg"),
                username: String::from("username"),
            })),
        };
        let other_proto = ServerMessageWrapper {
            inner_message: Some(ServerMessageWrapper_InnerMessage::ChatMsg(RecvChat {
                msg: String::from("other message"),
                username: String::from("username2"),
            })),
        };

        let mut stream = tokio_stream::iter(vec![
            initial_proto.clone(),
            other_proto.clone(),
            initial_proto.clone(),
        ]);
        assert_eq!(stream.next().await, Some(initial_proto.clone()));
        assert_eq!(stream.next().await, Some(other_proto));
        assert_eq!(stream.next().await, Some(initial_proto));
        assert_eq!(stream.next().await, None);
    }
}
