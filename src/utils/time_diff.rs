use chrono::*;

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
    let mut index = 0;

    for seg in segments {
        if let Ok(ref time) = u64::from_str_radix(seg, 10) {
            hms[index] = *time;
        } else { return Err(GnrError::new(FORMAT_ERROR, Handling::Print)); }

        index += 1;
    }

    index = 0;

    let mut timestamp = Local::now().timestamp() as u64;

    for time in hms.iter() {
        match index {
            0 => { timestamp = add_or_sub(timestamp, time * 60 * 60, action); }   // Hours
            1 => { timestamp = add_or_sub(timestamp, time * 60, action); }        // Minutes
            2 => { timestamp = add_or_sub(timestamp, *time as u64, action); }     // Seconds
            _ => break,
        }
        index += 1;
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