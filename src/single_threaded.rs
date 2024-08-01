use std::{io::Read, net::TcpListener};

pub fn single_threaded_server(port: &u32){
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
    loop {
        if let Ok(connection) = listener.accept(){
            println!("User ---{}--- connected!!!!!",connection.1);
            let mut stream = connection.0;
            let mut break_on_two = 0;
            loop {
                let mut buff = vec![0;200];
                if let Ok(bytes_read) = stream.read(&mut buff){
                    if bytes_read == 0{
                        // connection is closed ?
                        break;
                    }else if bytes_read == 1{
                        // \r\n ?
                        break_on_two +=1;
                    }else {
                        if let Ok(data) = std::str::from_utf8(&buff[0..bytes_read]){
                            print!("{data}");
                        }else{
                            println!("*_*")
                        }
                        break_on_two = 0;
                    }
                    if break_on_two == 2{
                        // we got 2 \r\n break the connection
                        break;
                    }
                }else{
                    break;
                }
            }
            println!("User ---{}--- disconnected!!!!!",connection.1);
        }
    }
}