use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use crate::banner::grab_banner;
use crate::service::{detect_service, probe_service};

pub async fn check_port(ip: String, port: u16, timeout_duration: Duration) {
    match timeout(timeout_duration, TcpStream::connect((ip, port))).await {
        Ok(Ok(mut stream)) => {
            let service = detect_service(port);

            // 1. Try to grab the banner first
            let result = grab_banner(&mut stream, timeout_duration).await;

            // 2. If banner grab fails, try service-specific probes
            let result = if result.is_none() {
                probe_service(&mut stream, port, timeout_duration).await
            } else {
                result
            };

            // 3. Print the result
            match result {
                Some(info) => println!("[+] Port {}/tcp open | {}", port, info.trim()),
                None       => println!("[+] Port {}/tcp open | {}", port, service),
            }
        },
        _ => (),
    }
}