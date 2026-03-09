use tokio::net::TcpStream;
use tokio::task;
use tokio::sync::Semaphore;
use tokio::time::timeout;
use clap::Parser;

#[derive(Parser)]
#[command(name = "rusty-port-scanner")]
#[command(about = "A fast async port scanner")]
struct Args {
    /// Target IP or hostname
    ip: String,
    /// Start port
    start_port: u16,
    /// End port
    end_port: Option<u16>,
    /// Timeout in seconds    #[arg(short, long, default_value_t = 5)]
    timeout: u64,
    /// Maximum concurrent connections    #[arg(short, long, default_value_t = 30)]
    concurrency: usize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut handles = vec![];
    println!("[+] Start Scanning ");
    for port in args.start_port..=args.end_port.unwrap_or(args.start_port) {
        let ip = args.ip.to_string();
        let handle = task::spawn(async move {
            match TcpStream::connect((ip, port)).await {
                Ok(_) => println!("[+] Port {} is open", port),
                Err(_) => (),
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();

    }
     println!("[+] Scanning Completed");
}