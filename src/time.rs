// Timestamps and durations.
//
// Palimpsest runs on a virtual clock so that lifetimes and time-travel queries
// are reproducible. Nothing here reads the wall clock.

use crate::lexer::duration_unit_secs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(pub u64);

impl Timestamp {
    pub const ZERO: Timestamp = Timestamp(0);

    pub fn from_secs(secs: u64) -> Self {
        Timestamp(secs)
    }

    pub fn as_secs(self) -> u64 {
        self.0
    }

    pub fn plus(self, dur: Duration) -> Self {
        Timestamp(self.0.saturating_add(dur.0))
    }

    pub fn since(self, earlier: Timestamp) -> Duration {
        Duration(self.0.saturating_sub(earlier.0))
    }

    /// Accepts `2026-08-15`, `2026-08-15T14:30`, `2026-08-15T14:30:00Z`, or a
    /// bare epoch second count.
    pub fn parse_iso(text: &str) -> Result<Self, String> {
        let text = text.trim();
        if text.is_empty() {
            return Err("Empty date".into());
        }

        if text.chars().all(|c| c.is_ascii_digit()) {
            return text
                .parse::<u64>()
                .map(Timestamp)
                .map_err(|_| format!("`{}` is not a valid date", text));
        }

        let (date_part, time_part) = match text.split_once('T') {
            Some((d, t)) => (d, t),
            None => (text, "00:00:00"),
        };

        let mut fields = date_part.split('-');
        let year: i64 = fields
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| format!("`{}` is missing a four-digit year", text))?;
        let month: u32 = fields
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| format!("`{}` is missing a month", text))?;
        let day: u32 = fields
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| format!("`{}` is missing a day", text))?;

        if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
            return Err(format!("`{}` is not a real calendar date", text));
        }

        let clock = time_part.trim_end_matches('Z');
        let mut parts = clock.split(':');
        let hour: u64 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        let minute: u64 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        let second: u64 = parts
            .next()
            .and_then(|s| s.split('.').next().unwrap_or("0").parse().ok())
            .unwrap_or(0);

        let days = days_from_civil(year, month, day);
        if days < 0 {
            return Err(format!("`{}` is before 1970 and cannot be represented", text));
        }

        Ok(Timestamp(
            days as u64 * 86_400 + hour * 3_600 + minute * 60 + second,
        ))
    }

    pub fn to_iso(self) -> String {
        let (y, m, d) = civil_from_days((self.0 / 86_400) as i64);
        let rem = self.0 % 86_400;
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            y,
            m,
            d,
            rem / 3_600,
            (rem % 3_600) / 60,
            rem % 60
        )
    }

    /// Calendar date alone, or date plus time when the time is not midnight.
    pub fn to_date(self) -> String {
        let (y, m, d) = civil_from_days((self.0 / 86_400) as i64);
        let rem = self.0 % 86_400;
        if rem == 0 {
            format!("{:04}-{:02}-{:02}", y, m, d)
        } else {
            format!(
                "{:04}-{:02}-{:02} {:02}:{:02}",
                y,
                m,
                d,
                rem / 3_600,
                (rem % 3_600) / 60
            )
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration(pub u64);

impl Duration {
    pub const ZERO: Duration = Duration(0);

    pub fn from_secs(secs: u64) -> Self {
        Duration(secs)
    }

    pub fn as_secs(self) -> u64 {
        self.0
    }

    /// Accepts `90d`, `30 days`, `24h`, or a bare number of seconds.
    pub fn parse_str(text: &str) -> Result<Self, String> {
        let text = text.trim();
        if text.is_empty() {
            return Err("Empty duration".into());
        }

        let split = text
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(text.len());
        let (digits, unit) = text.split_at(split);

        let n: u64 = digits
            .parse()
            .map_err(|_| format!("`{}` does not start with a number", text))?;

        let unit = unit.trim();
        if unit.is_empty() {
            return Ok(Duration(n));
        }

        duration_unit_secs(unit)
            .map(|mult| Duration(n * mult))
            .ok_or_else(|| format!("`{}` is not a unit of time", unit))
    }

    /// A reading of the duration in its largest whole unit. Exact multiples
    /// read plainly; anything else is prefixed with "about", because a
    /// remainder rounded away should look rounded.
    pub fn humanize(self) -> String {
        const UNITS: &[(u64, &str)] = &[
            (31_536_000, "year"),
            (2_592_000, "month"),
            (604_800, "week"),
            (86_400, "day"),
            (3_600, "hour"),
            (60, "minute"),
            (1, "second"),
        ];

        if self.0 == 0 {
            return "0 seconds".into();
        }

        for &(size, name) in UNITS {
            if self.0 < size {
                continue;
            }
            let count = self.0 / size;
            if self.0 % size == 0 {
                let plural = if count == 1 { "" } else { "s" };
                return format!("{} {}{}", count, name, plural);
            }
            // Rounding "90 seconds" down to "about 1 minute" throws away more
            // than it saves, so a lone unit with a remainder defers to the
            // next smaller one.
            if count > 1 {
                return format!("about {} {}s", count, name);
            }
        }

        format!("{} seconds", self.0)
    }
}

// Howard Hinnant's civil-date algorithms.

fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let m = m as i64;
    let d = d as i64;
    let y = y - if m <= 2 { 1 } else { 0 };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    (y + if m <= 2 { 1 } else { 0 }, m, d)
}
