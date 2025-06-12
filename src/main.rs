use cardiograph::HeartbeatMessage;
use cardiograph::logger;

use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use chrono::Utc;
use std::collections::HashMap;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{
    Duration, 
    Instant
};
use std::path::PathBuf;
use clap::Parser;
use sha2::{Digest, Sha256};

type DeviceStatusMap = Arc<Mutex<HashMap<String, DeviceStatus>>>;



#[derive(Parser, Debug)]
#[command(
    author, 
    version, 
    about="MyCardiograph - availability monitoring server receiving UDP heartbeats", 
    long_about = None
)]
struct Args {
    /// IP address of the monitoring server (e.g. 192.168.1.100)
    #[arg(long, default_value = "0.0.0.0")]
    server_ip: String,

    /// UDP port of the monitoring server (e.g. 9999)
    #[arg(long, default_value_t = 9999)]
    server_port: u16,

    /// Password used to derive AES-256 key
    #[arg(long)]
    password: String,

    /// Interval in seconds at which the server checks device status.
    #[arg(long, default_value_t = 10)]
    check_interval: u64,

    /// Console log level
    #[arg(long, value_enum, default_value = "debug")]
    console_log_level: logger::LogLevel,

    /// File log level
    #[arg(long, value_enum, default_value = "debug")]
    file_log_level: logger::LogLevel,
}

#[derive(Debug)]
struct DeviceStatus {
    last_seen: Instant,
    last_timestamp: i64,
    is_up: bool,
    heartbeat_interval: u64,
}


fn main() -> std::io::Result<()> {

    let args = Args::parse();
    let hash = Sha256::digest(args.password.as_bytes()); // [u8; 32]
    let key  = Key::<Aes256Gcm>::from_slice(&hash);

    let cipher = Aes256Gcm::new(key);

    let server_addr = format!("{}:{}", &args.server_ip, &args.server_port);

    let socket = UdpSocket::bind(&server_addr)?;
    socket.set_nonblocking(true)?;


    let device_status_map: DeviceStatusMap = Arc::new(Mutex::new(HashMap::new()));


    let log_config = logger::LogConfig{
        console_logger_level: args.console_log_level,
        file_logger_level: args.file_log_level,
        dir: PathBuf::from("log"),
    };
    let _logger_guard = logger::init(&log_config).unwrap();
    tracing::info!("Logging started!");


    {
        let cipher = cipher.clone();
        let device_status_map = device_status_map.clone();
        thread::spawn(move || {
            let mut buf = [0u8; 1500];
            loop {
                match socket.recv_from(&mut buf) {
                    Ok((n, addr)) if n > 12 => {
                        let (nonce_bytes, ciphertext) = buf[..n].split_at(12);
                        let nonce = Nonce::from_slice(nonce_bytes);
                        match cipher.decrypt(nonce, ciphertext) {
                            Ok(plaintext) => {
                                if let Ok(msg) = String::from_utf8(plaintext) {
                                    if let Ok(heartbeat_message) = serde_json::from_str::<HeartbeatMessage>(&msg) {


                                        let mut map = device_status_map.lock().unwrap();
                                        match map.get_mut(&heartbeat_message.device_id) {



                                            Some(status) => {

                                                if heartbeat_message.timestamp <= status.last_timestamp {
                                                
                                                    tracing::error!(
                                                        device_id=heartbeat_message.device_id, 
                                                        actual_timestamp=heartbeat_message.timestamp, 
                                                        last_timestamp=status.last_timestamp, 
                                                        "Replay detected:"
                                                    );
                                                
                                                } else {

                                                    if status.is_up {

                                                        tracing::debug!(
                                                            device_id=heartbeat_message.device_id,
                                                            "Heartbeat message:"
                                                        );
                                                
                                                        if let Some(ref health_data) = heartbeat_message.health_data {
                                                            let health_data_json = serde_json::to_string(health_data).unwrap();
                                                            tracing::trace!(
                                                                device_id=heartbeat_message.device_id,
                                                                health_data=%health_data_json,
                                                                "Healthdata received:",
                                                            );
                                                        }

                                                    } else {

                                                        status.is_up = true;

                                                        let since_last = status.last_seen.elapsed();
                                                        let was_down_since = (Utc::now() - since_last).format("%Y-%m-%dT%H:%M:%S");
                                                        tracing::info!(
                                                            device_id=heartbeat_message.device_id, 
                                                            %was_down_since,
                                                            "Device got UP:"
                                                        );

                                                    }

                                                    status.last_timestamp = heartbeat_message.timestamp;
                                                    status.heartbeat_interval = heartbeat_message.heartbeat_interval;
                                                    status.last_seen = Instant::now();

                                                }
                                                

                                            }
                                            _ => {

                                                map.insert(heartbeat_message.device_id.clone(), DeviceStatus{
                                                    last_timestamp: heartbeat_message.timestamp, 
                                                    last_seen: Instant::now(),
                                                    is_up: true,
                                                    heartbeat_interval: heartbeat_message.heartbeat_interval,
                                                });

                                                
                                                tracing::info!(
                                                    device_id=heartbeat_message.device_id,
                                                    "New device detected:",
                                                );

                                                if let Some(ref health_data) = heartbeat_message.health_data {
                                                    let health_data_json = serde_json::to_string(health_data).unwrap();
                                                    tracing::trace!(
                                                        device_id=heartbeat_message.device_id,
                                                        health_data=%health_data_json,
                                                        "Healthdata received:",
                                                    );
                                                }

                                            }


                                        }

                                    };

                                }
                            }
                            Err(_) => tracing::error!(
                                sender_addr=%addr, 
                                "Message received with invalid encryption:"
                            ),
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(100));
                    }
                    Err(e) => tracing::error!("recv error: {}", e),
                    _ => {}
                }
            }
        });
    }

    {
        let device_status_map = Arc::clone(&device_status_map);
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(args.check_interval));
            let mut map = device_status_map.lock().unwrap();

            for (device_id, status) in map.iter_mut() {

                let since_last = status.last_seen.elapsed();

                if since_last > Duration::from_secs(status.heartbeat_interval) {
                   
                    if status.is_up {
                        let last_seen = (Utc::now()-since_last).format("%Y-%m-%dT%H:%M:%S");
                        tracing::warn!(
                            device_id, 
                            %last_seen, 
                            "Device went DOWN:"
                        );
                    }

                    status.is_up = false;

                };

            }

        });
    }

    tracing::info!(
        server_addr, 
        "Monitoring server is listening:"
    );

    loop {
        thread::sleep(Duration::from_secs(60));
    }

    // logger::flush(logger_guard);
    // Ok(())

}