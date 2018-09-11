use chrono::*;

const FORMAT_ERROR: &str = "Please provide the time format as HH:MM:SS";

pub fn hms_to_u64(string: &str, action: &AddOrSub) -> Result<u64, &'static str> {
    let segments: Vec<&str> = string.split(':').collect();

    if segments.len() != 3 {
        return Err(FORMAT_ERROR);
    }

    if segments.iter().any(|s| (*s).len() > 2 || (*s).len() < 1) {
        return Err(FORMAT_ERROR);
    }

    let mut hms = [0, 0, 0];
    let mut index = 0;

    for seg in segments {
        if let Ok(ref time) = u64::from_str_radix(seg, 10) {
            hms[index] = *time;
        } else { return Err(FORMAT_ERROR); }

        index += 1;
    }

    index = 0;

    let mut timestamp = Local::now().timestamp() as u64;

    for time in hms.iter() {
        match index {
            0 => { timestamp = add_or_sub(timestamp, time * 60 * 60, action); }   // Hours
            1 => { timestamp = add_or_sub(timestamp, time * 60, action); }        // Minutes
            2 => { timestamp = add_or_sub(timestamp, *time as u64, action); }     // Seconds
            _ => break
        }
        index += 1;
    }

    Ok(timestamp)
}

pub fn add_or_sub(current: u64, new: u64, action: &AddOrSub) -> u64 {
    match action {
        AddOrSub::Add => current + new,
        AddOrSub::Sub => current - new,
    }
}

pub enum AddOrSub {
    Add,
    Sub,
}