//! ASCII distro logos for terminal display.

/// ANSI reset code.
pub const RESET: &str = "\x1b[0m";

/// A terminal-printable ASCII logo.
pub struct Logo {
    /// Raw ASCII lines (no color codes).
    pub lines: Vec<&'static str>,
    /// ANSI color escape code for this logo.
    pub color: &'static str,
    /// Visual width of the widest line.
    pub width: usize,
}

impl Logo {
    /// Number of lines in the logo.
    #[allow(dead_code)]
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    /// Return the logo lines pre-colored with ANSI codes.
    pub fn colored_lines(&self) -> Vec<String> {
        self.lines
            .iter()
            .map(|line| format!("{}{}{}", self.color, line, RESET))
            .collect()
    }
}

/// Get the logo for a distro name (case-insensitive, partial match).
/// Returns a generic Linux/Tux logo for unknown distros.
pub fn get_logo(distro: &str) -> Logo {
    let lower = distro.to_lowercase();
    if lower.contains("fedora") {
        fedora()
    } else if lower.contains("arch") {
        arch()
    } else if lower.contains("debian") {
        debian()
    } else if lower.contains("ubuntu") {
        ubuntu()
    } else if lower.contains("mint") {
        mint()
    } else if lower.contains("manjaro") {
        manjaro()
    } else if lower.contains("pop") {
        pop()
    } else if lower.contains("alpine") {
        alpine()
    } else if lower.contains("void") {
        void()
    } else if lower.contains("gentoo") {
        gentoo()
    } else if lower.contains("endeavour") {
        endeavour()
    } else if lower.contains("kali") {
        kali()
    } else if lower.contains("steam") {
        steamos()
    } else if lower.contains("artix") {
        artix()
    } else if lower.contains("red hat")
        || lower.contains("redhat")
        || lower.contains("rhel")
        || lower.contains("centos")
        || lower.contains("rocky")
        || lower.contains("alma")
    {
        redhat()
    } else if lower.contains("opensuse") || lower.contains("suse") {
        opensuse()
    } else if lower.contains("nixos") || lower.contains("nix") {
        nixos()
    } else {
        linux()
    }
}

fn fedora() -> Logo {
    Logo {
        lines: vec![
            r"        ________        ",
            r"       /    __  \       ",
            r"      |    /  \  |      ",
            r"      |   |    __|      ",
            r"      |   |   |_       ",
            r"      |   |    _|       ",
            r"      |    \__/ |       ",
            r"       \________/       ",
        ],
        color: "\x1b[34m", // blue
        width: 24,
    }
}

fn arch() -> Logo {
    Logo {
        lines: vec![
            r"        /\          ",
            r"       /  \         ",
            r"      /\   \        ",
            r"     /  \   \       ",
            r"    /   _\   \      ",
            r"   /   |  \   \     ",
            r"  /    |___\   \    ",
            r" /_____\    \___\   ",
        ],
        color: "\x1b[36m", // cyan
        width: 20,
    }
}

fn debian() -> Logo {
    Logo {
        lines: vec![
            r"     _______       ",
            r"    / _____ \      ",
            r"   / |     | \     ",
            r"  |  |     |  |    ",
            r"  |  |_____|  |    ",
            r"   \          /    ",
            r"    \  ____  /     ",
            r"     \_____/       ",
        ],
        color: "\x1b[31m", // red
        width: 19,
    }
}

fn ubuntu() -> Logo {
    Logo {
        lines: vec![
            r"          _       ",
            r"      ---(_)      ",
            r"  _/  ---  \      ",
            r" (_) |   |        ",
            r"  \  --- _/       ",
            r"      ---(_)      ",
            r"                  ",
        ],
        color: "\x1b[33m", // yellow/orange
        width: 18,
    }
}

fn opensuse() -> Logo {
    Logo {
        lines: vec![
            r"    _______       ",
            r"   |   __  \      ",
            r"   |  /  \  \     ",
            r"   |  \__/  |     ",
            r"   |   ___ /      ",
            r"   |  |           ",
            r"   |__|           ",
            r"                  ",
        ],
        color: "\x1b[32m", // green
        width: 18,
    }
}

fn nixos() -> Logo {
    Logo {
        lines: vec![
            r"    \\  \\ //     ",
            r"   ==\\__\\/ //   ",
            r"     //   \\//    ",
            r"  ==//     //==   ",
            r"   //\\___//      ",
            r"  // /\\  \\==    ",
            r"    // \\  \\     ",
        ],
        color: "\x1b[36m", // cyan
        width: 18,
    }
}

fn linux() -> Logo {
    Logo {
        lines: vec![
            r"      ___         ",
            r"     (.. |        ",
            r"     (<> |        ",
            r"    / __  \       ",
            r"   ( /  \ /|      ",
            r"  _/\ __)/_)      ",
            r"  \/-____\/       ",
            r"                  ",
        ],
        color: "\x1b[37m", // white
        width: 18,
    }
}

fn mint() -> Logo {
    Logo {
        lines: vec![
            r"   ____________    ",
            r"  |_          _ \   ",
            r"    | | _____| | |  ",
            r"    | | | | | | | | ",
            r"    | | | | | | | | ",
            r"    | \_____/  / |  ",
            r"    \_________/ /   ",
            r"     \_________/    ",
        ],
        color: "\x1b[32m", // green
        width: 20,
    }
}

fn manjaro() -> Logo {
    Logo {
        lines: vec![
            r"  ||||||||| ||||  ",
            r"  ||||||||| ||||  ",
            r"  ||||      ||||  ",
            r"  |||| |||| ||||  ",
            r"  |||| |||| ||||  ",
            r"  |||| |||| ||||  ",
            r"  |||| |||| ||||  ",
            r"  |||| |||| ||||  ",
        ],
        color: "\x1b[32m", // green
        width: 18,
    }
}

fn pop() -> Logo {
    Logo {
        lines: vec![
            r"   ______         ",
            r"  / ____/____  __ ",
            r" / /_   / __ \/ / ",
            r"/ __/  / /_/ /_/  ",
            r"/_/    \____(_)   ",
            r"                  ",
            r"                  ",
            r"                  ",
        ],
        color: "\x1b[36m", // cyan
        width: 18,
    }
}

fn alpine() -> Logo {
    Logo {
        lines: vec![
            r"      /\          ",
            r"     /  \  /\     ",
            r"    /    \/  \    ",
            r"   /   /\     \   ",
            r"  /   /  \     \  ",
            r" /___/    \_____\ ",
            r"                  ",
            r"                  ",
        ],
        color: "\x1b[34m", // blue
        width: 18,
    }
}

fn void() -> Logo {
    Logo {
        lines: vec![
            r"      _______     ",
            r"     /\ ____ \    ",
            r"    / / \   \ \   ",
            r"   | |   \   | |  ",
            r"   | |    \  | |  ",
            r"    \ \____\ \/   ",
            r"     \/______/    ",
            r"                  ",
        ],
        color: "\x1b[32m", // green
        width: 18,
    }
}

fn gentoo() -> Logo {
    Logo {
        lines: vec![
            r"   .-----.        ",
            r"  / .---. \       ",
            r" / /     \ \      ",
            r"| |  /--. | |     ",
            r"| |  |  | | |     ",
            r" \ \  \_/ / /     ",
            r"  \ `---' /       ",
            r"   `-----'        ",
        ],
        color: "\x1b[35m", // magenta
        width: 18,
    }
}

fn endeavour() -> Logo {
    Logo {
        lines: vec![
            r"        / \       ",
            r"       /   \      ",
            r"      /  /\ \     ",
            r"     /  /  \ \    ",
            r"    /  /    \ \   ",
            r"   /  /______\ \  ",
            r"  /_____________\ ",
            r"                  ",
        ],
        color: "\x1b[35m", // magenta
        width: 18,
    }
}

fn kali() -> Logo {
    Logo {
        lines: vec![
            r"   .---.          ",
            r"  /     \  /\     ",
            r" |   ()  \/  \    ",
            r" |       /    \   ",
            r"  \     /      \  ",
            r"   `---'        \ ",
            r"                 `",
            r"                  ",
        ],
        color: "\x1b[34m", // blue
        width: 18,
    }
}

fn steamos() -> Logo {
    Logo {
        lines: vec![
            r"      .----.      ",
            r"     /  ()  \     ",
            r"  --|        |--  ",
            r"     \  ()  /     ",
            r"      `----'      ",
            r"                  ",
            r"                  ",
            r"                  ",
        ],
        color: "\x1b[36m", // cyan
        width: 18,
    }
}

fn artix() -> Logo {
    Logo {
        lines: vec![
            r"        /\        ",
            r"       /  \       ",
            r"      /`'.,\      ",
            r"     /     ',     ",
            r"    /      ,`\    ",
            r"   /   ,.'`.  \   ",
            r"  /.,'`     `'.\  ",
            r"                  ",
        ],
        color: "\x1b[36m", // cyan
        width: 18,
    }
}

fn redhat() -> Logo {
    Logo {
        lines: vec![
            r"    .---.         ",
            r"   /_____\        ",
            r"  (  o o  )       ",
            r"   `--^--'        ",
            r"  /       \       ",
            r" (  ====   )      ",
            r"  `-------'       ",
            r"                  ",
        ],
        color: "\x1b[31m", // red
        width: 18,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_logo_fedora() {
        let logo = get_logo("fedora");
        assert_eq!(logo.color, "\x1b[34m");
        assert!(logo.height() >= 6);
    }

    #[test]
    fn test_get_logo_case_insensitive() {
        let logo1 = get_logo("Fedora");
        let logo2 = get_logo("FEDORA");
        let logo3 = get_logo("Fedora Linux 44");
        assert_eq!(logo1.color, logo2.color);
        assert_eq!(logo2.color, logo3.color);
    }

    #[test]
    fn test_get_logo_arch() {
        let logo = get_logo("arch");
        assert_eq!(logo.color, "\x1b[36m");
    }

    #[test]
    fn test_get_logo_unknown_returns_linux() {
        let logo = get_logo("totally_unknown_distro");
        assert_eq!(logo.color, "\x1b[37m"); // white = generic linux
    }

    #[test]
    fn test_colored_lines() {
        let logo = get_logo("fedora");
        let lines = logo.colored_lines();
        assert_eq!(lines.len(), logo.height());
        assert!(lines[0].starts_with("\x1b[34m"));
        assert!(lines[0].ends_with(RESET));
    }

    #[test]
    fn test_all_expanded_distros_return_correct_logos() {
        assert_eq!(get_logo("Linux Mint").color, "\x1b[32m");
        assert_eq!(get_logo("Manjaro Linux").color, "\x1b[32m");
        assert_eq!(get_logo("Pop!_OS").color, "\x1b[36m");
        assert_eq!(get_logo("Alpine Linux").color, "\x1b[34m");
        assert_eq!(get_logo("Void Linux").color, "\x1b[32m");
        assert_eq!(get_logo("Gentoo").color, "\x1b[35m");
        assert_eq!(get_logo("EndeavourOS").color, "\x1b[35m");
        assert_eq!(get_logo("Kali GNU/Linux").color, "\x1b[34m");
        assert_eq!(get_logo("SteamOS").color, "\x1b[36m");
        assert_eq!(get_logo("Artix Linux").color, "\x1b[36m");
        assert_eq!(get_logo("Red Hat Enterprise Linux").color, "\x1b[31m");
        assert_eq!(get_logo("Rocky Linux").color, "\x1b[31m");
        assert_eq!(get_logo("CentOS Stream").color, "\x1b[31m");
        assert_eq!(get_logo("AlmaLinux").color, "\x1b[31m");
    }
}
