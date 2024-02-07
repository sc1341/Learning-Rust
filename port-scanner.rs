use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;
use std::net::{TcpStream, SocketAddr};
use std::str::FromStr;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <target_ip> <(optional) start_port> <(optional) end_port>", args[0]);
        std::process::exit(1);
    }

    let target_ip = &args[1];
    let start_port: u16 = args.get(2).map(|s| s.parse().unwrap_or(1)).unwrap_or(1);
    let end_port: u16 = args.get(3).map(|s| s.parse().unwrap_or(65535)).unwrap_or(65535);

    scan_ports(target_ip, start_port, end_port);
}

fn scan_ports(target_ip: &str, start_port: u16, end_port: u16) {
    const MAX_THREADS: usize = 100; // Maximum number of threads to run at once
    let (sender, receiver) = mpsc::channel();
    let mut handles = vec![];

    for port in start_port..=end_port {
        let sender_clone = sender.clone();
        let target_ip = target_ip.to_string();

        let handle = thread::spawn(move || {
            let socket_address = SocketAddr::from_str(&format!("{}:{}", &target_ip, port)).unwrap();
            let timeout = Duration::from_millis(100);

            if TcpStream::connect_timeout(&socket_address, timeout).is_ok() {
                let _ = sender_clone.send(port);
            }
        });

        handles.push(handle);

        // When the number of running threads reaches the limit, wait for them to finish before continuing
        if handles.len() >= MAX_THREADS {
            for handle in handles.drain(..) {
                let _ = handle.join();
            }
        }
    }

    // Wait for any remaining handles to complete
    for handle in handles {
        let _ = handle.join();
    }

    let mut open_ports = Vec::new();
    while let Ok(port) = receiver.try_recv() {
        open_ports.push(port);
    }

    if !open_ports.is_empty() {
        open_ports.sort_unstable(); // Sort the ports before printing
        println!("Open ports on {}: {:?}", target_ip, open_ports);
    } else {
        println!("No open ports found on {}.", target_ip);
    }
}
