/// Formats hour and minute into 12-hour time with AM/PM.
///
/// Handles SEPTA's quirky time format where hours can exceed 24
/// (e.g., 25:30 means 1:30 AM the next day).
pub fn format_time(hour: u8, minute: u8) -> String {
    let (adjusted_hour, meridian) = match hour {
        0..=11 => (if hour == 0 { 12 } else { hour }, "AM"),
        12 => (12, "PM"),
        13..=23 => (hour - 12, "PM"),
        24 => (12, "AM"),
        _ => (hour - 24, "AM"), // 25+ wraps to next day
    };
    format!("{:02}:{:02} {}", adjusted_hour, minute, meridian)
}

/// Parses time string in "HH:MM" format.
pub fn parse_time(time: &str) -> String {
    let parts: Vec<&str> = time.split(':').collect();
    let hour = parts.first().and_then(|h| h.parse().ok()).unwrap_or(0);
    let minute = parts.get(1).and_then(|m| m.parse().ok()).unwrap_or(0);
    format_time(hour, minute)
}

/// Parses datetime string in "YYYY-MM-DD HH:MM:SS" format, extracting just the time.
pub fn parse_datetime(datetime: Option<&str>) -> String {
    let Some(dt) = datetime else {
        return "None".to_owned();
    };

    let time_part = dt.split_whitespace().nth(1).unwrap_or(dt);
    parse_time(time_part)
}
