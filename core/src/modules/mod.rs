pub mod battery;
pub mod bluetooth;
pub mod board;
pub mod brightness;
pub mod container;
pub mod cpu;
pub mod cpu_usage;
pub mod custom;
pub mod desktop;
pub mod disk;
pub mod display;
pub mod font;
pub mod gpu;
pub mod host;
pub mod init;
pub mod kernel;
pub mod local_ip;
pub mod locale;
pub mod media;
pub mod memory;
pub mod os;
pub mod packages;
pub mod processes;
pub mod shell;
pub mod sound;
pub mod swap;
pub mod temp;
pub mod terminal;
pub mod uptime;
pub mod wifi;

use crate::Module;

pub fn all_modules() -> Vec<Box<dyn Module>> {
    [
        "os",
        "host",
        "board",
        "kernel",
        "uptime",
        "packages",
        "shell",
        "desktop",
        "font",
        "terminal",
        "cpu",
        "temp",
        "gpu",
        "memory",
        "swap",
        "disk",
        "battery",
        "brightness",
        "display",
        "sound",
        "wifi",
        "local_ip",
        "locale",
        "processes",
        "init",
        "container",
    ]
    .into_iter()
    .filter_map(module_by_name)
    .collect()
}

pub fn module_by_name(name: &str) -> Option<Box<dyn Module>> {
    match name {
        "os" => Some(Box::new(os::Os)),
        "host" => Some(Box::new(host::Host)),
        "board" => Some(Box::new(board::Board)),
        "kernel" => Some(Box::new(kernel::Kernel)),
        "uptime" => Some(Box::new(uptime::Uptime)),
        "packages" => Some(Box::new(packages::PackagesModule)),
        "shell" => Some(Box::new(shell::Shell)),
        "desktop" => Some(Box::new(desktop::DesktopModule)),
        "font" => Some(Box::new(font::FontModule)),
        "terminal" => Some(Box::new(terminal::Terminal)),
        "cpu" => Some(Box::new(cpu::Cpu)),
        "cpu_usage" => Some(Box::new(cpu_usage::CpuUsage)),
        "temp" => Some(Box::new(temp::Temp)),
        "gpu" => Some(Box::new(gpu::GpuModule)),
        "memory" => Some(Box::new(memory::Memory)),
        "swap" => Some(Box::new(swap::Swap)),
        "disk" => Some(Box::new(disk::Disk)),
        "battery" => Some(Box::new(battery::BatteryModule)),
        "brightness" => Some(Box::new(brightness::BrightnessModule)),
        "display" => Some(Box::new(display::Display)),
        "sound" => Some(Box::new(sound::Sound)),
        "media" => Some(Box::new(media::Media)),
        "bluetooth" => Some(Box::new(bluetooth::BluetoothModule)),
        "wifi" => Some(Box::new(wifi::Wifi)),
        "local_ip" => Some(Box::new(local_ip::LocalIpModule)),
        "locale" => Some(Box::new(locale::Locale)),
        "processes" => Some(Box::new(processes::Processes)),
        "init" | "initsystem" => Some(Box::new(init::Init)),
        "container" | "vm" => Some(Box::new(container::Container)),
        "custom" => Some(Box::new(custom::Custom)),
        "break" | "divider" => Some(Box::new(BreakLine)),
        _ => None,
    }
}

pub struct BreakLine;

impl Module for BreakLine {
    fn name(&self) -> &'static str {
        "break"
    }

    fn detect(&self) -> Option<crate::Info> {
        Some(crate::Info::new("", ""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_modules_does_not_contain_media() {
        let all = all_modules();
        let names: Vec<&str> = all.iter().map(|m| m.name()).collect();
        assert!(names.contains(&"temp"));
        assert!(names.contains(&"sound"));
        assert!(names.contains(&"brightness"));
        assert!(names.contains(&"init"));
        assert!(names.contains(&"container"));
        assert!(!names.contains(&"media"));
    }

    #[test]
    fn test_module_by_name_resolution() {
        assert!(module_by_name("temp").is_some());
        assert!(module_by_name("sound").is_some());
        assert!(module_by_name("media").is_some());
        assert!(module_by_name("brightness").is_some());
        assert!(module_by_name("cpu_usage").is_some());
        assert!(module_by_name("bluetooth").is_some());
        assert!(module_by_name("init").is_some());
        assert!(module_by_name("container").is_some());
        assert!(module_by_name("custom").is_some());
        assert!(module_by_name("break").is_some());
        assert!(module_by_name("nonexistent").is_none());
    }
}
