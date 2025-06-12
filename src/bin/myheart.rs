use cardiograph::HeartbeatMessage;
use cardiograph::health_monitoring::collect_health_data;

use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit, OsRng, rand_core::RngCore};
use chrono::Utc;
use std::net::UdpSocket;
use tokio::time;
use std::time::Duration;
use clap::Parser;
use sha2::{Digest, Sha256};


#[derive(Parser, Debug)]
#[command(
    author, 
    version, 
    about="MyHeart - availability monitoring client sending UDP heartbeats", 
    long_about = None
)]
struct Args {

    /// Unique device ID string
    #[arg(long)]
    device_id: String,

    /// IP address of the monitoring server (e.g. 192.168.1.100)
    #[arg(long)]
    server_ip: String,

    /// udp port of the monitoring server (e.g. 9999)
    #[arg(long, default_value_t = 9999)]
    server_port: u16,

    /// Heartbeat interval in seconds (e.g. 60)
    #[arg(long, default_value_t = 30)]
    heartbeat_interval: u64,

    /// Password used to derive AES-256 key
    #[arg(long)]
    password: String,

    /// Include health data in heartbeats
    #[arg(long, default_value_t = false)]
    with_health_data: bool,

}


#[tokio::main]
async fn main() -> std::io::Result<()> {

    let args = Args::parse();
    let hash = Sha256::digest(args.password.as_bytes()); // [u8; 32]
    let key  = Key::<Aes256Gcm>::from_slice(&hash);

    let cipher = Aes256Gcm::new(key);
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    let server_addr = format!("{}:{}", &args.server_ip, &args.server_port);

    let mut interval = time::interval(Duration::from_secs(args.heartbeat_interval));

    loop {

        interval.tick().await;

        let timestamp = Utc::now().timestamp_millis(); // e.g. 1718064039123

        let health_data = if args.with_health_data { 
            Some( collect_health_data().await ) 
        } else { 
            None 
        };
        
        let payload = serde_json::to_string(&HeartbeatMessage{
            device_id: args.device_id.clone(), 
            timestamp,
            heartbeat_interval: args.heartbeat_interval,
            health_data
        }).unwrap();

        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, payload.as_bytes())
            .expect("encryption failed");

        let mut packet = Vec::with_capacity(12 + ciphertext.len());
        packet.extend_from_slice(&nonce_bytes);
        packet.extend_from_slice(&ciphertext);

        socket.send_to(&packet, &server_addr)?;
        println!("✅ Encrypted heartbeat sent to {}", &server_addr);

    }
}
