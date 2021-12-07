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
                let _bytes_read = reader.read_line(&mut buffer).await.unwrap();
                if buffer.trim() == "exit" {
                    break;
                }
                writer.write_all(buffer.as_bytes()).await.unwrap()
            }
            println!("Connection closed on {}", addr);
        });
    }
}