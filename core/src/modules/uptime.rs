use crate::{Info, Module};

pub struct Uptime;

impl Module for Uptime {
    fn name(&self) -> &'static str {
        "uptime"
    }

    fn detect(&self) -> Option<Info> {
        let content = std::fs::read_to_string("/proc/uptime").ok()?;
        let uptime = parse_uptime(&content)?;
        Some(Info::new("Uptime", uptime))
    }
}

pub fn parse_uptime(content: &str) -> Option<String> {
    let seconds: f64 = content.split_whitespace().next()?.parse().ok()?;
    if !seconds.is_finite() || seconds < 0.0 {
        return None;
    }

    let total = seconds as u64;
    let days = total / 86400;
    let hours = (total % 86400) / 3600;
    let mins = (total % 3600) / 60;
    let secs = total % 60;

    let formatted = if days > 0 {
        let d = if days == 1 { "day" } else { "days" };
        if hours > 0 {
            let h = if hours == 1 { "hour" } else { "hours" };
            format!("{days} {d}, {hours} {h}")
        } else if mins > 0 {
            let m = if mins == 1 { "min" } else { "mins" };
            format!("{days} {d}, {mins} {m}")
        } else {
            format!("{days} {d}")
        }
    } else if hours > 0 {
        let h = if hours == 1 { "hour" } else { "hours" };
        if mins > 0 {
            let m = if mins == 1 { "min" } else { "mins" };
            format!("{hours} {h}, {mins} {m}")
        } else {
            format!("{hours} {h}")
        }
    } else if mins > 0 {
        let m = if mins == 1 { "min" } else { "mins" };
        format!("{mins} {m}")
    } else {
        let s = if secs == 1 { "sec" } else { "secs" };
        format!("{secs} {s}")
    };

    Some(formatted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_uptime_fixture() {
        // Realistic Linux / Fedora /proc/uptime fixture
        let fixture = "8145.67 15234.89";
        let result = parse_uptime(fixture);
        assert_eq!(result.as_deref(), Some("2 hours, 15 mins"));
    }

    #[test]
    fn test_parse_uptime_days_and_hours() {
        // 3 days, 5 hours, 12 mins -> 3 * 86400 + 5 * 3600 + 12 * 60 = 277920
        let fixture = "277920.45 987654.32";
        let result = parse_uptime(fixture);
        assert_eq!(result.as_deref(), Some("3 days, 5 hours"));
    }

    #[test]
    fn test_parse_uptime_singular_units() {
        // 1 day, 1 hour -> 86400 + 3600 = 90000
        let fixture = "90000.00 12345.00";
        assert_eq!(parse_uptime(fixture).as_deref(), Some("1 day, 1 hour"));

        // 1 hour, 1 min -> 3600 + 60 = 3660
        let fixture_hour_min = "3660.00 1234.00";
        assert_eq!(
            parse_uptime(fixture_hour_min).as_deref(),
            Some("1 hour, 1 min")
        );

        // 1 min
        let fixture_min = "60.00 10.00";
        assert_eq!(parse_uptime(fixture_min).as_deref(), Some("1 min"));

        // 1 sec
        let fixture_sec = "1.00 0.50";
        assert_eq!(parse_uptime(fixture_sec).as_deref(), Some("1 sec"));
    }

    #[test]
    fn test_parse_uptime_minutes_only() {
        let fixture = "300.00 150.00"; // 5 minutes
        assert_eq!(parse_uptime(fixture).as_deref(), Some("5 mins"));
    }

    #[test]
    fn test_parse_uptime_seconds_only() {
        let fixture = "45.12 10.00";
        assert_eq!(parse_uptime(fixture).as_deref(), Some("45 secs"));
    }

    #[test]
    fn test_parse_uptime_invalid() {
        assert_eq!(parse_uptime(""), None);
        assert_eq!(parse_uptime("invalid"), None);
        assert_eq!(parse_uptime("-10.5 20.0"), None);
        assert_eq!(parse_uptime("NaN 20.0"), None);
    }
}
