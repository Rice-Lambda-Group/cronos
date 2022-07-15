use log::{info, trace};
use strigoi::logging::init;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    init().unwrap();
    info!("Server started up");
    // simple echo server with proper error handleing
    match TcpListener::bind("127.0.0.1:8087").await {
        Ok(listener) => {
            let (mut socket, addr) = listener.accept().await.expect("accept failed");
            info!("CLIENT ADDRESS: {addr}");

            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::with_capacity(1024);

            loop {
                line.clear();
                let bytes_read = reader.read_line(&mut line).await.expect("read failed");
                trace!("CLIENT MESSAGE: {}", line.trim());
                if bytes_read == 0 {
                    break;
                }

                writer
                    .write_all(line.as_bytes())
                    .await
                    .expect("write failed");
            }
        }

        Err(err) => panic!("{err}"),
    }
}
