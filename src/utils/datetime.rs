#![allow(dead_code)]

use jiff::Timestamp;

pub fn format_timestamp(timestamp: &Timestamp) -> String {
    timestamp.to_string()
}
