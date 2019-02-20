use chrono::{offset::TimeZone, Duration, Local};

use crate::utils::fail::AppError;

const FORMAT_ERROR: &str = "Please provide the time format as HH:MM:SS";
const ARG_NAME: &str = "timestamp";

pub fn hms_to_u64_fwd(string: &str) -> Result<u64, AppError> {
    let hmw = str_to_hms(string)?;

    let dur = hms_to_dur(hmw);

    let datetime = Local::now().checked_add_signed(dur).expect("Some or stackoverflow");

    Ok(datetime.timestamp() as u64)
}

pub fn hms_to_u64_rvs(string: &str) -> Result<u64, AppError> {
    let hmw = str_to_hms(string)?;

    let dur = hms_to_dur(hmw);

    let datetime = Local::now().checked_sub_signed(dur).expect("Some or stackoverflow");

    Ok(datetime.timestamp() as u64)
}

pub fn u64_to_hms_fwd(timestamp: u64) -> String {
    let now = Local::now();
    let then = Local.timestamp(timestamp as i64, 0);

    dur_to_hms(then - now)
}

pub fn u64_to_hms_rvs(timestamp: u64) -> String {
    let now = Local::now();
    let then = Local.timestamp(timestamp as i64, 0);

    dur_to_hms(now - then)
}

fn dur_to_hms(duration: Duration) -> String {
    let total_sec = duration.num_seconds();

    let hours = total_sec / (60 * 60);
    let remaining = total_sec % (60 * 60);
    let minutes = remaining / 60;
    let seconds = remaining % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn hms_to_dur(hms: [u64; 3]) -> Duration {
    let h = Duration::hours(hms[2] as i64);
    let m = Duration::minutes(hms[1] as i64);
    let s = Duration::seconds(hms[0] as i64);

    h + m + s
}

fn str_to_hms(string: &str) -> Result<[u64; 3], AppError> {
    let segments: Vec<&str> = string.split(':').collect();

    if segments.len() != 3 {
        return Err(AppError::ParseFailure {
            arg: String::from(ARG_NAME),
            reason: String::from(FORMAT_ERROR),
        });
    }

    if segments.iter().any(|s| s.len() > 2 || s.len() < 1) {
        return Err(AppError::ParseFailure {
            arg: String::from(ARG_NAME),
            reason: String::from(FORMAT_ERROR),
        });
    }

    let mut hms = [0u64, 0u64, 0u64];

    for (i, seg) in segments.iter().enumerate() {
        match u64::from_str_radix(seg, 10) {
            Ok(time) => {
                hms[i] = time;
            }
            Err(_) => {
                return Err(AppError::ParseFailure {
                    arg: String::from(ARG_NAME),
                    reason: String::from(FORMAT_ERROR),
                });
            }
        }
    }

    Ok(hms)
}
