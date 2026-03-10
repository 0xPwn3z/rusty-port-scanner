mod scanner;
mod banner;
mod output;
mod service;

use tokio::task;
use tokio::sync::Semaphore;
use std::sync::Arc;
use std::time::Duration;
use clap::Parser;
use indicatif::ProgressBar;
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
    // Calculate total ports for progress bar
    let total_ports = (args.end_port.unwrap_or(args.start_port) - args.start_port + 1) as u64;
    let progress_bar = Arc::new(ProgressBar::new(total_ports));
    // Spawn a task to update the progress bar
    for port in args.start_port..=args.end_port.unwrap_or(args.start_port) {
        let semaphore_clone = semaphore.clone();
        let progress_bar_clone = progress_bar.clone();
        let ip = args.address.to_string();
        let handle = task::spawn(async move {
            let _permit = semaphore_clone.acquire().await.unwrap();
            check_port(ip, port, Duration::from_secs(args.timeout), &progress_bar_clone).await;
            progress_bar_clone.inc(1);
            drop(_permit);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
     println!("[+] Scanning Completed");
}