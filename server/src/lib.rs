use std::sync::mpsc;
use std::thread;
use std::net{self, TcpStream};
use proto_chatroom::chatroom as proto;

#[derive(Copy, Clone)]
struct ServerRunnerConfig {
    port: u16,
}

struct ServerRunner {
    config: ServerRunnerConfig,

    threads: Vec<thread::JoinHandle<()>>,
    tcp_streams: Vec<TcpStream>,
}

// Specific pre-parsed commands that the stdin reader will send to the main thread to run
#[derive(Debug)]
enum StdinCommand {}

// Ops various threads will ask the "main thread" to run
#[derive(Debug)]
enum MainOp {
    NewConnection(TcpStream),
    NewChat(proto::SendChat),
    UserLeft{username: String},
    HandleStdin(StdinCommand),
}

impl ServerRunner {
    fn new(port: u16) -> ServerRunner {
        ServerRunner {
            config: ServerRunnerConfig{port},
            threads: vec![],
            tcp_streams: vec![],
        }
    }

    // Thread closures

    fn connection_loop(config: ServerRunnerConfig, main_op_channel: mpsc::Sender<MainOp>) {

    }

    fn user_read_loop(tcp_stream: TcpStream) {
    }

    // Server helpers

    // Handles NewConnection on the main thread by starting up a thread to read the values, registering any metadata, etc.
    fn handle_new_connection(&mut self, tcp_stream: TcpStream) {

        let cloned_stream = tcp_stream.try_clone().expect("failed to dupe tcp stream");
        self.threads.push(thread::spawn(move || {
            ServerRunner::user_read_loop(cloned_stream)
        }));
        self.tcp_streams.push(tcp_stream);
    }

    /// Run the server. Blocks until exit
    fn run(mut self) {
        let (ops_sender, ops_receiver) = mpsc::channel::<MainOp>();
        {
            let config = self.config;
            let ops_sender_clone = ops_sender.clone();
            // TODO: We don't/can't easily gracefully bring down the connection-accepting loop if its doing blocking reads
            // If we do, then push into self.threads
            thread::spawn(move || {
                ServerRunner::connection_loop(config, ops_sender_clone);
            });
        }


        loop {
            let maybe_op = ops_receiver.recv();
            if let Err(recv_err) = maybe_op {
                eprintln!("all senders dead? {}", recv_err);
                break
            }

            match maybe_op.unwrap() {
                MainOp::NewConnection(tcp_stream) => self.handle_new_connection(tcp_stream),
                o => panic!("unhandled op! {:?}", o),
            }
        }

        // TODO: signal all threads to stop

        for join_handle in self.threads.into_iter() {
            if let Err(e) = join_handle.join() {
                eprintln!("thread panicked: {:?}", e)
            }
        }

    }
}

