// Palimpsest Time and Duration System

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(pub u64); // Seconds since Unix epoch (1970-01-01T00:00:00Z)

impl Timestamp {
    pub const ZERO: Timestamp = Timestamp(0);

    pub fn from_secs(secs: u64) -> Self {
        Timestamp(secs)
    }

    pub fn as_secs(&self) -> u64 {
        self.0
    }

    pub fn add_duration(&self, dur: Duration) -> Self {
        Timestamp(self.0.saturating_add(dur.as_secs()))
    }

    pub fn sub_timestamp(&self, earlier: Timestamp) -> Duration {
        Duration::from_secs(self.0.saturating_sub(earlier.0))
    }

    /// Parse ISO8601 / RFC3339 formatted date-time string (e.g. "2026-03-01T00:00:00Z" or "2026-03-01")
    pub fn parse_iso(s: &str) -> Result<Self, String> {
        let trimmed = s.trim();
        // Check if numeric seconds
        if let Ok(secs) = trimmed.parse::<u64>() {
            return Ok(Timestamp(secs));
        }

        // Format: YYYY-MM-DD or YYYY-MM-DDTHH:MM:SS...
        let parts: Vec<&str> = trimmed.split('T').collect();
        let date_part = parts[0];
        let time_part = if parts.len() > 1 { parts[1] } else { "00:00:00" };

        let ymd: Vec<&str> = date_part.split('-').collect();
        if ymd.len() != 3 {
            return Err(format!("Invalid date format (expected YYYY-MM-DD): {}", s));
        }

        let year: i64 = ymd[0].parse().map_err(|_| format!("Invalid year: {}", ymd[0]))?;
        let month: u32 = ymd[1].parse().map_err(|_| format!("Invalid month: {}", ymd[1]))?;
        let day: u32 = ymd[2].parse().map_err(|_| format!("Invalid day: {}", ymd[2]))?;

        if month < 1 || month > 12 || day < 1 || day > 31 {
            return Err(format!("Date out of range: {}", s));
        }

        let time_clean = time_part.trim_end_matches('Z');
        let hms: Vec<&str> = time_clean.split(':').collect();
        let hour: u32 = if !hms.is_empty() { hms[0].parse().unwrap_or(0) } else { 0 };
        let min: u32 = if hms.len() > 1 { hms[1].parse().unwrap_or(0) } else { 0 };
        let sec: u32 = if hms.len() > 2 {
            // handle optional fractional seconds
            hms[2].split('.').next().unwrap_or("0").parse().unwrap_or(0)
        } else { 0 };

        let days = days_from_civil(year, month, day);
        if days < 0 {
            return Err(format!("Date before Unix epoch: {}", s));
        }

        let total_secs = (days as u64) * 86400 + (hour as u64) * 3600 + (min as u64) * 60 + (sec as u64);
        Ok(Timestamp(total_secs))
    }

    /// Formats as ISO8601 UTC string (e.g. "2026-03-01T00:00:00Z")
    pub fn to_iso(&self) -> String {
        let secs = self.0;
        let days = (secs / 86400) as i64;
        let rem_secs = (secs % 86400) as u32;

        let hour = rem_secs / 3600;
        let min = (rem_secs % 3600) / 60;
        let sec = rem_secs % 60;

        let (year, month, day) = civil_from_days(days);
        format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z", year, month, day, hour, min, sec)
    }
}

// Howard Hinnant's algorithm for civil calendar calculations
fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let mut y = y;
    let m = m as i64;
    let d = d as i64;
    y -= if m <= 2 { 1 } else { 0 };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400; // [0, 399]
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1; // [0, 365]
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy; // [0, 146096]
    era * 146097 + doe - 719468 // 719468 = days from 0000-03-01 to 1970-01-01
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // [0, 399]
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32; // [1, 12]
    let y = y + if m <= 2 { 1 } else { 0 };
    (y, m, d)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration(pub u64); // In seconds

impl Duration {
    pub const ZERO: Duration = Duration(0);

    pub fn from_secs(secs: u64) -> Self {
        Duration(secs)
    }

    pub fn as_secs(&self) -> u64 {
        self.0
    }

    /// Parse duration like "300s", "10m", "2h", "30d", "1y"
    pub fn parse_str(s: &str) -> Result<Self, String> {
        let s = s.trim();
        if s.is_empty() {
            return Err("Empty duration".to_string());
        }

        let (num_part, unit_part) = s.split_at(
            s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len())
        );

        let num: u64 = num_part.parse().map_err(|_| format!("Invalid number in duration: {}", num_part))?;

        let multiplier = match unit_part {
            "" | "s" | "sec" | "secs" => 1,
            "m" | "min" | "mins" => 60,
            "h" | "hr" | "hrs" | "hours" => 3600,
            "d" | "day" | "days" => 86400,
            "w" | "week" | "weeks" => 86400 * 7,
            "y" | "year" | "years" => 86400 * 365,
            other => return Err(format!("Unknown duration unit: {}", other)),
        };

        Ok(Duration(num * multiplier))
    }

    pub fn format_human(&self) -> String {
        let secs = self.0;
        if secs == 0 {
            return "0s".to_string();
        }
        if secs % 86400 == 0 {
            return format!("{}d", secs / 86400);
        }
        if secs % 3600 == 0 {
            return format!("{}h", secs / 3600);
        }
        if secs % 60 == 0 {
            return format!("{}m", secs / 60);
        }
        format!("{}s", secs)
    }
}
