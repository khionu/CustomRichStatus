use chrono::Local;

use utils::gnr_error::{GnrError, Handling};

const FORMAT_ERROR: &str = "Please provide the time format as HH:MM:SS";

pub fn hms_to_u64(string: &str, action: &AddOrSub) -> Result<u64, Box<GnrError>> {
    let segments: Vec<&str> = string.split(':').collect();

    if segments.len() != 3 {
        return Err(GnrError::new(FORMAT_ERROR, Handling::Print));
    }

    if segments.iter().any(|s| (*s).len() > 2 || (*s).len() < 1) {
        return Err(GnrError::new(FORMAT_ERROR, Handling::Print));
    }

    let mut hms = [0, 0, 0];

    for (index, seg) in segments.iter().enumerate() {
        match u64::from_str_radix(seg, 10) {
            Ok(time) => { hms[index] = time; },
            Err(_err) => { return Err(GnrError::new(FORMAT_ERROR, Handling::Print)); },
        }
    }

    let mut timestamp = Local::now().timestamp() as u64;

    for (index, time) in hms.iter().enumerate() {
        match index {
            0 => { timestamp = add_or_sub(timestamp, time * 60 * 60, action); }   // Hours
            1 => { timestamp = add_or_sub(timestamp, time * 60, action); }        // Minutes
            2 => { timestamp = add_or_sub(timestamp, *time as u64, action); }     // Seconds
            _ => break,
        }
    }

    Ok(timestamp)
}

pub fn add_or_sub(current: u64, diff: u64, action: &AddOrSub) -> u64 {
    match action {
        AddOrSub::Add => current + diff,
        AddOrSub::Sub => current - diff,
    }
}

pub enum AddOrSub {
    Add,
    Sub,
}