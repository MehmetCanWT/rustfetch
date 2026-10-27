use crate::{Info, Module};
use std::fs;

pub struct PowerProfile;

impl Module for PowerProfile {
    fn name(&self) -> &'static str {
        "power_profile"
    }

    fn detect(&self) -> Option<Info> {
        let platform = fs::read_to_string("/sys/firmware/acpi/platform_profile").ok();
        let governor =
            fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor").ok();
        let epp = fs::read_to_string(
            "/sys/devices/system/cpu/cpu0/cpufreq/energy_performance_preference",
        )
        .ok();

        let value = format_power_profile(platform.as_deref(), governor.as_deref(), epp.as_deref())?;
        Some(Info::new("Power Profile", value))
    }
}

pub fn format_power_profile(
    platform_profile: Option<&str>,
    governor: Option<&str>,
    epp: Option<&str>,
) -> Option<String> {
    let prof = platform_profile
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(prettify_profile);
    let gov = governor
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    let epp_val = epp
        .map(str::trim)
        .filter(|s| !s.is_empty() && *s != "default")
        .map(str::to_string);

    match (prof, gov, epp_val) {
        (Some(p), Some(g), _) => {
            if p.eq_ignore_ascii_case(&g) {
                Some(p)
            } else {
                Some(format!("{p} ({g})"))
            }
        }
        (Some(p), None, _) => Some(p),
        (None, Some(g), Some(e)) => {
            let pretty_g = prettify_profile(&g);
            if g.eq_ignore_ascii_case(&e) {
                Some(pretty_g)
            } else {
                Some(format!("{pretty_g} ({e})"))
            }
        }
        (None, Some(g), None) => Some(prettify_profile(&g)),
        (None, None, Some(e)) => Some(prettify_profile(&e)),
        (None, None, None) => None,
    }
}

fn prettify_profile(raw: &str) -> String {
    match raw.trim().to_ascii_lowercase().as_str() {
        "performance" => "Performance".to_string(),
        "balanced" | "balance_performance" => "Balanced".to_string(),
        "balance_power" => "Balanced Power".to_string(),
        "low-power" | "power" | "powersave" => "Power Saver".to_string(),
        "quiet" => "Quiet".to_string(),
        "schedutil" => "Schedutil".to_string(),
        "ondemand" => "OnDemand".to_string(),
        other => {
            let mut chars = other.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_profile_module_name() {
        assert_eq!(PowerProfile.name(), "power_profile");
    }

    #[test]
    fn test_format_power_profile() {
        assert_eq!(
            format_power_profile(Some("balanced\n"), Some("powersave\n"), None),
            Some("Balanced (powersave)".to_string())
        );
        assert_eq!(
            format_power_profile(Some("performance\n"), Some("performance\n"), None),
            Some("Performance".to_string())
        );
        assert_eq!(
            format_power_profile(None, Some("schedutil\n"), Some("balance_performance\n")),
            Some("Schedutil (balance_performance)".to_string())
        );
        assert_eq!(format_power_profile(None, None, None), None);
    }
}
