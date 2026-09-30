use crate::{Info, Module};

pub struct Host;

impl Module for Host {
    fn name(&self) -> &'static str {
        "host"
    }

    fn detect(&self) -> Option<Info> {
        let value = read_host()?;
        Some(Info::new("Host", value))
    }
}

pub fn read_host() -> Option<String> {
    let vendor = std::fs::read_to_string("/sys/devices/virtual/dmi/id/sys_vendor")
        .or_else(|_| std::fs::read_to_string("/sys/class/dmi/id/sys_vendor"))
        .ok();
    let product = std::fs::read_to_string("/sys/devices/virtual/dmi/id/product_name")
        .or_else(|_| std::fs::read_to_string("/sys/class/dmi/id/product_name"))
        .ok();

    if vendor.is_none() && product.is_none() {
        return None;
    }

    parse_host(
        vendor.as_deref().unwrap_or(""),
        product.as_deref().unwrap_or(""),
    )
}

pub fn parse_host(vendor: &str, product: &str) -> Option<String> {
    let v = vendor.trim();
    let p = product.trim();

    match (v.is_empty(), p.is_empty()) {
        (true, true) => None,
        (false, true) => Some(v.to_string()),
        (true, false) => Some(p.to_string()),
        (false, false) => {
            if p.to_lowercase().starts_with(&v.to_lowercase()) {
                Some(p.to_string())
            } else {
                Some(format!("{v} {p}"))
            }
        }
    }
}

pub fn combine_host(vendor: &str, product: &str) -> Option<String> {
    parse_host(vendor, product)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_host_combined() {
        assert_eq!(
            parse_host("Acer", "Nitro AN515-57"),
            Some("Acer Nitro AN515-57".to_string())
        );
        assert_eq!(
            parse_host("LENOVO", "ThinkPad T480"),
            Some("LENOVO ThinkPad T480".to_string())
        );
        assert_eq!(
            combine_host("ASUSTeK COMPUTER INC.", "ROG Zephyrus G14"),
            Some("ASUSTeK COMPUTER INC. ROG Zephyrus G14".to_string())
        );
    }

    #[test]
    fn test_parse_host_with_newlines() {
        // Files in /sys often contain a trailing newline
        let vendor_fixture = "Acer\n";
        let product_fixture = "Nitro AN515-57\n";
        assert_eq!(
            parse_host(vendor_fixture, product_fixture),
            Some("Acer Nitro AN515-57".to_string())
        );
    }

    #[test]
    fn test_parse_host_no_duplication() {
        assert_eq!(
            parse_host("Framework", "Framework Laptop 13"),
            Some("Framework Laptop 13".to_string())
        );
        assert_eq!(
            parse_host("Dell", "Dell XPS 13 9300"),
            Some("Dell XPS 13 9300".to_string())
        );
    }

    #[test]
    fn test_parse_host_partial() {
        assert_eq!(
            parse_host("Custom Vendor", ""),
            Some("Custom Vendor".to_string())
        );
        assert_eq!(
            parse_host("", "Custom Motherboard"),
            Some("Custom Motherboard".to_string())
        );
    }

    #[test]
    fn test_parse_host_empty() {
        assert_eq!(parse_host("", ""), None);
        assert_eq!(parse_host("   \n", "\t\r\n"), None);
    }

    #[test]
    fn test_host_detect() {
        let host = Host;
        assert_eq!(host.name(), "host");
        if let Some(info) = host.detect() {
            assert_eq!(info.label, "Host");
            assert!(!info.value.is_empty());
        }
    }
}
