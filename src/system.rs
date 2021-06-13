use serde::{Deserialize, Serialize};
use std::thread::sleep;
use systemstat::{Duration, Platform, System};

#[derive(Serialize, Deserialize)]
pub struct DiskInfo {
    pub drive: String,
    pub free: f32,
}

#[derive(Serialize, Deserialize)]
pub struct SystemStat {
    pub cpu_load: Option<f32>,
    pub memery_free: Option<u64>,
    pub mount: Vec<DiskInfo>,
}

fn cpu_load(system: &System) -> Option<f32> {
    let cpu = system.cpu_load_aggregate().ok()?;
    sleep(Duration::from_secs(1));
    Some(cpu.done().ok()?.idle)
}

pub fn system_stat() -> SystemStat {
    let system = System::new();
    let mount: Vec<DiskInfo> = system
        .mounts()
        .unwrap_or_default()
        .iter()
        .map(|f| DiskInfo {
            drive: f.fs_mounted_on.to_string(),
            free: f.free.as_u64() as f32 / f.total.as_u64() as f32,
        })
        .collect();

    SystemStat {
        cpu_load: cpu_load(&system),
        memery_free: system.memory().and_then(|m| Ok(m.free.as_u64())).ok(),
        mount: mount,
    }
}
