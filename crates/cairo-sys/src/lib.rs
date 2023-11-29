// Copyright 2017 TiKV Project Authors. Licensed under Apache-2.0.

// #[cfg(target_os = "linux")]
// mod cgroup;
// pub mod cpu_time;
// pub mod disk;
// pub mod inspector;
// pub mod ioload;
// pub mod thread;

// re-export some traits for ease of use
use std::{
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};

pub const HIGH_PRI: i32 = -1;
const CPU_CORES_QUOTA_ENV_VAR_KEY: &str = "TIKV_CPU_CORES_QUOTA";

static GLOBAL_MEMORY_USAGE: AtomicU64 = AtomicU64::new(0);
static MEMORY_USAGE_HIGH_WATER: AtomicU64 = AtomicU64::new(u64::MAX);



/// Get the current global memory usage in bytes. Users need to call
/// `record_global_memory_usage` to refresh it periodically.
pub fn get_global_memory_usage() -> u64 {  // kb
    record_global_memory_usage();
    GLOBAL_MEMORY_USAGE.load(Ordering::Acquire) / 1024
}

pub fn printCurTimeStr() -> String {
    use chrono::Local;
    let current_time = Local::now();
    current_time.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}

#[cfg(target_os = "linux")]
pub fn page_size() -> u64 {
    unsafe { libc::sysconf(libc::_SC_PAGE_SIZE) as u64 }
}


/// Record the current global memory usage of the process.
#[cfg(target_os = "linux")]
pub fn record_global_memory_usage() {
    let s = procinfo::pid::statm_self().unwrap();
    let usage = s.resident * page_size() as usize;
    GLOBAL_MEMORY_USAGE.store(usage as u64, Ordering::Release);
}

#[cfg(not(target_os = "linux"))]
pub fn record_global_memory_usage() {
    GLOBAL_MEMORY_USAGE.store(0, Ordering::Release);
}

/// Register the high water mark so that `memory_usage_reaches_high_water` is
/// available.
pub fn register_memory_usage_high_water(mark: u64) {
    MEMORY_USAGE_HIGH_WATER.store(mark, Ordering::Release);
}

pub fn memory_usage_reaches_high_water(usage: &mut u64) -> bool {
    // fail_point!("memory_usage_reaches_high_water", |_| true);
    *usage = get_global_memory_usage();
    *usage >= MEMORY_USAGE_HIGH_WATER.load(Ordering::Acquire)
}

fn limit_cpu_cores_quota_by_env_var(quota: f64) -> f64 {
    match std::env::var(CPU_CORES_QUOTA_ENV_VAR_KEY)
        .ok()
        .and_then(|value| value.parse().ok())
    {
        Some(env_var_quota) if quota.is_sign_positive() => f64::min(quota, env_var_quota),
        _ => quota,
    }
}
