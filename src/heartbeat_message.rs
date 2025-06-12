use crate::health_monitoring::HealthData;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HeartbeatMessage {
    pub device_id: String,
    pub timestamp: i64,
    pub heartbeat_interval: u64,
    pub health_data: Option<HealthData>,
}