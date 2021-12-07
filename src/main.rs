use std::net::SocketAddr;

use tokio::{net::TcpListener, io::{AsyncWriteExt, AsyncReadExt, BufReader, AsyncBufReadExt}, sync::broadcast};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("localhost:8001").await?;

    let (tx, rx) = broadcast::channel::<(SocketAddr, String)>(100);

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Listening on {}", addr);

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn( async move {
            loop {
                let (reader, mut writer) = socket.split();
                let mut reader = BufReader::new(reader);
                let mut buffer = String::new();

                tokio::select! {
                    msg = rx.recv() => {
                        let (other_addr, msg) = msg.unwrap();
                        if other_addr != addr {
                            writer.write_all(format!("{}: {}", other_addr, msg).as_bytes()).await.unwrap();    
                        }
                    }
                    result = reader.read_line(&mut buffer) => {
                        if result.is_err() || buffer.trim() == "exit" {
                            println!("Disconnected {}", addr);
                            break;
                        }
                        tx.send((addr, buffer.clone())).unwrap();
                    }
                }                
            }
            println!("Connection closed on {}", addr);
        });
    }
}