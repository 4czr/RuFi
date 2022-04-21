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
        Ok(Ok(_)) => println!("\x1B[32mOpen Port:\x1B[0m {}", port),
        _ => {}
    }
}

fn get_ports() -> Box<dyn Iterator<Item = u16>> {
    Box::new((1..=u16::MAX).into_iter())
}
