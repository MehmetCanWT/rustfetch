use crate::{Info, Module};
use std::fs;
use std::path::Path;

pub struct NetIo;

impl Module for NetIo {
    fn name(&self) -> &'static str {
        "net_io"
    }

    fn detect(&self) -> Option<Info> {
        let (iface, rx_bytes, tx_bytes) = read_active_interface_bytes()?;
        Some(Info::new(
            format!("Network ({iface})"),
            format_net_totals(rx_bytes, tx_bytes),
        ))
    }
}

pub fn format_bytes_human(bytes: u64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = 1024.0 * 1024.0;
    const GIB: f64 = 1024.0 * 1024.0 * 1024.0;
    const TIB: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0;

    let b = bytes as f64;
    if b >= TIB {
        format!("{:.2} TiB", b / TIB)
    } else if b >= GIB {
        format!("{:.2} GiB", b / GIB)
    } else if b >= MIB {
        format!("{:.1} MiB", b / MIB)
    } else if b >= KIB {
        format!("{:.1} KiB", b / KIB)
    } else {
        format!("{bytes} B")
    }
}

pub fn format_net_totals(rx_bytes: u64, tx_bytes: u64) -> String {
    format!(
        "↓ {} / ↑ {}",
        format_bytes_human(rx_bytes),
        format_bytes_human(tx_bytes)
    )
}

pub fn read_active_interface_bytes() -> Option<(String, u64, u64)> {
    let net_dir = Path::new("/sys/class/net");
    let entries = fs::read_dir(net_dir).ok()?;

    let mut best: Option<(String, u64, u64, bool)> = None;

    for entry in entries.flatten() {
        let iface = entry.file_name().to_string_lossy().into_owned();
        if iface == "lo"
            || iface.starts_with("docker")
            || iface.starts_with("veth")
            || iface.starts_with("br-")
            || iface.starts_with("virbr")
        {
            continue;
        }

        let path = entry.path();
        let operstate = fs::read_to_string(path.join("operstate"))
            .unwrap_or_default()
            .trim()
            .to_ascii_lowercase();
        let is_up = operstate == "up";

        let rx = fs::read_to_string(path.join("statistics/rx_bytes"))
            .ok()
            .and_then(|s| s.trim().parse::<u64>().ok())
            .unwrap_or(0);
        let tx = fs::read_to_string(path.join("statistics/tx_bytes"))
            .ok()
            .and_then(|s| s.trim().parse::<u64>().ok())
            .unwrap_or(0);

        if rx == 0 && tx == 0 && !is_up {
            continue;
        }

        let replace = match &best {
            None => true,
            Some((_, best_rx, best_tx, best_up)) => {
                if is_up && !*best_up {
                    true
                } else if is_up == *best_up {
                    (rx + tx) > (*best_rx + *best_tx)
                } else {
                    false
                }
            }
        };

        if replace {
            best = Some((iface, rx, tx, is_up));
        }
    }

    best.map(|(iface, rx, tx, _)| (iface, rx, tx))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_net_io_module_name() {
        assert_eq!(NetIo.name(), "net_io");
    }

    #[test]
    fn test_format_net_totals() {
        assert_eq!(
            format_net_totals(2 * 1024 * 1024 * 1024, 512 * 1024 * 1024),
            "↓ 2.00 GiB / ↑ 512.0 MiB"
        );
        assert_eq!(format_net_totals(500, 1024), "↓ 500 B / ↑ 1.0 KiB");
    }
}
