mod scanner;
mod banner;
mod output;

use tokio::task;
use tokio::sync::Semaphore;
use std::sync::Arc;
use std::time::Duration;
use clap::Parser;
use crate::scanner::check_port;

#[derive(Parser)]
#[command(name = "rusty-port-scanner")]
#[command(about = "A fast async port scanner")]
struct Args {
    /// Target IP or hostname
    #[arg(short, long)]
    address: String,
    /// Start port
    #[arg(short, long)]
    start_port: u16,
    /// End port
    #[arg(short, long)]
    end_port: Option<u16>,
    /// Timeout in seconds
    #[arg(short, long, default_value_t = 5)]
    timeout: u64,
    /// Maximum concurrent connections
    #[arg(short, long, default_value_t = 30)]
    concurrency: usize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut handles = vec![];
    println!("[+] Start Scanning ");
    let semaphore = Arc::new(Semaphore::new(args.concurrency));
    for port in args.start_port..=args.end_port.unwrap_or(args.start_port) {
        let semaphore_clone = semaphore.clone();
        let ip = args.address.to_string();
        let handle = task::spawn(async move {
            let _permit = semaphore_clone.acquire().await.unwrap();
            check_port(ip, port, Duration::from_secs(args.timeout)).await;
            drop(_permit);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
     println!("[+] Scanning Completed");
}