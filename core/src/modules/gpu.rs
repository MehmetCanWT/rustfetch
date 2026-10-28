use crate::{cache::Cache, Info, Module};
use std::fs;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PciGpuDevice {
    pub slot: String,
    pub gpu_type: String,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GpuFilter {
    #[default]
    DgpuPriority,
    IgpuOnly,
    All,
    DiscreteOnly,
}

impl GpuFilter {
    pub fn parse(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "igpu" | "integrated" | "i_gpu" | "internal" => Self::IgpuOnly,
            "all" | "both" => Self::All,
            "discrete" | "discrete_only" | "dgpu_only" | "dedicated" | "dedicated_only"
            | "external" | "egpu" => Self::DiscreteOnly,
            _ => Self::DgpuPriority,
        }
    }
}

pub fn filter_gpus(devices: &[PciGpuDevice], filter: GpuFilter) -> Vec<PciGpuDevice> {
    match filter {
        GpuFilter::All => devices.to_vec(),
        GpuFilter::IgpuOnly => devices
            .iter()
            .filter(|d| d.gpu_type == "Integrated")
            .cloned()
            .collect(),
        GpuFilter::DiscreteOnly => devices
            .iter()
            .filter(|d| d.gpu_type == "Discrete")
            .cloned()
            .collect(),
        GpuFilter::DgpuPriority => {
            let discrete: Vec<_> = devices
                .iter()
                .filter(|d| d.gpu_type == "Discrete")
                .cloned()
                .collect();
            if !discrete.is_empty() {
                discrete
            } else {
                devices
                    .iter()
                    .filter(|d| d.gpu_type == "Integrated")
                    .cloned()
                    .collect()
            }
        }
    }
}

pub fn format_gpu_devices(devices: &[PciGpuDevice]) -> Vec<String> {
    let mut gpus = Vec::new();
    for dev in devices {
        let mut parts = Vec::new();
        parts.push(dev.name.clone());
        parts.push(format!("[{}]", dev.gpu_type));

        if let Some(vram) = detect_vram(&dev.slot) {
            parts.push(format!("[{vram}]"));
        }

        if dev.gpu_type == "Discrete" {
            if let Some(status) = detect_runtime_status(&dev.slot) {
                parts.push(format!("[{status}]"));
            }
        }

        gpus.push(parts.join(" "));
    }
    gpus
}

#[derive(Debug, Clone, Default)]
pub struct GpuModule {
    pub filter: GpuFilter,
}

impl GpuModule {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_type(filter_str: &str) -> Self {
        Self {
            filter: GpuFilter::parse(filter_str),
        }
    }

    pub fn with_filter(filter: GpuFilter) -> Self {
        Self { filter }
    }
}

impl Module for GpuModule {
    fn name(&self) -> &'static str {
        "gpu"
    }

    fn detect(&self) -> Option<Info> {
        let static_devices = get_or_cache_static_gpus()?;
        if static_devices.is_empty() {
            return None;
        }

        let filtered_devices = filter_gpus(&static_devices, self.filter);
        if filtered_devices.is_empty() {
            return None;
        }

        let gpus = format_gpu_devices(&filtered_devices);
        if gpus.is_empty() {
            None
        } else {
            Some(Info::new("GPU", gpus.join(", ")))
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DgpuModule;

impl Module for DgpuModule {
    fn name(&self) -> &'static str {
        "dgpu"
    }

    fn detect(&self) -> Option<Info> {
        let static_devices = get_or_cache_static_gpus()?;
        if static_devices.is_empty() {
            return None;
        }

        let filtered_devices = filter_gpus(&static_devices, GpuFilter::DiscreteOnly);
        if filtered_devices.is_empty() {
            return None;
        }

        let gpus = format_gpu_devices(&filtered_devices);
        if gpus.is_empty() {
            None
        } else {
            Some(Info::new("dGPU", gpus.join(", ")))
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct IgpuModule;

impl Module for IgpuModule {
    fn name(&self) -> &'static str {
        "igpu"
    }

    fn detect(&self) -> Option<Info> {
        let static_devices = get_or_cache_static_gpus()?;
        if static_devices.is_empty() {
            return None;
        }

        let filtered_devices = filter_gpus(&static_devices, GpuFilter::IgpuOnly);
        if filtered_devices.is_empty() {
            return None;
        }

        let gpus = format_gpu_devices(&filtered_devices);
        if gpus.is_empty() {
            None
        } else {
            Some(Info::new("iGPU", gpus.join(", ")))
        }
    }
}

pub fn canonicalize_pci_slot(slot: &str) -> String {
    let s = slot.trim();
    if s.is_empty() {
        return String::new();
    }
    if !s.contains(':') || s.matches(':').count() == 1 {
        format!("0000:{s}")
    } else {
        s.to_string()
    }
}

fn get_or_cache_static_gpus() -> Option<Vec<PciGpuDevice>> {
    let cache = Cache::new("gpu");
    if let Some(cached_val) = cache.read(Duration::from_secs(24 * 3600)) {
        if let Some(devices) = parse_static_gpu_cache(&cached_val) {
            if !devices.is_empty() {
                // If sysfs PCI devices directory exists, verify all cached slots are still present.
                // If a GPU was detached/unbinded (e.g. Thunderbolt eGPU unplugged), invalidate cache.
                let pci_dir = std::path::Path::new("/sys/bus/pci/devices");
                let all_present = !pci_dir.exists()
                    || devices.iter().all(|d| {
                        let slot = canonicalize_pci_slot(&d.slot);
                        pci_dir.join(&slot).exists()
                    });
                if all_present {
                    return Some(devices);
                }
            }
        }
    }

    let devices = scan_pci_gpus();
    if !devices.is_empty() {
        let serialized = serialize_static_gpu_cache(&devices);
        cache.write(&serialized);
        Some(devices)
    } else {
        None
    }
}

pub fn is_discrete_gpu(class: &str, vendor: &str) -> bool {
    is_discrete_gpu_detailed("", class, vendor, "")
}

pub fn is_discrete_gpu_detailed(slot: &str, class: &str, vendor: &str, name: &str) -> bool {
    let class_clean = class.trim();
    if class_clean.starts_with("0x0302")
        || class_clean.to_ascii_lowercase().contains("3d controller")
    {
        return true;
    }

    let vendor_clean = vendor.trim().trim_start_matches("0x").to_ascii_lowercase();
    let name_lower = name.to_ascii_lowercase();

    // NVIDIA is always discrete
    if vendor_clean == "10de"
        || name_lower.contains("nvidia")
        || name_lower.contains("geforce")
        || name_lower.contains("quadro")
        || name_lower.contains("rtx")
    {
        return true;
    }

    // Intel Arc / Iris Xe MAX / discrete Xe
    if vendor_clean == "8086" || name_lower.contains("intel") {
        if name_lower.contains("arc ")
            || name_lower.contains("arc(tm)")
            || name_lower.contains("arc pro")
            || name_lower.contains("iris xe max")
            || name_lower.contains("dg1")
            || name_lower.contains("dg2")
            || name_lower.contains("battlemage")
            || name_lower.contains("alchemist")
        {
            return true;
        }
        return false;
    }

    // AMD / ATI
    if vendor_clean == "1002" || name_lower.contains("amd") || name_lower.contains("radeon") {
        // Check hwmon in1_input: APU indicator
        if !slot.is_empty() {
            let canon = canonicalize_pci_slot(slot);
            let pci_path = std::path::Path::new("/sys/bus/pci/devices").join(&canon);
            if is_amd_apu_hwmon(&pci_path) {
                return false;
            }
        }

        // Known APU names
        if is_amd_apu_name(&name_lower) {
            return false;
        }

        // Known discrete names
        if is_amd_discrete_name(&name_lower) {
            return true;
        }

        // On laptops, check if runtime status exists or suspended
        if !slot.is_empty() && detect_runtime_status(slot).is_some() {
            return true;
        }

        // Desktop AMD graphics controllers with 0x0300 (VGA) that are not APUs are discrete
        if !name_lower.is_empty() && !name_lower.contains("integrated") {
            return true;
        }
    }

    false
}

pub fn is_amd_apu_hwmon(pci_device_path: &std::path::Path) -> bool {
    let hwmon_dir = pci_device_path.join("hwmon");
    if let Ok(entries) = fs::read_dir(hwmon_dir) {
        for entry in entries.flatten() {
            if entry.path().join("in1_input").exists() {
                return true;
            }
        }
    }
    false
}

pub fn is_amd_apu_name(name_lower: &str) -> bool {
    name_lower.contains("vega 3")
        || name_lower.contains("vega 6")
        || name_lower.contains("vega 7")
        || name_lower.contains("vega 8")
        || name_lower.contains("vega 9")
        || name_lower.contains("vega 10")
        || name_lower.contains("vega 11")
        || name_lower.contains("radeon 660m")
        || name_lower.contains("radeon 680m")
        || name_lower.contains("radeon 740m")
        || name_lower.contains("radeon 760m")
        || name_lower.contains("radeon 780m")
        || name_lower.contains("radeon 880m")
        || name_lower.contains("radeon 890m")
        || name_lower.contains("renoir")
        || name_lower.contains("cezanne")
        || name_lower.contains("barcelo")
        || name_lower.contains("rembrandt")
        || name_lower.contains("phoenix")
        || name_lower.contains("hawk point")
        || name_lower.contains("strix")
        || name_lower.contains("lucienne")
        || name_lower.contains("mendocino")
        || name_lower.contains("raven ridge")
        || name_lower.contains("picasso")
        || name_lower.contains("carrizo")
        || name_lower.contains("bristol")
        || name_lower.contains("kaveri")
        || name_lower.contains("kabini")
        || name_lower.contains("temash")
        || name_lower.contains("beema")
        || name_lower.contains("mullins")
        || name_lower.contains("apu")
        || (name_lower.contains("radeon graphics") && !name_lower.contains("rx "))
}

pub fn is_amd_discrete_name(name_lower: &str) -> bool {
    name_lower.contains("radeon rx")
        || name_lower.contains("radeon pro")
        || name_lower.contains("firepro")
        || name_lower.contains("radeon vii")
        || name_lower.contains("instinct")
        || name_lower.contains("radeon r9")
        || name_lower.contains("radeon r7")
        || name_lower.contains("radeon r5")
        || name_lower.contains("radeon hd")
        || name_lower.contains("navi")
        || name_lower.contains("polaris")
        || name_lower.contains("ellesmere")
        || name_lower.contains("baffin")
        || name_lower.contains("tonga")
        || name_lower.contains("fiji")
        || name_lower.contains("hawaii")
        || name_lower.contains("tahiti")
        || name_lower.contains("pitcairn")
}

pub fn detect_runtime_status(slot: &str) -> Option<String> {
    let canon_slot = canonicalize_pci_slot(slot);
    let path = std::path::Path::new("/sys/bus/pci/devices")
        .join(&canon_slot)
        .join("power/runtime_status");
    let content = fs::read_to_string(path).ok()?;
    let trimmed = content.trim();
    if !trimmed.is_empty() {
        Some(trimmed.to_string())
    } else {
        None
    }
}

pub fn format_vram(bytes: u64) -> String {
    const GIB: u64 = 1024 * 1024 * 1024;
    const MIB: u64 = 1024 * 1024;
    if bytes >= GIB {
        let gib = bytes as f64 / GIB as f64;
        if (gib * 10.0).round() % 10.0 == 0.0 {
            format!("{:.0} GiB", gib)
        } else {
            format!("{:.1} GiB", gib)
        }
    } else {
        format!("{} MiB", bytes / MIB)
    }
}

pub fn detect_vram(slot: &str) -> Option<String> {
    let canon_slot = canonicalize_pci_slot(slot);

    // 1. Direct path under PCI device: /sys/bus/pci/devices/<slot>/mem_info_vram_total
    let direct_pci = std::path::Path::new("/sys/bus/pci/devices")
        .join(&canon_slot)
        .join("mem_info_vram_total");
    if let Ok(bytes_str) = fs::read_to_string(&direct_pci) {
        if let Ok(bytes) = bytes_str.trim().parse::<u64>() {
            if bytes > 0 {
                return Some(format_vram(bytes));
            }
        }
    }

    // 2. Via DRM cards: /sys/class/drm/card*/device/mem_info_vram_total
    if let Ok(entries) = fs::read_dir("/sys/class/drm") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if !name_str.starts_with("card") || name_str.contains('-') {
                continue;
            }
            let dev_link = entry.path().join("device");
            if let Ok(canonical) = fs::canonicalize(&dev_link) {
                let matches_slot = canonical
                    .file_name()
                    .map(|n| {
                        let n_str = n.to_string_lossy();
                        n_str.eq_ignore_ascii_case(&canon_slot)
                            || n_str.eq_ignore_ascii_case(slot)
                            || n_str.ends_with(&format!(":{slot}"))
                            || slot.ends_with(&format!(":{n_str}"))
                    })
                    .unwrap_or(false);
                if matches_slot {
                    let vram_path = dev_link.join("mem_info_vram_total");
                    if let Ok(bytes_str) = fs::read_to_string(&vram_path) {
                        if let Ok(bytes) = bytes_str.trim().parse::<u64>() {
                            if bytes > 0 {
                                return Some(format_vram(bytes));
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

pub fn parse_static_gpu_cache(content: &str) -> Option<Vec<PciGpuDevice>> {
    let mut devices = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 3 {
            devices.push(PciGpuDevice {
                slot: parts[0].to_string(),
                gpu_type: parts[1].to_string(),
                name: parts[2..].join("\t"),
            });
        } else {
            return None;
        }
    }
    if devices.is_empty() {
        None
    } else {
        Some(devices)
    }
}

pub fn serialize_static_gpu_cache(devices: &[PciGpuDevice]) -> String {
    devices
        .iter()
        .map(|d| format!("{}\t{}\t{}", d.slot, d.gpu_type, d.name))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn scan_pci_gpus() -> Vec<PciGpuDevice> {
    let mut raw_pci_devices = Vec::new();
    if let Ok(dir) = fs::read_dir("/sys/bus/pci/devices") {
        for entry in dir.flatten() {
            let path = entry.path();
            let class = fs::read_to_string(path.join("class")).unwrap_or_default();
            let class = class.trim();
            if class.starts_with("0x0300")
                || class.starts_with("0x0380")
                || class.starts_with("0x0302")
            {
                let vendor = fs::read_to_string(path.join("vendor")).unwrap_or_default();
                let device = fs::read_to_string(path.join("device")).unwrap_or_default();
                let v = vendor.trim().trim_start_matches("0x");
                let d = device.trim().trim_start_matches("0x");
                let slot = entry.file_name().to_string_lossy().to_string();
                if !v.is_empty() && !d.is_empty() {
                    raw_pci_devices.push((slot, class.to_string(), v.to_string(), d.to_string()));
                }
            }
        }
    }

    raw_pci_devices.sort_by(|a, b| a.0.cmp(&b.0));

    let mut devices = Vec::new();
    if !raw_pci_devices.is_empty() {
        let pci_content = [
            "/usr/share/hwdata/pci.ids",
            "/usr/share/misc/pci.ids",
            "/usr/local/share/hwdata/pci.ids",
        ]
        .iter()
        .find_map(|p| fs::read_to_string(p).ok())
        .unwrap_or_default();

        for (slot, class, vendor, device) in raw_pci_devices {
            let name_opt = lookup_pci_in_content(&pci_content, &vendor, &device)
                .or_else(|| lookup_pci_slot_lspci(&slot));
            if let Some(name) = name_opt {
                let is_disc = is_discrete_gpu_detailed(&slot, &class, &vendor, &name);
                let gpu_type = if is_disc { "Discrete" } else { "Integrated" };
                devices.push(PciGpuDevice {
                    slot,
                    gpu_type: gpu_type.to_string(),
                    name,
                });
            }
        }
    }

    if devices.is_empty() {
        let output = std::process::Command::new("lspci")
            .arg("-D")
            .output()
            .or_else(|_| std::process::Command::new("lspci").output());
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if let Some((prefix, rest)) = line.split_once(": ") {
                    let mut prefix_parts = prefix.split_whitespace();
                    let raw_slot = prefix_parts.next().unwrap_or_default();
                    let class_part = prefix_parts.collect::<Vec<_>>().join(" ");
                    if !class_part.contains("VGA compatible")
                        && !class_part.contains("3D controller")
                        && !class_part.contains("Display controller")
                    {
                        continue;
                    }

                    let slot = canonicalize_pci_slot(raw_slot);
                    let clean_name = clean_lspci_gpu_name(rest);
                    let is_disc = is_discrete_gpu_detailed(&slot, &class_part, "", &clean_name);
                    let gpu_type = if is_disc { "Discrete" } else { "Integrated" };
                    devices.push(PciGpuDevice {
                        slot,
                        gpu_type: gpu_type.to_string(),
                        name: clean_name,
                    });
                }
            }
        }
    }

    devices
}

fn lookup_pci_slot_lspci(slot: &str) -> Option<String> {
    let output = std::process::Command::new("lspci")
        .args(["-s", slot])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.lines().next()?;
    let (_, rest) = line.split_once(": ")?;
    Some(clean_lspci_gpu_name(rest))
}

pub fn clean_lspci_gpu_name(raw: &str) -> String {
    let mut s = raw.trim();
    if let Some(rev_pos) = s.rfind("(rev ") {
        s = s[..rev_pos].trim();
    }

    let vendor = if s.contains("NVIDIA") {
        "NVIDIA"
    } else if s.contains("Intel") {
        "Intel"
    } else if s.contains("AMD") || s.contains("Advanced Micro Devices") {
        "AMD"
    } else {
        ""
    };

    if let (Some(start), Some(end)) = (s.rfind('['), s.rfind(']')) {
        if start < end {
            let inside = s[start + 1..end].trim();
            if !vendor.is_empty() && !inside.starts_with(vendor) {
                return format!("{vendor} {inside}");
            } else {
                return inside.to_string();
            }
        }
    }

    let cleaned = s
        .replace(" Corporation", "")
        .replace(" Advanced Micro Devices, Inc. [AMD/ATI]", "AMD")
        .replace(" Advanced Micro Devices, Inc. [AMD]", "AMD");
    cleaned.trim().to_string()
}

fn lookup_pci_in_content(content: &str, vendor_id: &str, device_id: &str) -> Option<String> {
    if content.is_empty() {
        return None;
    }
    let mut current_vendor = String::new();
    let mut in_vendor = false;

    for line in content.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        if !line.starts_with('\t') {
            if line.len() >= 4
                && line.is_char_boundary(4)
                && line[..4].eq_ignore_ascii_case(vendor_id)
            {
                in_vendor = true;
                current_vendor = line[4..]
                    .replace(" Corporation", "")
                    .replace(" Advanced Micro Devices, Inc. [AMD/ATI]", "AMD")
                    .replace(" Advanced Micro Devices, Inc. [AMD]", "AMD")
                    .trim()
                    .to_string();
            } else {
                in_vendor = false;
            }
        } else if in_vendor && !line.starts_with("\t\t") {
            let line = &line[1..];
            if line.len() >= 4
                && line.is_char_boundary(4)
                && line[..4].eq_ignore_ascii_case(device_id)
            {
                let dev_name = line[4..].trim();
                let dev_clean = if let (Some(s), Some(e)) = (dev_name.find('['), dev_name.find(']'))
                {
                    if s < e {
                        &dev_name[s + 1..e]
                    } else {
                        dev_name
                    }
                } else {
                    dev_name
                };
                return Some(format!("{current_vendor} {dev_clean}"));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_module_names() {
        assert_eq!(GpuModule::default().name(), "gpu");
        assert_eq!(DgpuModule.name(), "dgpu");
        assert_eq!(IgpuModule.name(), "igpu");
    }

    #[test]
    fn test_gpu_filter_parse() {
        assert_eq!(GpuFilter::parse("dgpu"), GpuFilter::DgpuPriority);
        assert_eq!(GpuFilter::parse("auto"), GpuFilter::DgpuPriority);
        assert_eq!(GpuFilter::parse(""), GpuFilter::DgpuPriority);
        assert_eq!(GpuFilter::parse("igpu"), GpuFilter::IgpuOnly);
        assert_eq!(GpuFilter::parse("integrated"), GpuFilter::IgpuOnly);
        assert_eq!(GpuFilter::parse("all"), GpuFilter::All);
        assert_eq!(GpuFilter::parse("both"), GpuFilter::All);
        assert_eq!(GpuFilter::parse("discrete"), GpuFilter::DiscreteOnly);
        assert_eq!(GpuFilter::parse("dgpu_only"), GpuFilter::DiscreteOnly);
    }

    #[test]
    fn test_filter_gpus_both_dgpu_and_igpu() {
        let igpu = PciGpuDevice {
            slot: "0000:00:02.0".to_string(),
            gpu_type: "Integrated".to_string(),
            name: "Intel UHD Graphics".to_string(),
        };
        let dgpu = PciGpuDevice {
            slot: "0000:01:00.0".to_string(),
            gpu_type: "Discrete".to_string(),
            name: "NVIDIA GeForce RTX 3050 Mobile".to_string(),
        };
        let both = vec![igpu.clone(), dgpu.clone()];

        // Priority dgpu (default) selects dgpu
        let filtered_default = filter_gpus(&both, GpuFilter::DgpuPriority);
        assert_eq!(filtered_default, vec![dgpu.clone()]);

        // IgpuOnly selects igpu
        let filtered_igpu = filter_gpus(&both, GpuFilter::IgpuOnly);
        assert_eq!(filtered_igpu, vec![igpu.clone()]);

        // All selects both
        let filtered_all = filter_gpus(&both, GpuFilter::All);
        assert_eq!(filtered_all, both);

        // DiscreteOnly selects dgpu
        let filtered_discrete = filter_gpus(&both, GpuFilter::DiscreteOnly);
        assert_eq!(filtered_discrete, vec![dgpu]);
    }

    #[test]
    fn test_filter_gpus_only_igpu_fallback() {
        let igpu = PciGpuDevice {
            slot: "0000:00:02.0".to_string(),
            gpu_type: "Integrated".to_string(),
            name: "Intel UHD Graphics".to_string(),
        };
        let only_igpu = vec![igpu.clone()];

        // Priority dgpu falls back to igpu when no discrete GPU exists
        let filtered_default = filter_gpus(&only_igpu, GpuFilter::DgpuPriority);
        assert_eq!(filtered_default, vec![igpu.clone()]);

        // IgpuOnly selects igpu
        let filtered_igpu = filter_gpus(&only_igpu, GpuFilter::IgpuOnly);
        assert_eq!(filtered_igpu, vec![igpu]);

        // DiscreteOnly returns empty
        let filtered_discrete = filter_gpus(&only_igpu, GpuFilter::DiscreteOnly);
        assert!(filtered_discrete.is_empty());
    }

    #[test]
    fn test_filter_gpus_only_dgpu() {
        let dgpu = PciGpuDevice {
            slot: "0000:01:00.0".to_string(),
            gpu_type: "Discrete".to_string(),
            name: "NVIDIA GeForce RTX 4090".to_string(),
        };
        let only_dgpu = vec![dgpu.clone()];

        let filtered_default = filter_gpus(&only_dgpu, GpuFilter::DgpuPriority);
        assert_eq!(filtered_default, vec![dgpu.clone()]);

        let filtered_discrete = filter_gpus(&only_dgpu, GpuFilter::DiscreteOnly);
        assert_eq!(filtered_discrete, vec![dgpu]);

        let filtered_igpu = filter_gpus(&only_dgpu, GpuFilter::IgpuOnly);
        assert!(filtered_igpu.is_empty());
    }

    #[test]
    fn test_lookup_pci_in_content() {
        let sample = "\
10de  NVIDIA Corporation
\t1f91  TU117M [GeForce GTX 1650 Mobile / Max-Q]
8086  Intel Corporation
\t9a60  TigerLake-H GT1 [UHD Graphics]
";
        let gpu1 = lookup_pci_in_content(sample, "10de", "1f91");
        assert_eq!(
            gpu1,
            Some("NVIDIA GeForce GTX 1650 Mobile / Max-Q".to_string())
        );

        let gpu2 = lookup_pci_in_content(sample, "8086", "9a60");
        assert_eq!(gpu2, Some("Intel UHD Graphics".to_string()));

        let gpu3 = lookup_pci_in_content(sample, "ffff", "0000");
        assert_eq!(gpu3, None);
    }

    #[test]
    fn test_is_discrete_gpu() {
        assert!(is_discrete_gpu("0x030200", "8086"));
        assert!(is_discrete_gpu("0x030000", "10de"));
        assert!(is_discrete_gpu("0x030000", "0x10de"));
        assert!(!is_discrete_gpu("0x030000", "8086"));
    }

    #[test]
    fn test_format_vram() {
        assert_eq!(format_vram(4 * 1024 * 1024 * 1024), "4 GiB");
        assert_eq!(format_vram(8 * 1024 * 1024 * 1024), "8 GiB");
        assert_eq!(format_vram(512 * 1024 * 1024), "512 MiB");
        assert_eq!(
            format_vram((5.5 * 1024.0 * 1024.0 * 1024.0) as u64),
            "5.5 GiB"
        );
    }

    #[test]
    fn test_static_gpu_cache_serialization_and_parsing() {
        let devices = vec![
            PciGpuDevice {
                slot: "0000:00:02.0".to_string(),
                gpu_type: "Integrated".to_string(),
                name: "Intel UHD Graphics".to_string(),
            },
            PciGpuDevice {
                slot: "0000:01:00.0".to_string(),
                gpu_type: "Discrete".to_string(),
                name: "NVIDIA GeForce RTX 3050 Mobile".to_string(),
            },
        ];

        let serialized = serialize_static_gpu_cache(&devices);
        let parsed = parse_static_gpu_cache(&serialized).expect("must parse");
        assert_eq!(parsed, devices);

        // Invalidate old format (no tabs)
        let old_format = "Intel UHD Graphics, NVIDIA GeForce RTX 3050 Mobile";
        assert_eq!(parse_static_gpu_cache(old_format), None);
    }

    #[test]
    fn test_canonicalize_pci_slot() {
        assert_eq!(canonicalize_pci_slot("01:00.0"), "0000:01:00.0");
        assert_eq!(canonicalize_pci_slot("0000:01:00.0"), "0000:01:00.0");
        assert_eq!(canonicalize_pci_slot(""), "");
    }

    #[test]
    fn test_clean_lspci_gpu_name() {
        assert_eq!(
            clean_lspci_gpu_name("Intel Corporation TigerLake-H GT1 [UHD Graphics] (rev 01)"),
            "Intel UHD Graphics"
        );
        assert_eq!(
            clean_lspci_gpu_name("NVIDIA Corporation GA107M [GeForce RTX 3050 Mobile] (rev a1)"),
            "NVIDIA GeForce RTX 3050 Mobile"
        );
        assert_eq!(
            clean_lspci_gpu_name(
                "Advanced Micro Devices, Inc. [AMD/ATI] Navi 23 [Radeon RX 6600/6600 XT/6600M]"
            ),
            "AMD Radeon RX 6600/6600 XT/6600M"
        );
        assert_eq!(
            clean_lspci_gpu_name("Red Hat, Inc. Virtio GPU (rev 01)"),
            "Red Hat, Inc. Virtio GPU"
        );
    }

    #[test]
    fn test_is_discrete_gpu_detailed() {
        // AMD Discrete
        assert!(is_discrete_gpu_detailed(
            "0000:03:00.0",
            "0x030000",
            "1002",
            "AMD Radeon RX 6600"
        ));
        assert!(is_discrete_gpu_detailed(
            "0000:03:00.0",
            "0x030000",
            "1002",
            "AMD Radeon RX 7900 XTX"
        ));
        assert!(is_discrete_gpu_detailed(
            "0000:03:00.0",
            "0x030000",
            "1002",
            "AMD Radeon Pro W6800"
        ));

        // AMD APU / Integrated
        assert!(!is_discrete_gpu_detailed(
            "0000:00:02.0",
            "0x030000",
            "1002",
            "AMD Radeon 780M"
        ));
        assert!(!is_discrete_gpu_detailed(
            "0000:00:02.0",
            "0x030000",
            "1002",
            "AMD Radeon Graphics (Cezanne)"
        ));
        assert!(!is_discrete_gpu_detailed(
            "0000:00:02.0",
            "0x030000",
            "1002",
            "AMD Radeon Vega 8 Graphics"
        ));

        // Intel Arc / Iris Xe MAX Discrete
        assert!(is_discrete_gpu_detailed(
            "0000:03:00.0",
            "0x030000",
            "8086",
            "Intel Arc A770 Graphics"
        ));
        assert!(is_discrete_gpu_detailed(
            "0000:03:00.0",
            "0x030000",
            "8086",
            "Intel Arc B580 Graphics"
        ));
        assert!(is_discrete_gpu_detailed(
            "0000:03:00.0",
            "0x030000",
            "8086",
            "Intel Iris Xe MAX Graphics"
        ));

        // Intel Integrated
        assert!(!is_discrete_gpu_detailed(
            "0000:00:02.0",
            "0x030000",
            "8086",
            "Intel UHD Graphics 770"
        ));
        assert!(!is_discrete_gpu_detailed(
            "0000:00:02.0",
            "0x030000",
            "8086",
            "Intel Iris Xe Graphics"
        ));

        // NVIDIA Discrete
        assert!(is_discrete_gpu_detailed(
            "0000:01:00.0",
            "0x030000",
            "10de",
            "NVIDIA GeForce RTX 4090"
        ));

        // 3D Controller is always discrete
        assert!(is_discrete_gpu_detailed(
            "0000:01:00.0",
            "0x030200",
            "8086",
            "Unknown"
        ));
    }

    #[test]
    fn test_filter_parser_synonyms() {
        assert_eq!(GpuFilter::parse("dedicated"), GpuFilter::DiscreteOnly);
        assert_eq!(GpuFilter::parse("dedicated_only"), GpuFilter::DiscreteOnly);
        assert_eq!(GpuFilter::parse("internal"), GpuFilter::IgpuOnly);
        assert_eq!(GpuFilter::parse("external"), GpuFilter::DiscreteOnly);
        assert_eq!(GpuFilter::parse("egpu"), GpuFilter::DiscreteOnly);
    }
}
