
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use crate::banner::grab_banner;

pub async fn check_port(ip: String, port: u16, timeout_duration: Duration) {
    match timeout(timeout_duration, TcpStream::connect((ip, port))).await {
        Ok(Ok(stream)) => {
            let banner = grab_banner(stream, timeout_duration).await;
            if let Some(banner) = banner {
                println!("[+] Port {} is open | {}", port, banner.trim());
            } else {
                println!("[+] Port {} is open | no banner", port)
            }
        },
        _ => (),
    }
}