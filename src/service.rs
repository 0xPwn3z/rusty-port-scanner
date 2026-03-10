use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use phf::phf_map;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub static SERVICES: phf::Map<u16, &'static str> = phf_map! {
    20u16 => "FTP Data",
    21u16 => "FTP",
    22u16 => "SSH",
    23u16 => "Telnet",
    25u16 => "SMTP",
    53u16 => "DNS",
    67u16 => "DHCP Server",
    68u16 => "DHCP Client",
    69u16 => "TFTP",
    80u16 => "HTTP",
    88u16 => "Kerberos",
    110u16 => "POP3",
    111u16 => "RPCBind",
    119u16 => "NNTP",
    123u16 => "NTP",
    135u16 => "MS-RPC",
    137u16 => "NetBIOS Name",
    138u16 => "NetBIOS Datagram",
    139u16 => "NetBIOS Session",
    143u16 => "IMAP",
    161u16 => "SNMP",
    162u16 => "SNMP Trap",
    179u16 => "BGP",
    194u16 => "IRC",
    389u16 => "LDAP",
    443u16 => "HTTPS",
    445u16 => "SMB",
    464u16 => "Kerberos Change/Set",
    465u16 => "SMTPS",
    500u16 => "IKE/IPSec",
    512u16 => "rexec",
    513u16 => "rlogin",
    514u16 => "Syslog/rsh",
    515u16 => "LPD/LPR",
    587u16 => "SMTP Submission",
    631u16 => "IPP",
    636u16 => "LDAPS",
    873u16 => "rsync",
    902u16 => "VMware",
    989u16 => "FTPS Data",
    990u16 => "FTPS",
    993u16 => "IMAPS",
    995u16 => "POP3S",
    1080u16 => "SOCKS Proxy",
    1194u16 => "OpenVPN",
    1433u16 => "MSSQL",
    1521u16 => "Oracle DB",
    1723u16 => "PPTP",
    2049u16 => "NFS",
    2181u16 => "Zookeeper",
    2375u16 => "Docker",
    2376u16 => "Docker TLS",
    3000u16 => "Dev Server",
    3268u16 => "LDAP Global Catalog",
    3269u16 => "LDAPS Global Catalog",
    3306u16 => "MySQL",
    3389u16 => "RDP",
    4444u16 => "Metasploit",
    4505u16 => "SaltStack",
    4506u16 => "SaltStack",
    5000u16 => "Flask/Docker Registry",
    5432u16 => "PostgreSQL",
    5555u16 => "ADB Android",
    5601u16 => "Kibana",
    5672u16 => "RabbitMQ",
    5900u16 => "VNC",
    5985u16 => "WinRM HTTP",
    5986u16 => "WinRM HTTPS",
    6379u16 => "Redis",
    6443u16 => "Kubernetes API",
    7001u16 => "WebLogic",
    8080u16 => "HTTP Alt",
    8443u16 => "HTTPS Alt",
    8888u16 => "Jupyter",
    9000u16 => "SonarQube/PHP-FPM",
    9090u16 => "Prometheus",
    9200u16 => "Elasticsearch",
    9300u16 => "Elasticsearch Cluster",
    9418u16 => "Git",
    11211u16 => "Memcached",
    27017u16 => "MongoDB",
    27018u16 => "MongoDB Shard",
    47001u16 => "WinRM",
    49152u16 => "Windows RPC Dynamic",
};

pub fn detect_service(port: u16) -> &'static str {
    SERVICES.get(&port).copied().unwrap_or("Unknown")
}

pub async fn probe_service(stream: &mut TcpStream, port: u16, timeout_duration: Duration) -> Option<String> {
    let probe = match port {
        80 | 8080 | 8888 => "GET / HTTP/1.0\r\n\r\n",
        _ => return None,
    };

    // Send the probe
    stream.write_all(probe.as_bytes()).await.ok()?;

    let mut buffer = vec![0u8; 4096];
    match timeout(timeout_duration, stream.read(&mut buffer)).await {
        Ok(Ok(n)) if n > 0 => {
            let response = String::from_utf8_lossy(&buffer[..n]).to_string();
            // Extract useful information from the response (e.g., Server header)
            let useful: Vec<&str> = response
                .lines()
                .filter(|line| {
                    line.starts_with("HTTP/") ||
                        line.to_lowercase().starts_with("server:")
                })
                .collect();
            // If we found something useful, return it; otherwise, return None
            if useful.is_empty() {
                None
            } else {
                Some(useful.join(" | "))
            }
        },
        _ => None,
    }
}