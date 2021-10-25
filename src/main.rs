use tokio::net::{TcpListener, TcpStream};
use tokio::io::Interest;

const SERVER_ADDRESS: &str = "127.0.0.1:11037";
const BUFFER_MAX_LEN: usize = 1024;

async fn read_from_client(stream: &TcpStream, buf: &mut [u8]) -> std::io::Result<usize> {
    let _ready = match stream.ready(Interest::READABLE).await {
        Ok(x) => x,
        Err(err) => return Err(err),
    };

    // The stream should be readable if an error was not raised.
    let len = stream.try_read(buf)?;

    Ok(len)
}

async fn write_to_client(stream: &TcpStream, buf: &[u8]) -> std::io::Result<usize> {
    let _ready = match stream.ready(Interest::WRITABLE).await {
        Ok(x) => x,
        Err(err) => return Err(err),
    };

    // The stream should be writable if an error was not raised.
    let len = stream.try_write(buf)?;

    Ok(len)
}

async fn process_client(stream: TcpStream) {
    let mut buf = [0; BUFFER_MAX_LEN];

    match read_from_client(&stream, &mut buf).await {
        Ok(len) => {
            println!("Read {} bytes, the message is: {}", len, String::from_utf8_lossy(&buf[..len]));
            match write_to_client(&stream, &buf[..len]).await {
                Ok(len) => println!("Sent back {} bytes!", len),
                Err(err) => eprintln!("Write failed: {}", err),
            }
        }
        Err(err) => eprintln!("Read failed: {}", err),
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
