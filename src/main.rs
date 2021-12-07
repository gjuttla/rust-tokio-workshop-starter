use tokio::{net::TcpListener, io::{AsyncWriteExt, AsyncReadExt, BufReader, AsyncBufReadExt}};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("localhost:8001").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Listening on {}", addr);

        tokio::spawn( async move {
            loop {
                let (reader, mut writer) = socket.split();
                let mut reader = BufReader::new(reader);
                let mut buffer = String::new();

                if reader.read_line(&mut buffer).await.is_err() || buffer.trim() == "exit" {
                    println!("Disconnected {}", addr);
                    break;
                }
                writer.write_all(buffer.as_bytes()).await.unwrap();
            }
            println!("Connection closed on {}", addr);
        });
    }
}