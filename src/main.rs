use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt, AsyncReadExt};

const SERVER_ADDRESS: &str = "127.0.0.1:11037";
const BUFFER_MAX_LEN: usize = 1024;

async fn process_client(mut stream: TcpStream) {
    let mut buf = [0u8; BUFFER_MAX_LEN];

    loop {
        match stream.read(&mut buf).await {
            Ok(len) => {
                if len == 0 {
                    // If buffer is too small, also 0 will be returned,
                    // but in this case I think that we should also break the connection.
                    println!("Connection lost...");
                    break;
                } else {
                    println!("Read {} bytes! The message: {}", len, String::from_utf8_lossy(&buf[..len]));

                    if let Err(err) = stream.write_all(&mut buf).await {
                        eprintln!("An error occurred while writing: {}", err);
                        break;
                    }
                    println!("Sent {} bytes back!", len);
                }
            },
            Err(err) => {
                eprintln!("An error occured while reading: {}", err);
                break;
            },
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(SERVER_ADDRESS).await?;

    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                tokio::spawn(async move {
                    process_client(socket).await;
                });
            },
            Err(err) => eprintln!("Listen error: {}", err)
        }
    }
}
