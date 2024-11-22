use std::{io::{Read, Write}, net::TcpListener};

pub fn single_threaded_server(port: &u32){
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
    loop {
        if let Ok(connection) = listener.accept(){
            println!("User ---{}--- connected!!!!!",connection.1);
            
            let mut stream = connection.0;
            let mut shut_down_counter = 0;
            loop {
                let mut buff = vec![0;200];
                if let Ok(bytes_read) = stream.read(&mut buff){
                    if bytes_read == 0{
                        // connection is closed ?
                        println!("Connection closed by client {}",connection.1);
                        break;
                    }else if bytes_read == 1{
                        shut_down_counter +=1;
                        if shut_down_counter == 2 {
                            let _ = stream.write("disconnected...".as_bytes());
                            let _ = stream.shutdown(std::net::Shutdown::Both);
                            break;
                        }
                    }else {
                        if let Ok(data) = std::str::from_utf8(&buff[0..bytes_read]){
                            print!("{data}");
                        }else{
                            println!("*_*")
                        }
                    }
                }else{
                    break;
                }
            }
            println!("User ---{}--- disconnected!!!!!",connection.1);
        }
    }
}