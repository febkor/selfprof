use std::{
    mem::size_of,
    time::{SystemTime, UNIX_EPOCH},
};

use active_win_pos_rs::get_active_window;
use user_idle::UserIdle;

// Offset from UNIX EPOCH to 2020-01-01 00:00:00 UTC
const APP_EPOCH_OFFSET: u64 = 1577836800;

pub mod storage;

#[derive(Debug, Clone)]
pub struct Snap {
    pub time: u32, // ~136y
    pub name: u32, // ~4bn, excessive
    pub idle: u16, // ~18h
    pub pad: u16,
}

impl Snap {
    pub fn to_bytes(self: &Snap) -> [u8; 12] {
        const N: usize = size_of::<Snap>();
        let mut res = [0u8; N];
        res[0..4].copy_from_slice(&self.time.to_be_bytes());
        res[4..8].copy_from_slice(&self.name.to_be_bytes());
        res[8..10].copy_from_slice(&self.idle.to_be_bytes());
        res
    }
}

pub fn time_now() -> u32 {
    let utc = SystemTime::now();
    let unix = utc.duration_since(UNIX_EPOCH).expect("valid unix time");
    let epoch_secs = unix.as_secs() - APP_EPOCH_OFFSET;
    epoch_secs.try_into().unwrap_or(std::u32::MIN) // default to 0
}

pub fn idle_time() -> u16 {
    let idle = UserIdle::get_time().unwrap();
    let s = idle.as_seconds();
    s.try_into().unwrap_or(std::u16::MAX)
}

pub fn active_app_name() -> String {
    match get_active_window() {
        Ok(w) => format!("{} : {}", w.process_name, w.title),
        Err(_) => "".to_string(),
    }
}
