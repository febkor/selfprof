use std::{
    ffi::{CStr, CString},
    time::{SystemTime, UNIX_EPOCH},
};

use active_win_pos_rs::get_active_window;
use user_idle::UserIdle;

// Offset from UNIX EPOCH to 2020-01-01 00:00:00 UTC
const APP_EPOCH_OFFSET: u64 = 1577836800;

pub mod storage;

#[derive(Debug, Clone, PartialEq)]
pub struct Snap {
    pub time: u32, // ~136y
    pub idle: u16, // ~18h
    pub index: u8, // lookback index for name
    pub name: CString,
}

impl Snap {
    pub fn count_bytes(self: &Snap) -> usize {
        4 + 2 + 1 + self.name.as_bytes_with_nul().len()
    }

    pub fn to_bytes(self: &Snap) -> Vec<u8> {
        let name = self.name.as_bytes_with_nul();
        let mut res = Vec::<u8>::with_capacity(self.count_bytes());
        res.extend(&self.time.to_be_bytes());
        res.extend(&self.idle.to_be_bytes());
        res.extend(&self.index.to_be_bytes());
        res.extend(name);
        res
    }

    pub fn from_bytes(buf: &[u8]) -> Snap {
        let time = u32::from_be_bytes(*<&[u8; 4]>::try_from(&buf[0..4]).expect("enough bytes"));
        let idle = u16::from_be_bytes(*<&[u8; 2]>::try_from(&buf[4..6]).expect("enough bytes"));
        let index = u8::from_be_bytes(*<&[u8; 1]>::try_from(&buf[6..7]).expect("enough bytes"));
        let name: CString = CStr::from_bytes_until_nul(&buf[7..])
            .expect("valid C string")
            .into();
        Snap {
            time,
            idle,
            index,
            name,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let s = Snap {
            time: 123456,
            idle: 78,
            index: 9,
            name: CString::new("test").unwrap(),
        };
        let bytes = s.to_bytes();
        let r = Snap::from_bytes(&bytes);
        assert_eq!(r, s);
    }
}
