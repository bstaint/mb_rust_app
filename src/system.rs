use serde::{Deserialize, Serialize};
use std::thread::sleep;
use systemstat::{Duration, Platform, System};

#[derive(Serialize, Deserialize)]
pub struct SystemStat {
    pub cpu_load: Option<f32>,
    pub memery_free: Option<u64>,
}

fn cpu_load(system: &System) -> Option<f32> {
    let cpu = system.cpu_load_aggregate().ok()?;
    sleep(Duration::from_secs(1));
    Some(cpu.done().ok()?.idle)
}

pub fn system_stat() -> SystemStat {
    let system = System::new();

    SystemStat {
        cpu_load: cpu_load(&system),
        memery_free: system.memory().and_then(|m| Ok(m.free.as_u64())).ok(),
    }
}
