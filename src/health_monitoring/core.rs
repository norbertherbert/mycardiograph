use sysinfo::{
    // Components, 
    Disks, Networks, System,
    RefreshKind, CpuRefreshKind, MemoryRefreshKind
};

use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct HealthData {
    pub cpu_usage: f32,
    pub memory: MemoryInfo,
    pub networks: Vec<NetworkInfo>,
    pub disks: Vec<DiskInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub memory_usage: f32,
    pub total_memory: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuInfo {
    pub name: String,
    pub cpu_usage: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub name: String,
    pub total_received: u64,
    pub total_transmitted: u64,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,
    pub disk_space_usage: f32,
    pub total_space: u64,
}


pub async fn collect_health_data() -> HealthData {

    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything())
    );

    // Wait a bit because CPU usage is based on diff.
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_all();

    // let cpus = sys
    //     .cpus()
    //     .iter()
    //     .map(|c| { CpuInfo {
    //         name: c.name().to_string(),
    //         cpu_usage: c.cpu_usage() / 100.0,
    //     }})
    //     .collect();

    let cpu_usage = (sys
        .cpus()
        .iter()
        .map(|c| c.cpu_usage())
        .sum::<f32>() / 100_f32) / sys.cpus().len() as f32;

    let memory = MemoryInfo{ 
        memory_usage: sys.used_memory() as f32 / sys.total_memory() as f32,
        total_memory: sys.total_memory(),
    };

    // let swap = SwapInfo{
    //     swap_usage: sys.used_swap() as f32 / sys.total_swap() as f32,
    //     total_swap: sys.total_swap(),
    // };

    let networks = Networks::new_with_refreshed_list()
        .iter()
        .map(|(interface_name, nwk_data)| { NetworkInfo {
            name: interface_name.to_string(),
            total_received: nwk_data.total_received(),
            total_transmitted: nwk_data.total_transmitted(),
        }})
        .collect();

    let disks = Disks::new_with_refreshed_list()
        .iter()
        .map(|d| { DiskInfo {
            name: d.name().to_string_lossy().to_string(),
            disk_space_usage: (d.total_space() - d.available_space()) as f32 / d.total_space() as f32,
            total_space: d.total_space(),
        }})
        .collect();

    HealthData {
        cpu_usage,
        memory,
        networks,
        disks,
    }

}
