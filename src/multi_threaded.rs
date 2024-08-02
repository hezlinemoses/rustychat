use std::{ io::{Read, Write}, net::{SocketAddr, TcpListener, TcpStream}, os::fd::{AsFd, AsRawFd}, sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread, time::Duration};

pub fn multi_threaded_server(port: &u32){
    let server = TcpListener::bind(format!("127.0.0.1:{}",port))
    .expect("Port error?, use a different port");

    let (sender, receiver) = mpsc::channel::<Event>();

    // writer thread
    thread::spawn(||{
        handle_writing_to_all(receiver);
    });
    loop {
        if let Ok(connection) = server.accept(){
            let stream = connection.0;
            let sender = sender.clone();
            // start a thread and read from the stream
            thread::spawn(move ||{
                process_stream(stream,connection.1,sender);
            });
        }
    }
}

fn process_stream(stream: TcpStream, client: SocketAddr, sender: Sender<Event>){
    println!("Client {client} connected...");
    let stream = Arc::new(stream);
    let _ = sender.send(Event::Connection(Arc::clone(&stream)));
    let mut stream = &*stream;

    let _ =stream.write("Welcome to Sree RajaRajeswari adholokam...\n".as_bytes());
    loop {
        let mut buff = vec![0;200];
        if let Ok(bytes_read) = stream.read(&mut buff){
            if bytes_read == 0{
                // connection is closed ?
                println!("Connection closed by client {client}");
                break;
            }else if bytes_read == 1{
                // \r\n ?
                println!("Client {client} disconnected...");
                let _ = stream.write("disconnected...".as_bytes());
                break;
            }else {
                if let Ok(data) = std::str::from_utf8(&buff[0..bytes_read]){
                    let _ = sender.send(Event::Message((data.to_string(),stream.as_raw_fd() as u32)));
                    print!("{data}");
                }else{
                    println!("*_*")
                }
            }
        }else{
            println!("Client {client} disconnected...");
            break;
        }
    }
}

enum Event{
    Message((String,u32)),
    Connection(Arc<TcpStream>)
}


/// needs to be called when a new connection is opened
/// needs to be called when a stream gets a message
/// should be run in a seperate thread
fn handle_writing_to_all(receiver: Receiver<Event>){
    let mut streams: Vec<Option<Arc<TcpStream>>> = Vec::new();

    while let Some(event) = receiver.iter().next(){
        match event {
            Event::Message((msg,fd)) => {
                streams.iter_mut()
                .for_each(|some_stream| {
                    if let Some(ref stream) = some_stream{
                        let mut stream2 = &**stream;
                        if fd != stream2.as_raw_fd() as u32{
                            if let Err(_) = stream2.write_all(msg.as_bytes()){
                                *some_stream = None;
                            };
                        }
                    };
                });
            },
            Event::Connection(stream) => {streams.push(Some(stream))},
        }
    }
}