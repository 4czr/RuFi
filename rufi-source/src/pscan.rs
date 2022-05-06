use futures::{stream, StreamExt};
use std::{net::{IpAddr, SocketAddr, ToSocketAddrs},time::Duration,};
use tokio::net::TcpStream;

#[tokio::main]
pub async fn scanner(target: &str) -> Result<(), anyhow::Error>  {
    println!("Scanning {} for all 65535 ports", target);
    let socket_addresses: Vec<SocketAddr> = format!("{}:0", target).to_socket_addrs()?.collect();

    if socket_addresses.is_empty() {
        return Err(anyhow::anyhow!("Socket address list is empty"));
    }

    scan(socket_addresses[0].ip(), 1002, 30).await;
    Ok(())
}

async fn scan(target: IpAddr, concurrency: usize, timeout: u64) {
    let ports = stream::iter(get_ports());
    
    ports
        .for_each_concurrent(concurrency, |port| scan_port(target, port, timeout))
        .await;
}

async fn scan_port(target: IpAddr, port: u16, timeout: u64) {
    let timeout = Duration::from_secs(timeout);
    let socket_address = SocketAddr::new(target.clone(), port);

    match tokio::time::timeout(timeout, TcpStream::connect(&socket_address)).await {
        Ok(Ok(_)) => {
            print_ports(port);
        },
        _ => {}
    }
}

fn get_ports() -> Box<dyn Iterator<Item = u16>> {
    Box::new((1..=u16::MAX).into_iter())
}

// Probably going to change this to a HashMap in the future

fn print_ports(port: u16) {
    match port {
        80 => println!("\x1B[32mOpen Port:\x1B[0m {} (HTTP)", port),
        8080 => println!("\x1B[32mOpen Port:\x1B[0m {} (HTTP)", port),
        88 => println!("\x1B[32mOpen Port:\x1B[0m {} (Kerberos)", port),
        443 => println!("\x1B[32mOpen Port:\x1B[0m {} (HTTPS/SSL)", port),
        8443 => println!("\x1B[32mOpen Port:\x1B[0m {} (HTTPS/SSL)", port),
        20 => println!("\x1B[32mOpen Port:\x1B[0m {} (FTP)", port),
        21 => println!("\x1B[32mOpen Port:\x1B[0m {} (FTP)", port),
        22 => println!("\x1B[32mOpen Port:\x1B[0m {} (SSH/FTPS)", port),
        23 => println!("\x1B[32mOpen Port:\x1B[0m {} (Telnet)", port),
        25 => println!("\x1B[32mOpen Port:\x1B[0m {} (SMTP)", port),
        26 => println!("\x1B[32mOpen Port:\x1B[0m {} (SMTP)", port),
        53 => println!("\x1B[32mOpen Port:\x1B[0m {} (DNS)", port),
        69 => println!("\x1B[32mOpen Port:\x1B[0m {} (TFTP)", port),
        110 => println!("\x1B[32mOpen Port:\x1B[0m {} (POP3)", port),
        137 => println!("\x1B[32mOpen Port:\x1B[0m {} (SMB)", port),
        139 => println!("\x1B[32mOpen Port:\x1B[0m {} (SMB)", port),
        143 => println!("\x1B[32mOpen Port:\x1B[0m {} (IMAP)", port),
        389 => println!("\x1B[32mOpen Port:\x1B[0m {} (LDAP)", port),
        445 => println!("\x1B[32mOpen Port:\x1B[0m {} (Active Directory/SMB)", port),
        587 => println!("\x1B[32mOpen Port:\x1B[0m {} (SMTP SSL)", port),
        636 => println!("\x1B[32mOpen Port:\x1B[0m {} (LDAPS)", port),
        993 => println!("\x1B[32mOpen Port:\x1B[0m {} (IMAP SSL)", port),
        995 => println!("\x1B[32mOpen Port:\x1B[0m {} (POP3 SSL)", port),
        2049 => println!("\x1B[32mOpen Port:\x1B[0m {} (NFS)", port),
        2077 => println!("\x1B[32mOpen Port:\x1B[0m {} (WebDAV/WebDisk)", port),
        2078 => println!("\x1B[32mOpen Port:\x1B[0m {} (WebDAV/WebDisk SSL)", port),
        2082 => println!("\x1B[32mOpen Port:\x1B[0m {} (cPanel)", port),
        2083 => println!("\x1B[32mOpen Port:\x1B[0m {} (cPanel SSL)", port),
        2086 => println!("\x1B[32mOpen Port:\x1B[0m {} (WHM)", port),
        2095 => println!("\x1B[32mOpen Port:\x1B[0m {} (WebMail)", port),
        2096 => println!("\x1B[32mOpen Port:\x1B[0m {} (WebMail SSL)", port),
        3268 => println!("\x1B[32mOpen Port:\x1B[0m {} (ActiveDirectory LDAP)", port),
        3269 => println!("\x1B[32mOpen Port:\x1B[0m {} (ActiveDirectory LDAPS)", port),
        3306 => println!("\x1B[32mOpen Port:\x1B[0m {} (MySQL)", port),
        4172 => println!("\x1B[32mOpen Port:\x1B[0m {} (PCoIP (AWS))", port),
        5000 => println!("\x1B[32mOpen Port:\x1B[0m {} (UPnP)", port),
        5555 => println!("\x1B[32mOpen Port:\x1B[0m {} (Android Debug Bridge)", port),
        _ => println!("\x1B[32mOpen Port:\x1B[0m {}", port)
    }
}
