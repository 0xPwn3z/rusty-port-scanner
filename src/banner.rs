use tokio::io::AsyncReadExt;
use tokio::time::timeout;
use std::time::Duration;
use tokio::net::TcpStream;

pub async fn grab_banner(mut stream: TcpStream, timeout_duration: Duration) -> Option<String> {
    let mut buffer = vec![0u8; 1024];
    match timeout(timeout_duration, stream.read(&mut buffer)).await {
        Ok(Ok(n)) if n > 0 => {
            // Convert the bytes to a string, handling potential UTF-8 errors
            let banner = String::from_utf8_lossy(&buffer[..n]).to_string();
            Some(banner)
        },
        _ => None,
    }
}