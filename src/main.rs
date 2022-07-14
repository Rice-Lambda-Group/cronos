use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    // simple echo server
    let listener = TcpListener::bind("localhost:8080")
        .await
        .expect("bind failed");
    let (mut socket, _addr) = listener.accept().await.expect("accept failed");

    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::with_capacity(1024);

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await.expect("read failed");

        if bytes_read == 0 {
            break;
        }

        writer
            .write_all(line.as_bytes())
            .await
            .expect("write failed");
    }
}
