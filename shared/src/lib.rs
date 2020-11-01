use std::convert::TryInto;
use std::io::{self, Read, Write};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use pb_jelly;

/// Send the given proto over the wire to be read by read_next_message.
pub fn send_message<W, T>(writer: &mut W, proto: &T) -> Result<(), io::Error> 
where W: Write, T : pb_jelly::Message {

    let ser_proto = proto.serialize_to_vec();
    let msg_size_usize = ser_proto.len();
    let msg_size: u32 = msg_size_usize.try_into().expect("<32 bit system?");

    writer.write_u32::<BigEndian>(msg_size)?;
    writer.write_all(&ser_proto)
}

/// Reads the next seriaized proto from the given stream and deserializes it. Assumes this was sent
/// by send_message.
pub fn read_next_message<R, T>(reader: &mut R) -> Result<T, io::Error>
where R: Read, T : pb_jelly::Message {

    let msg_size: usize = reader.read_u32::<BigEndian>()?.try_into().expect("<32bit system?");
    let mut read_vec = vec![0u8; msg_size as usize];

    reader.read_exact(&mut read_vec)?;

    T::deserialize_from_slice(read_vec.as_slice())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use proto_chatroom::chatroom::*;

    #[test]
    fn round_trip() {
        let initial_proto = ServerMessageWrapper{
            inner_message: Some(ServerMessageWrapper_InnerMessage::ChatMsg(RecvChat{msg: String::from("msg"), username: String::from("username")}))
        };

        let mut vec_buff: Cursor<Vec<u8>> = Cursor::new(vec![]);
        send_message(&mut vec_buff, &initial_proto).expect("serialize failed");
        
        vec_buff.set_position(0);
        let parsed_msg: ServerMessageWrapper = read_next_message(&mut vec_buff).expect("deserialize failed");

        assert_eq!(parsed_msg, initial_proto);
    }
}
