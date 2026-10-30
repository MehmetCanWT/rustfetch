//! ASCII distro logos for terminal display.

/// ANSI reset code.
pub const RESET: &str = "\x1b[0m";

// Authentic 24-bit TrueColor RGB escape codes that completely bypass Kitty themes and terminal palettes
pub const FEDORA_BLUE: &str = "\x1b[38;2;60;110;180m"; // #3c6eb4
pub const ARCH_CYAN: &str = "\x1b[38;2;23;147;209m"; // #1793d1
pub const ARCH_LIGHT: &str = "\x1b[38;2;81;190;240m"; // #51bef0
pub const GENTOO_PURPLE: &str = "\x1b[38;2;186;150;226m"; // #ba96e2
pub const UBUNTU_ORANGE: &str = "\x1b[38;2;233;84;32m"; // #e95420
pub const DEBIAN_RED: &str = "\x1b[38;2;215;10;83m"; // #d70a53
pub const MINT_GREEN: &str = "\x1b[38;2;135;207;128m"; // #87cf80
pub const MANJARO_GREEN: &str = "\x1b[38;2;53;191;92m"; // #35bf5c
pub const SUSE_GREEN: &str = "\x1b[38;2;115;186;37m"; // #73ba25
pub const NIXOS_BLUE: &str = "\x1b[38;2;82;119;195m"; // #5277c3
pub const NIXOS_CYAN: &str = "\x1b[38;2;126;186;228m"; // #7ebae4
pub const VOID_GREEN: &str = "\x1b[38;2;71;128;97m"; // #478061
pub const POP_TEAL: &str = "\x1b[38;2;72;185;199m"; // #48b9c7
pub const POP_ORANGE: &str = "\x1b[38;2;255;160;0m"; // #ffa000
pub const ALPINE_BLUE: &str = "\x1b[38;2;13;89;127m"; // #0d597f
pub const KALI_BLUE: &str = "\x1b[38;2;85;124;148m"; // #557c94
pub const STEAM_BLUE: &str = "\x1b[38;2;26;159;255m"; // #1a9fff
pub const ARTIX_CYAN: &str = "\x1b[38;2;23;147;209m";
pub const ZORIN_BLUE: &str = "\x1b[38;2;0;163;255m";
pub const RASPBIAN_RED: &str = "\x1b[38;2;197;26;74m";
pub const RASPBIAN_GREEN: &str = "\x1b[38;2;117;169;40m";
pub const REDHAT_RED: &str = "\x1b[38;2;238;0;0m";
pub const TUX_YELLOW: &str = "\x1b[38;2;255;193;7m";
pub const TUX_DARK: &str = "\x1b[38;2;45;45;45m";
pub const WHITE: &str = "\x1b[38;2;255;255;255m";
pub const CACHYOS_CYAN: &str = "\x1b[38;2;0;180;216m";
pub const CACHYOS_GREEN: &str = "\x1b[38;2;0;204;153m";
pub const CACHYOS_DARK: &str = "\x1b[38;2;40;40;40m";
pub const FREEBSD_RED: &str = "\x1b[38;2;218;31;40m";
pub const FREEBSD_WHITE: &str = "\x1b[38;2;255;255;255m";
pub const OPENBSD_YELLOW: &str = "\x1b[38;2;242;194;0m";
pub const OPENBSD_BLUE: &str = "\x1b[38;2;0;122;204m";
pub const NETBSD_ORANGE: &str = "\x1b[38;2;255;102;0m";
pub const ENDEAVOUR_PURPLE: &str = "\x1b[38;2;127;63;191m";
pub const ENDEAVOUR_RED: &str = "\x1b[38;2;255;75;100m";
pub const ENDEAVOUR_BLUE: &str = "\x1b[38;2;0;164;239m";
pub const MACOS_GREEN: &str = "\x1b[38;2;97;187;70m";
pub const MACOS_YELLOW: &str = "\x1b[38;2;244;180;0m";
pub const MACOS_ORANGE: &str = "\x1b[38;2;230;113;34m";
pub const MACOS_RED: &str = "\x1b[38;2;227;61;64m";
pub const MACOS_PURPLE: &str = "\x1b[38;2;150;61;151m";
pub const MACOS_BLUE: &str = "\x1b[38;2;0;157;220m";
pub const WINDOWS_BLUE: &str = "\x1b[38;2;0;164;239m";
pub const WINDOWS_RED: &str = "\x1b[38;2;242;80;34m";
pub const WINDOWS_GREEN: &str = "\x1b[38;2;127;186;0m";
pub const WINDOWS_YELLOW: &str = "\x1b[38;2;255;185;0m";
pub const GARUDA_ORANGE: &str = "\x1b[38;2;240;80;35m";
pub const GARUDA_YELLOW: &str = "\x1b[38;2;255;170;0m";
pub const NOBARA_WHITE: &str = "\x1b[38;2;255;255;255m";
pub const ARCHCRAFT_TEAL: &str = "\x1b[38;2;94;178;167m";
pub const BAZZITE_PURPLE: &str = "\x1b[38;2;138;79;255m"; // #8a4fff
pub const CAT_AMBER: &str = "\x1b[38;2;249;175;79m"; // #f9af4f
pub const CAT_EMERALD: &str = "\x1b[38;2;102;217;160m"; // #66d9a0
pub const CAT_CREAM: &str = "\x1b[38;2;245;224;220m"; // #f5e0dc

pub fn list_available_logos() -> &'static [(&'static str, &'static str)] {
    &[
        ("arch", "Arch Linux (also: arch_small)"),
        ("fedora", "Fedora Linux (also: fedora_small)"),
        ("bazzite", "Bazzite (Immutable Gaming OS)"),
        ("nobara", "Nobara Linux"),
        ("cachyos", "CachyOS (also: cachyos_small)"),
        ("nixos", "NixOS (also: nixos_small)"),
        ("ubuntu", "Ubuntu (also: ubuntu_small)"),
        ("debian", "Debian GNU/Linux (also: debian_small)"),
        ("mint", "Linux Mint"),
        ("manjaro", "Manjaro Linux"),
        ("endeavouros", "EndeavourOS"),
        ("garuda", "Garuda Linux"),
        ("archcraft", "Archcraft"),
        ("pop", "Pop!_OS"),
        ("gentoo", "Gentoo Linux"),
        ("alpine", "Alpine Linux"),
        ("void", "Void Linux"),
        ("opensuse", "openSUSE"),
        ("kali", "Kali Linux"),
        ("steamos", "SteamOS"),
        ("artix", "Artix Linux"),
        ("zorin", "Zorin OS"),
        ("raspbian", "Raspberry Pi OS"),
        ("rocky", "Rocky Linux"),
        ("almalinux", "AlmaLinux"),
        ("centos", "CentOS Stream"),
        ("redhat", "Red Hat Enterprise Linux"),
        ("freebsd", "FreeBSD (also: freebsd_small)"),
        ("openbsd", "OpenBSD"),
        ("netbsd", "NetBSD"),
        ("macos", "macOS / Darwin"),
        ("windows", "Windows"),
        ("cat", "RustFetch Cat Mascot (also: cat_small)"),
        ("bengal", "Bengal Cat (v0.1.2 Codename Mascot)"),
        ("maine_coon", "Maine Coon Cat (v0.1.3 Codename Mascot)"),
        ("siamese", "Siamese Cat (v0.1.4 Codename Mascot)"),
        (
            "turkish_van",
            "Turkish Van Cat — Heterochromia Eyes (v0.1.0 Mascot)",
        ),
        ("turkish_angora", "Turkish Angora Cat (v0.1.1 Mascot)"),
    ]
}

/// A terminal-printable ASCII logo.
#[derive(Debug, Clone)]
pub struct Logo {
    /// Raw ASCII lines (may contain $1..$9 color placeholders).
    pub lines: Vec<&'static str>,
    /// Primary ANSI color escape code for this logo.
    pub color: &'static str,
    /// ANSI colors corresponding to $1, $2, $3, etc.
    pub colors: Vec<&'static str>,
    /// Visual width of the widest line.
    pub width: usize,
}

impl Logo {
    /// Return the logo lines pre-colored with ANSI codes.
    /// Carries the active color across lines for multi-colored logos.
    pub fn colored_lines(&self) -> Vec<String> {
        self.colored_lines_with_custom(None, None)
    }

    /// Return the logo lines colored with optional custom override color or custom palette.
    pub fn colored_lines_with_custom(
        &self,
        override_color: Option<&str>,
        custom_palette: Option<&[&str]>,
    ) -> Vec<String> {
        let base_color = override_color.unwrap_or(self.color);
        let mut active_color = if let Some(pal) = custom_palette {
            pal.first().copied().unwrap_or(base_color)
        } else {
            override_color.unwrap_or_else(|| self.colors.first().copied().unwrap_or(self.color))
        };
        self.lines
            .iter()
            .map(|line| {
                let mut out = String::with_capacity(line.len() + 32);
                out.push_str(active_color);
                let mut chars = line.chars().peekable();
                while let Some(c) = chars.next() {
                    if c == '$' {
                        if let Some(&next) = chars.peek() {
                            if let Some(digit) = next.to_digit(10) {
                                chars.next();
                                let idx = (digit as usize).saturating_sub(1);
                                if let Some(pal) = custom_palette {
                                    active_color = pal.get(idx).copied().unwrap_or(base_color);
                                } else if let Some(ov) = override_color {
                                    active_color = ov;
                                } else {
                                    active_color =
                                        self.colors.get(idx).copied().unwrap_or(self.color);
                                }
                                out.push_str(active_color);
                                continue;
                            }
                        }
                    }
                    out.push(c);
                }
                out.push_str(RESET);
                out
            })
            .collect()
    }
}

/// Compute visual width ignoring color placeholders ($1..$9).
pub fn compute_logo_width(lines: &[&str]) -> usize {
    lines
        .iter()
        .map(|line| {
            let mut w = 0;
            let mut chars = line.chars().peekable();
            while let Some(c) = chars.next() {
                if c == '$' {
                    if let Some(&next) = chars.peek() {
                        if next.is_ascii_digit() {
                            chars.next();
                            continue;
                        }
                    }
                }
                w += unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
            }
            w
        })
        .max()
        .unwrap_or(0)
}

/// Get the logo for a distro name (case-insensitive, partial match).
/// Returns a generic Linux/Tux logo for unknown distros.
pub fn get_logo(distro: &str) -> Logo {
    let lower = distro.to_lowercase();
    // 1. Compact / small variants first
    if lower.contains("cachyos_small") || lower.contains("cachy_small") {
        cachyos_small()
    } else if lower.contains("freebsd_small") || lower.contains("bsd_small") {
        freebsd_small()
    } else if lower.contains("nixos_small") || lower.contains("nix_small") {
        nixos_small()
    } else if lower.contains("arch_small") {
        arch_small()
    } else if lower.contains("fedora_small") {
        fedora_small()
    } else if lower.contains("ubuntu_small") {
        ubuntu_small()
    } else if lower.contains("debian_small") {
        debian_small()
    } else if lower.contains("cat_small")
        || lower.contains("bengal_small")
        || lower.contains("van_small")
    {
        cat_small()
    // 2. Cat Breed Mascot Logos
    } else if lower.contains("turkish_van") || lower.contains("turkish van") || lower == "van" {
        turkish_van()
    } else if lower.contains("turkish_angora")
        || lower.contains("turkish angora")
        || lower == "angora"
    {
        turkish_angora()
    } else if lower.contains("bengal") {
        bengal_cat()
    } else if lower.contains("maine_coon") || lower.contains("maine coon") || lower == "mainecoon" {
        maine_coon_cat()
    } else if lower.contains("siamese") || lower == "siamese_cat" {
        siamese_cat()
    } else if lower == "cat"
        || lower == "kitty_cat"
        || lower == "kedi"
        || lower == "ragdoll"
        || lower == "anatolian"
    {
        cat_mascot()
    // 3. Standard distribution logos
    } else if lower.contains("cachyos") || lower.contains("cachy") {
        cachyos()
    } else if lower.contains("freebsd") {
        freebsd()
    } else if lower.contains("openbsd") {
        openbsd()
    } else if lower.contains("netbsd") {
        netbsd()
    } else if lower.contains("archcraft") {
        archcraft()
    } else if lower.contains("garuda") {
        garuda()
    } else if lower.contains("nobara") {
        nobara()
    } else if lower.contains("bazzite") {
        let mut logo = fedora();
        logo.color = BAZZITE_PURPLE;
        logo.colors = vec![BAZZITE_PURPLE, WHITE];
        logo
    } else if lower.contains("darwin") || lower.contains("macos") || lower.contains("apple") {
        macos()
    } else if lower.contains("windows") {
        windows()
    } else if lower.contains("fedora") {
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
    } else if lower.contains("zorin") {
        zorin()
    } else if lower.contains("raspbian") || lower.contains("raspberry") {
        raspbian()
    } else if lower.contains("rocky") {
        rocky()
    } else if lower.contains("alma") {
        almalinux()
    } else if lower.contains("centos") {
        centos()
    } else if lower.contains("red hat") || lower.contains("redhat") || lower.contains("rhel") {
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
    let lines = vec![
        r##"             .',;::::;,'."##,
        r##"         .';:cccccccccccc:;,."##,
        r##"      .;cccccccccccccccccccccc;."##,
        r##"    .:cccccccccccccccccccccccccc:."##,
        r##"  .;ccccccccccccc;$2.:dddl:.$1;ccccccc;."##,
        r##" .:ccccccccccccc;$2OWMKOOXMWd$1;ccccccc:."##,
        r##".:ccccccccccccc;$2KMMc$1;cc;$2xMMc$1;ccccccc:."##,
        r##",cccccccccccccc;$2MMM.$1;cc;$2;WW:$1;cccccccc,"##,
        r##":cccccccccccccc;$2MMM.$1;cccccccccccccccc:"##,
        r##":ccccccc;$2oxOOOo$1;$2MMM000k.$1;cccccccccccc:"##,
        r##"cccccc;$20MMKxdd:$1;$2MMMkddc.$1;cccccccccccc;"##,
        r##"ccccc;$2XMO'$1;cccc;$2MMM.$1;cccccccccccccccc'"##,
        r##"ccccc;$2MMo$1;ccccc;$2MMW.$1;ccccccccccccccc;"##,
        r##"ccccc;$20MNc.$1ccc$2.xMMd$1;ccccccccccccccc;"##,
        r##"cccccc;$2dNMWXXXWM0:$1;cccccccccccccc:,"##,
        r##"cccccccc;$2.:odl:.$1;cccccccccccccc:,."##,
        r##"ccccccccccccccccccccccccccccc:'."##,
        r##":ccccccccccccccccccccccc:;,.."##,
        r##" ':cccccccccccccccc::;,."##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: FEDORA_BLUE,
        colors: vec![FEDORA_BLUE, WHITE],
        width,
    }
}

fn arch() -> Logo {
    let lines = vec![
        r##"                  -`"##,
        r##"                 .o+`"##,
        r##"                `ooo/"##,
        r##"               `+oooo:"##,
        r##"              `+oooooo:"##,
        r##"              -+oooooo+:"##,
        r##"            `/:-:++oooo+:"##,
        r##"           `/++++/+++++++:"##,
        r##"          `/++++++++++++++:"##,
        r##"         `/+++o$2oooooooo$1oooo/`"##,
        r##"        ./$2ooosssso++osssssso$1+`"##,
        r##"$2       .oossssso-````/ossssss+`"##,
        r##"      -osssssso.      :ssssssso."##,
        r##"     :osssssss/        osssso+++."##,
        r##"    /ossssssss/        +ssssooo/-"##,
        r##"  `/ossssso+/:-        -:/+osssso+-"##,
        r##" `+sso+:-`                 `.-/+oso:"##,
        r##"`++:.                           `-/+/"##,
        r##".`                                 `/"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: ARCH_CYAN,
        colors: vec![ARCH_CYAN, ARCH_LIGHT],
        width,
    }
}

fn debian() -> Logo {
    let lines = vec![
        r##"        $2_,met$$$$$$$$$$gg."##,
        r##"     ,g$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$P."##,
        r##"   ,g$$$$P""       """Y$$$$."."##,
        r##"  ,$$$$P'              `$$$$$$."##,
        r##"',$$$$P       ,ggs.     `$$$$b:"##,
        r##"`d$$$$'     ,$P"'   $1.$2    $$$$$$"##,
        r##" $$$$P      d$'     $1,$2    $$$$P"##,
        r##" $$$$:      $$$.   $1-$2    ,d$$$$'"##,
        r##" $$$$;      Y$b._   _,d$P'"##,
        r##" Y$$$$.    $1`.$2`"Y$$$$$$$$P"'"##,
        r##" `$$$$b      $1"-.__"##,
        r##"  $2`Y$$$$b"##,
        r##"   `Y$$$$."##,
        r##"     `$$$$b."##,
        r##"       `Y$$$$b."##,
        r##"         `"Y$$b._"##,
        r##"             `"""""##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: DEBIAN_RED,
        colors: vec![WHITE, DEBIAN_RED],
        width,
    }
}

fn ubuntu() -> Logo {
    let lines = vec![
        r##"                             ...."##,
        r##"              $2.',:clooo:  $1.:looooo:."##,
        r##"           $2.;looooooooc  $1.oooooooooo'"##,
        r##"        $2.;looooool:,''.  $1:ooooooooooc"##,
        r##"       $2;looool;.         $1'oooooooooo,"##,
        r##"      $2;clool'             $1.cooooooc.  $2,,"##,
        r##"         $2...                $1......  $2.:oo,"##,
        r##"  $1.;clol:,.                        $2.loooo'"##,
        r##" $1:ooooooooo,                        $2'ooool"##,
        r##"$1'ooooooooooo.                        $2loooo."##,
        r##"$1'ooooooooool                         $2coooo."##,
        r##" $1,loooooooc.                        $2.loooo."##,
        r##"   $1.,;;;'.                          $2;ooooc"##,
        r##"       $2...                         $2,ooool."##,
        r##"    $2.cooooc.              $1..',,'.  $2.cooo."##,
        r##"      $2;ooooo:.           $1;oooooooc.  $2:l."##,
        r##"       $2.coooooc,..      $1coooooooooo."##,
        r##"         $2.:ooooooolc:. $1.ooooooooooo'"##,
        r##"           $2.':loooooo;  $1,oooooooooc"##,
        r##"               $2..';::c'  $1.;loooo:'"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: UBUNTU_ORANGE,
        colors: vec![UBUNTU_ORANGE, WHITE],
        width,
    }
}

fn mint() -> Logo {
    let lines = vec![
        r##"            $2_.-ppOOOOOOqq-._"##,
        r##"         .oOOOOPPPPPPPPPPOOOOo."##,
        r##"      .oOOOO$1.=oOOOOOOOOOOo=.$2OOOOo."##,
        r##"    .:OOO$1.=oOOOOOOOOOOOOOOOOo=.$2OOO:."##,
        r##"   .OOO$1.OOOOOOOOOOOOOOOOOOOOOOOO.$2OOO."##,
        r##"  .OOO$1.OO    OOO:´   `::´    `:OOO.$2OO:"##,
        r##" .OOO$1.OOO    OO                OOO.$2OOO:"##,
        r##" OOO$1.OOOO    OO    oo    oo    OOOO.$2OOO"##,
        r##":OOO$1:OOOO    OO    OO    OO    OOOO:$2OOO:"##,
        r##":OOO$1:OOOO    OO    OO    OO    OOOO:$2OOO:"##,
        r##"'OOO$1'OOOO    OO    OO    OO    OOOO'$2OOO'"##,
        r##" OOO$1'OOOO    OO____OO____OO    OOOO'$2OOO'"##,
        r##" 'OOO$1'OOO    'OOOOOOOOOOOO'    OOOO'$2OOO"##,
        r##"  'OOO$1'OOO                    .OOO'$2OOO'"##,
        r##"   'OOO$1'OOOO:ooooooooooooooo:OOOO'$2OOO'"##,
        r##"    ':OOOo$1'=OOOOOOOOOOOOOOOOO='$2oOOO:'"##,
        r##"      ':OOOOo$1'=OOOOOOOOOOO='$2oOOOO:'"##,
        r##"         ``-OOOOooooooooooOOOO-´´"##,
        r##"             ```-=:OOOO:=-´´´"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: MINT_GREEN,
        colors: vec![MINT_GREEN, WHITE],
        width,
    }
}

fn manjaro() -> Logo {
    let lines = vec![
        r##"██████████████████  ████████"##,
        r##"██████████████████  ████████"##,
        r##"██████████████████  ████████"##,
        r##"██████████████████  ████████"##,
        r##"████████            ████████"##,
        r##"████████  ████████  ████████"##,
        r##"████████  ████████  ████████"##,
        r##"████████  ████████  ████████"##,
        r##"████████  ████████  ████████"##,
        r##"████████  ████████  ████████"##,
        r##"████████  ████████  ████████"##,
        r##"████████  ████████  ████████"##,
        r##"████████  ████████  ████████"##,
        r##"████████  ████████  ████████"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: MANJARO_GREEN,
        colors: vec![MANJARO_GREEN],
        width,
    }
}

fn pop() -> Logo {
    let lines = vec![
        r##"             /////////////"##,
        r##"         /////////////////////"##,
        r##"      ///////$2*767$1////////////////"##,
        r##"    //////$27676767676*$1//////////////"##,
        r##"   /////$276767$1//$27676767$1//////////////"##,
        r##"  /////$2767676$1///$2*76767$1///////////////"##,
        r##" ///////$2767676$1///$276767$1.///$27676*$1///////"##,
        r##"/////////$2767676$1//$276767$1///$2767676$1////////"##,
        r##"//////////$276767676767$1////$276767$1/////////"##,
        r##"///////////$276767676$1//////$27676$1//////////"##,
        r##"////////////,$27676$1,///////$2767$1///////////"##,
        r##"/////////////*$27676$1///////$276$1////////////"##,
        r##"///////////////$27676$1////////////////////"##,
        r##" ///////////////$27676$1///$2767$1////////////"##,
        r##"  //////////////////////$2'$1////////////"##,
        r##"   //////$2.7676767676767676767,$1//////"##,
        r##"    /////$2767676767676767676767$1/////"##,
        r##"      ///////////////////////////"##,
        r##"         /////////////////////"##,
        r##"             /////////////"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: POP_TEAL,
        colors: vec![POP_TEAL, POP_ORANGE],
        width,
    }
}

fn alpine() -> Logo {
    let lines = vec![
        r##"       .hddddddddddddddddddddddh."##,
        r##"      :dddddddddddddddddddddddddd:"##,
        r##"     /dddddddddddddddddddddddddddd/"##,
        r##"    +dddddddddddddddddddddddddddddd+"##,
        r##"  `sdddddddddddddddddddddddddddddddds`"##,
        r##" `ydddddddddddd++hdddddddddddddddddddy`"##,
        r##".hddddddddddd+`  `+ddddh:-sdddddddddddh."##,
        r##"hdddddddddd+`      `+y:    .sddddddddddh"##,
        r##"ddddddddh+`   `//`   `.`     -sddddddddd"##,
        r##"ddddddh+`   `/hddh/`   `:s-    -sddddddd"##,
        r##"ddddh+`   `/+/dddddh/`   `+s-    -sddddd"##,
        r##"ddd+`   `/o` :dddddddh/`   `oy-    .yddd"##,
        r##"hdddyo+ohddyosdddddddddho+oydddy++ohdddh"##,
        r##".hddddddddddddddddddddddddddddddddddddh."##,
        r##" `yddddddddddddddddddddddddddddddddddy`"##,
        r##"  `sdddddddddddddddddddddddddddddddds`"##,
        r##"    +dddddddddddddddddddddddddddddd+"##,
        r##"     /dddddddddddddddddddddddddddd/"##,
        r##"      :dddddddddddddddddddddddddd:"##,
        r##"       .hddddddddddddddddddddddh."##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: ALPINE_BLUE,
        colors: vec![ALPINE_BLUE, WHITE],
        width,
    }
}

fn void() -> Logo {
    let lines = vec![
        r##"                __.;=====;.__"##,
        r##"            _.=+==++=++=+=+===;."##,
        r##"             -=+++=+===+=+=+++++=_"##,
        r##"        .     -=:``     `--==+=++==."##,
        r##"       _vi,    `            --+=++++:"##,
        r##"      .uvnvi.       _._       -==+==+."##,
        r##"     .vvnvnI`    .;==|==;.     :|=||=|."##,
        r##"$2+QmQQm$1pvvnv;$2 _yYsyQQWUUQQQm #QmQ#$1:$2QQQWUV$QQm."##,
        r##" $2-QQWQW$1pvvo$2wZ?.wQQQE$1==<$2QWWQ/QWQW.QQWW$1(:$2 jQWQE"##,
        r##"  $2-$QQQQmmU'  jQQQ$1@+=<$2QWQQ)mQQQ.mQQQC$1+;$2jWQQ@'"##,
        r##"   $2-$WQ8Y$1nI:$2   QWQQwgQQWV$1`$2mWQQ.jQWQQgyyWW@!"##,
        r##"     $1-1vvnvv.     `~+++`        ++|+++"##,
        r##"      +vnvnnv,                 `-|==="##,
        r##"       +vnvnvns.           .      :=-"##,
        r##"        -Invnvvnsi..___..=sv=.     `"##,
        r##"          +Invnvnvnnnnnnnnvvnn;."##,
        r##"            ~|Invnvnvvnvvvnnv}+`"##,
        r##"               -~|{*l}*|~"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: VOID_GREEN,
        colors: vec![VOID_GREEN, WHITE],
        width,
    }
}

fn gentoo() -> Logo {
    let lines = vec![
        r##"         -/oyddmdhs+:."##,
        r##"     -o$2dNMMMMMMMMNNmhy+$1-`"##,
        r##"   -y$2NMMMMMMMMMMMNNNmmdhy$1+-"##,
        r##" `o$2mMMMMMMMMMMMMNmdmmmmddhhy$1/`"##,
        r##" om$2MMMMMMMMMMMN$1hhyyyo$2hmdddhhhd$1o`"##,
        r##".y$2dMMMMMMMMMMd$1hs++so/s$2mdddhhhhdm$1+`"##,
        r##" oy$2hdmNMMMMMMMN$1dyooy$2dmddddhhhhyhN$1d."##,
        r##"  :o$2yhhdNNMMMMMMMNNNmmdddhhhhhyym$1Mh"##,
        r##"    .:$2+sydNMMMMMNNNmmmdddhhhhhhmM$1my"##,
        r##"       /m$2MMMMMMNNNmmmdddhhhhhmMNh$1s:"##,
        r##"    `o$2NMMMMMMMNNNmmmddddhhdmMNhs$1+`"##,
        r##"  `s$2NMMMMMMMMNNNmmmdddddmNMmhs$1/."##,
        r##" /N$2MMMMMMMMNNNNmmmdddmNMNdso$1:`"##,
        r##"+M$2MMMMMMNNNNNmmmmdmNMNdso$1/-"##,
        r##"yM$2MNNNNNNNmmmmmNNMmhs+/$1-`"##,
        r##"/h$2MMNNNNNNNNMNdhs++/$1-`"##,
        r##"`/$2ohdmmddhys+++/:$1.`"##,
        r##"  `-//////:--."##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: GENTOO_PURPLE,
        colors: vec![WHITE, GENTOO_PURPLE],
        width,
    }
}

fn nixos() -> Logo {
    let lines = vec![
        r##"          $1▗▄▄▄       $2▗▄▄▄▄    ▄▄▄▖"##,
        r##"          $1▜███▙       $2▜███▙  ▟███▛"##,
        r##"           $1▜███▙       $2▜███▙▟███▛"##,
        r##"            $1▜███▙       $2▜██████▛"##,
        r##"     $1▟█████████████████▙ $2▜████▛     $3▟▙"##,
        r##"    $1▟███████████████████▙ $2▜███▙    $3▟██▙"##,
        r##"           $6▄▄▄▄▖           $2▜███▙  $3▟███▛"##,
        r##"          $6▟███▛             $2▜██▛ $3▟███▛"##,
        r##"         $6▟███▛               $2▜▛ $3▟███▛"##,
        r##"$6▟███████████▛                  $3▟██████████▙"##,
        r##"$6▜██████████▛                  $3▟███████████▛"##,
        r##"      $6▟███▛ $5▟▙               $3▟███▛"##,
        r##"     $6▟███▛ $5▟██▙             $3▟███▛"##,
        r##"    $6▟███▛  $5▜███▙           $3▝▀▀▀▀"##,
        r##"    $6▜██▛    $5▜███▙ $4▜██████████████████▛"##,
        r##"     $6▜▛     $5▟████▙ $4▜████████████████▛"##,
        r##"           $5▟██████▙         $4▜███▙"##,
        r##"          $5▟███▛▜███▙         $4▜███▙"##,
        r##"         $5▟███▛  ▜███▙         $4▜███▙"##,
        r##"         $5▝▀▀▀    ▀▀▀▀▘         $4▀▀▀▘"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: NIXOS_CYAN,
        colors: vec![
            NIXOS_BLUE, NIXOS_CYAN, NIXOS_BLUE, NIXOS_CYAN, NIXOS_BLUE, NIXOS_CYAN,
        ],
        width,
    }
}

fn opensuse() -> Logo {
    let lines = vec![
        r##"          ,...,"##,
        r##"     .,:lloooooc;."##,
        r##"   ,ool'     oo,;oo:"##,
        r##" .lo'        oo.   oo:"##,
        r##".oo.         oo.    oo:"##,
        r##":ol          oo.    'oo"##,
        r##":oo         .oo.    .oo."##,
        r##".oooooooooooooo.    .oo."##,
        r##" ;oo.               .oo."##,
        r##"  'oo,              .oo."##,
        r##"    "ooc,',,,,,,,,,,:ooc,,,,,,,,,,,"##,
        r##"       ':cooooooooooooooooooooooooool;."##,
        r##"                    .oo.             .oo;"##,
        r##"                    .oo.               .oo."##,
        r##"                    .oo.    'oooooooooo:ooo."##,
        r##"                    .oo.    'oo.         col"##,
        r##"                    .oo'    'oo          col"##,
        r##"                     coo    'oo          oo'"##,
        r##"                      coc   'oo        .lo,"##,
        r##"                       `oo, 'oo      .:oo"##,
        r##"                         'ooooc,, ,:lol"##,
        r##"                            `''"clc"'"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: SUSE_GREEN,
        colors: vec![SUSE_GREEN],
        width,
    }
}

fn endeavour() -> Logo {
    let lines = vec![
        r##"                     $2./$1o$3."##,
        r##"                   $2./$1sssso$3-"##,
        r##"                 $2`:$1osssssss+$3-"##,
        r##"               $2`:+$1sssssssssso$3/."##,
        r##"             $2`-/o$1ssssssssssssso$3/."##,
        r##"           $2`-/+$1sssssssssssssssso$3+:`"##,
        r##"         $2`-:/+$1sssssssssssssssssso$3+/."##,
        r##"       $2`.://o$1sssssssssssssssssssso$3++-"##,
        r##"      $2.://+$1ssssssssssssssssssssssso$3++:"##,
        r##"    $2.:///o$1ssssssssssssssssssssssssso$3++:"##,
        r##"  $2`:////$1ssssssssssssssssssssssssssso$3+++."##,
        r##"$2`-////+$1ssssssssssssssssssssssssssso$3++++-"##,
        r##" $2`..-+$1oosssssssssssssssssssssssso$3+++++/`"##,
        r##"   $3./++++++++++++++++++++++++++++++/:."##,
        r##"  `:::::::::::::::::::::::::------``"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: ENDEAVOUR_PURPLE,
        colors: vec![ENDEAVOUR_PURPLE, ENDEAVOUR_RED, ENDEAVOUR_BLUE],
        width,
    }
}

fn kali() -> Logo {
    let lines = vec![
        r##".............."##,
        r##"            ..,;:ccc,."##,
        r##"          ......''';lxO."##,
        r##".....''''..........,:ld;"##,
        r##"           .';;;:::;,,.x,"##,
        r##"      ..'''.            0Xxoc:,.  ..."##,
        r##"  ....                ,ONkc;,;cokOdc',."##,
        r##" .                   OMo           ':$2dd$1o."##,
        r##"                    dMc               :OO;"##,
        r##"                    0M.                 .:o."##,
        r##"                    ;Wd"##,
        r##"                     ;XO,"##,
        r##"                       ,d0Odlc;,.."##,
        r##"                           ..',;:cdOOd::,."##,
        r##"                                    .:d;.':;."##,
        r##"                                       'd,  .'"##,
        r##"                                         ;l   .."##,
        r##"                                          .o"##,
        r##"                                            c"##,
        r##"                                            .'"##,
        r##"                                             ."##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: KALI_BLUE,
        colors: vec![KALI_BLUE, WHITE],
        width,
    }
}

fn steamos() -> Logo {
    let lines = vec![
        r##"$1              .,,,,."##,
        r##"        .,'onNMMMMMNNnn',."##,
        r##"     .'oNMANKMMMMMMMMMMMNNn'."##,
        r##"   .'ANMMMMMMMXKNNWWWPFFWNNMNn."##,
        r##"  ;NNMMMMMMMMMMNWW'' ,.., 'WMMM,"##,
        r##" ;NMMMMV+##+VNWWW' .+;'':+, 'WMW,"##,
        r##",VNNWP+$2######$1+WW,  $2+:    $1:+, +MMM,"##,
        r##"'$2+#############,   +.    ,+' $1+NMMM"##,
        r##"$2  '*#########*'     '*,,*' $1.+NMMMM."##,
        r##"$2     `'*###*'          ,.,;###$1+WNM,"##,
        r##"$2         .,;;,      .;##########$1+W"##,
        r##"$2,',.         ';  ,+##############'"##,
        r##" '###+. :,. .,; ,###############'"##,
        r##"  '####.. `'' .,###############'"##,
        r##"    '#####+++################'"##,
        r##"      '*##################*'"##,
        r##"         ''*##########*''"##,
        r##"              ''''''"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: STEAM_BLUE,
        colors: vec![STEAM_BLUE, WHITE],
        width,
    }
}

fn artix() -> Logo {
    let lines = vec![
        r##"                   '"##,
        r##"                  'o'"##,
        r##"                 'ooo'"##,
        r##"                'ooxoo'"##,
        r##"               'ooxxxoo'"##,
        r##"              'oookkxxoo'"##,
        r##"             'oiioxkkxxoo'"##,
        r##"            ':;:iiiioxxxoo'"##,
        r##"               `'.;::ioxxoo'"##,
        r##"          '-.      `':;jiooo'"##,
        r##"         'oooio-..     `'i:io'"##,
        r##"        'ooooxxxxoio:,.   `'-;'"##,
        r##"       'ooooxxxxxkkxoooIi:-.  `'"##,
        r##"      'ooooxxxxxkkkkxoiiiiiji'"##,
        r##"     'ooooxxxxxkxxoiiii:'`     .i'"##,
        r##"    'ooooxxxxxoi:::'`       .;ioxo'"##,
        r##"   'ooooxooi::'`         .:iiixkxxo'"##,
        r##"  'ooooi:'`                `'';ioxxo'"##,
        r##" 'i:'`                          '':io'"##,
        r##"'`                                   `'"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: ARTIX_CYAN,
        colors: vec![ARTIX_CYAN],
        width,
    }
}

fn redhat() -> Logo {
    let lines = vec![
        r##"           .MMM..:MMMMMMM"##,
        r##"          MMMMMMMMMMMMMMMMMM"##,
        r##"          MMMMMMMMMMMMMMMMMMMM."##,
        r##"         MMMMMMMMMMMMMMMMMMMMMM"##,
        r##"        ,MMMMMMMMMMMMMMMMMMMMMM:"##,
        r##"        MMMMMMMMMMMMMMMMMMMMMMMM"##,
        r##"  .MMMM'  MMMMMMMMMMMMMMMMMMMMMM"##,
        r##" MMMMMM    `MMMMMMMMMMMMMMMMMMMM."##,
        r##"MMMMMMMM      MMMMMMMMMMMMMMMMMM ."##,
        r##"MMMMMMMMM.       `MMMMMMMMMMMMM' MM."##,
        r##"MMMMMMMMMMM.                     MMMM"##,
        r##"`MMMMMMMMMMMMM.                 ,MMMMM."##,
        r##" `MMMMMMMMMMMMMMMMM.          ,MMMMMMMM."##,
        r##"    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM"##,
        r##"      MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM:"##,
        r##"         MMMMMMMMMMMMMMMMMMMMMMMMMMMMMM"##,
        r##"            `MMMMMMMMMMMMMMMMMMMMMMMM:"##,
        r##"                ``MMMMMMMMMMMMMMMMM'"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: REDHAT_RED,
        colors: vec![REDHAT_RED],
        width,
    }
}

fn rocky() -> Logo {
    let lines = vec![
        r##"          __wgliliiligw_,"##,
        r##"       _williiiiiiliilililw,"##,
        r##"     _%iiiiiilililiiiiiiiiiii_"##,
        r##"   .Qliiiililiiiiiiililililiilm."##,
        r##"  _iiiiiliiiiiililiiiiiiiiiiliil,"##,
        r##" .lililiiilililiiiilililililiiiii,"##,
        r##"_liiiiiiliiiiiiiliiiiiF{iiiiiilili,"##,
        r##"jliililiiilililiiili@`  ~ililiiiiiL"##,
        r##"iiiliiiiliiiiiiili>`      ~liililii"##,
        r##"liliiiliiilililii`         -9liiiil"##,
        r##"iiiiiliiliiiiii~             "4lili"##,
        r##"4ililiiiiilil~|      -w,       )4lf"##,
        r##"-liiiiililiF'       _liig,       )'"##,
        r##" )iiiliii@`       _QIililig,"##,
        r##"  )iiii>`       .Qliliiiililw"##,
        r##"   )<>~       .mliiiiiliiiiiil,"##,
        r##"            _gllilililiililii~"##,
        r##"           giliiiiiiiiiiiiT`"##,
        r##"          -^~$ililili@~~'"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: SUSE_GREEN,
        colors: vec![SUSE_GREEN, REDHAT_RED],
        width,
    }
}

fn almalinux() -> Logo {
    let lines = vec![
        r##"$1         'c:."##,
        r##"$1        lkkkx, ..       $2..   ,cc,"##,
        r##"$1        okkkk:ckkx'  $2.lxkkx.okkkkd"##,
        r##"$1        .:llcokkx'  $2:kkkxkko:xkkd,"##,
        r##"$1      .xkkkkdood:  $2;kx,  .lkxlll;"##,
        r##"$1       xkkx.       $2xk'     xkkkkk:"##,
        r##"$1       'xkx.       $2xd      .....,."##,
        r##"$3      .. $1:xkl'     $2:c      ..''.."##,
        r##"$3    .dkx'  $1.:ldl:'. $2'  $4':lollldkkxo;"##,
        r##"$3  .''lkkko'                     $4ckkkx."##,
        r##"$3'xkkkd:kkd.       ..  $5;'        $4:kkxo."##,
        r##"$3,xkkkd;kk'      ,d;    $5ld.   $4':dkd::cc,"##,
        r##"$3 .,,.;xkko'.';lxo.      $5dx,  $4:kkk'xkkkkc"##,
        r##"$3     'dkkkkkxo:.        $5;kx  $4.kkk:;xkkd."##,
        r##"$3       .....   $5.;dk:.   $5lkk.  $4:;,"##,
        r##"             $5:kkkkkkkdoxkkx"##,
        r##"              ,c,,;;;:xkkd."##,
        r##"                ;kkkkl..."##,
        r##"                ;kkkkl"##,
        r##"                 ,od;"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: DEBIAN_RED,
        colors: vec![DEBIAN_RED, TUX_YELLOW, SUSE_GREEN, ARCH_CYAN, FEDORA_BLUE],
        width,
    }
}

fn centos() -> Logo {
    let lines = vec![
        r##"                 .."##,
        r##"               .PLTJ."##,
        r##"              <><><><>"##,
        r##"     $2KKSSV' 4KKK $1LJ$4 KKKL.'VSSKK"##,
        r##"     $2KKV' 4KKKKK $1LJ$4 KKKKAL 'VKK"##,
        r##"     $2V' ' 'VKKKK $1LJ$4 KKKKV' ' 'V"##,
        r##"     $2.4MA.' 'VKK $1LJ$4 KKV' '.4Mb."##,
        r##"   $4. $2KKKKKA.' 'V $1LJ$4 V' '.4KKKKK $3."##,
        r##" $4.4D $2KKKKKKKA.'' $1LJ$4 ''.4KKKKKKK $3FA."##,
        r##"$4<QDD ++++++++++++  $3++++++++++++ GFD>"##,
        r##" '$4VD $3KKKKKKKK'.. $2LJ $1..'KKKKKKKK $3FV"##,
        r##"   $4' $3VKKKKK'. .4 $2LJ $1K. .'KKKKKV $3'"##,
        r##"      $3'VK'. .4KK $2LJ $1KKA. .'KV'"##,
        r##"     $3A. . .4KKKK $2LJ $1KKKKA. . .4"##,
        r##"     $3KKA. 'KKKKK $2LJ $1KKKKK' .4KK"##,
        r##"     $3KKSSA. VKKK $2LJ $1KKKV .4SSKK"##,
        r##"              $2<><><><>"##,
        r##"               $2'MKKM'"##,
        r##"                 $2''"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: REDHAT_RED,
        colors: vec![REDHAT_RED, SUSE_GREEN, FEDORA_BLUE, GENTOO_PURPLE],
        width,
    }
}

fn zorin() -> Logo {
    let lines = vec![
        r##"        `osssssssssssssssssssso`"##,
        r##"       .osssssssssssssssssssssso."##,
        r##"      .+oooooooooooooooooooooooo+."##,
        r##""##,
        r##""##,
        r##"  `::::::::::::::::::::::.         .:`"##,
        r##" `+ssssssssssssssssss+:.`     `.:+ssso`"##,
        r##".ossssssssssssssso/.       `-+ossssssso."##,
        r##"ssssssssssssso/-`      `-/osssssssssssss"##,
        r##".ossssssso/-`      .-/ossssssssssssssso."##,
        r##" `+sss+:.      `.:+ssssssssssssssssss+`"##,
        r##"  `:.         .::::::::::::::::::::::`"##,
        r##""##,
        r##""##,
        r##"      .+oooooooooooooooooooooooo+."##,
        r##"       -osssssssssssssssssssssso-"##,
        r##"        `osssssssssssssssssssso`"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: ZORIN_BLUE,
        colors: vec![ZORIN_BLUE],
        width,
    }
}

fn raspbian() -> Logo {
    let lines = vec![
        r##"   $2`.::///+:/-.        --///+//-:`"##,
        r##" `+oooooooooooo:   `+oooooooooooo:"##,
        r##"  /oooo++//ooooo:  ooooo+//+ooooo."##,
        r##"  `+ooooooo:-:oo-  +o+::/ooooooo:"##,
        r##"   `:oooooooo+``    `.oooooooo+-"##,
        r##"     `:++ooo/.        :+ooo+/.`$1"##,
        r##"        ...`  `.----.` ``.."##,
        r##"     .::::-``:::::::::.`-:::-`"##,
        r##"    -:::-`   .:::::::-`  `-:::-"##,
        r##"   `::.  `.--.`  `` `.---.``.::`"##,
        r##"       .::::::::`  -::::::::` `"##,
        r##" .::` .:::::::::- `::::::::::``::."##,
        r##"-:::` ::::::::::.  ::::::::::.`:::-"##,
        r##"::::  -::::::::.   `-::::::::  ::::"##,
        r##"-::-   .-:::-.``....``.-::-.   -::-"##,
        r##" .. ``       .::::::::.     `..`.."##,
        r##"   -:::-`   -::::::::::`  .:::::`"##,
        r##"   :::::::` -::::::::::` :::::::."##,
        r##"   .:::::::  -::::::::. ::::::::"##,
        r##"    `-:::::`   ..--.`   ::::::."##,
        r##"      `...`  `...--..`  `...`"##,
        r##"            .::::::::::"##,
        r##"             `.-::::-`"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: RASPBIAN_RED,
        colors: vec![RASPBIAN_GREEN, RASPBIAN_RED],
        width,
    }
}

fn linux() -> Logo {
    let lines = vec![
        r##"        $2#####"##,
        r##"       $2#######"##,
        r##"       $2##$1O$2#$1O$2##"##,
        r##"       $2#$3#####$2#"##,
        r##"     $2##$1##$3###$1##$2##"##,
        r##"    $2#$1##########$2##"##,
        r##"   $2#$1############$2##"##,
        r##"   $2#$1############$2###"##,
        r##"  $3##$2#$1###########$2##$3#"##,
        r##"$3######$2#$1#######$2#$3######"##,
        r##"$3#######$2#$1#####$2#$3#######"##,
        r##"  $3#####$2#######$3#####"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: WHITE,
        colors: vec![WHITE, TUX_DARK, TUX_YELLOW],
        width,
    }
}

fn cachyos() -> Logo {
    let lines = vec![
        r##"           $3.$1-------------------------:"##,
        r##"          .$2+=$1========================."##,
        r##"         :$2++$1===$2++===$1===============-       :$2++$1-"##,
        r##"        :$2*++$1====$2+++++==$1===========-        .==:"##,
        r##"       -$2*+++$1=====$2+***++=$1=========:"##,
        r##"      =$2*++++=$1=======------------:"##,
        r##"     =$2*+++++=$1====-                     $3...$1"##,
        r##"   .$2+*+++++$1=-===:                    .$2=+++=$1:"##,
        r##"  :$2++++$1=====-==:                     -***$2**$1+"##,
        r##" :$2++=$1=======-=.                      .=+**+$3.$1"##,
        r##".$2+$1==========-.                          $3.$1"##,
        r##" :$2+++++++$1====-                                $3.$1--==-$3.$1"##,
        r##"  :$2++$1==========.                             $3:$2+++++++$1$3:"##,
        r##"   $1.-===========.                            =*****+*+"##,
        r##"    $1.-===========:                           .+*****+:"##,
        r##"      $1-=======$2++++$1:::::::::::::::::::::::::-:  $3.$1---:"##,
        r##"       :======$2++++$1====$2+++******************=."##,
        r##"        $1:=====$2+++$1==========$2++++++++++++++*-"##,
        r##"         $1.====$2++$1==============$2++++++++++*-"##,
        r##"          $1.===$2+$1==================$2+++++++:"##,
        r##"           $1.-=======================$2+++:"##,
        r##"             $3.........................."##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: CACHYOS_CYAN,
        colors: vec![CACHYOS_CYAN, CACHYOS_GREEN, CACHYOS_DARK],
        width,
    }
}

fn cachyos_small() -> Logo {
    let lines = vec![
        r##"     ..........."##,
        r##"    $2/,,,,$1...../"##,
        r##"   $2/,,,$1..$2,,$1../   $2()"##,
        r##"  $2/,,,,$1../"##,
        r##" $2/,,,$1.../     $1/'\"##,
        r##"$2/,$1...../      $2\,/"##,
        r##"$2\,,,,,,\\             $1_"##,
        r##" $2\,,$1....\\           $1/ \"##,
        r##"  $1\...$2,$1..\\          $2\_/"##,
        r##"   $1\..$2,,,,,,,,,,,,/"##,
        r##"    $1\$2,,$1........$2,,/"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: CACHYOS_CYAN,
        colors: vec![CACHYOS_GREEN, CACHYOS_CYAN],
        width,
    }
}

fn freebsd() -> Logo {
    let lines = vec![
        r##"```                        $2`"##,
        r##"  $1` `.....---...$2....--.```   -/"##,
        r##"  $1+o   .--`         $2/y:`      +."##,
        r##"   $1yo`:.            $2:o      `+-"##,
        r##"    $1y/               $2-/`   -o/"##,
        r##"   $1.-                  $2::/sy+:."##,
        r##"   $1/                     $2`--  /"##,
        r##"  $1`:                          $2:`"##,
        r##"  $1`:                          $2:`"##,
        r##"   $1/                          $2/"##,
        r##"   $1.-                        $2-."##,
        r##"    $1--                      $2-."##,
        r##"     $1`:`                  $2`:`"##,
        r##"       .--             `--."##,
        r##"          .---.....----."##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: FREEBSD_RED,
        colors: vec![FREEBSD_WHITE, FREEBSD_RED],
        width,
    }
}

fn freebsd_small() -> Logo {
    let lines = vec![
        r##"$1/\,-'''''-,/\"##,
        r##"\_)       (_/"##,
        r##"|           |"##,
        r##"|           |"##,
        r##" ;         ;"##,
        r##"  '-_____-'"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: FREEBSD_RED,
        colors: vec![FREEBSD_RED],
        width,
    }
}

fn nixos_small() -> Logo {
    let lines = vec![
        r##"$1  ▗▄   $2▗▄ ▄▖"##,
        r##"$1 ▄▄🬸█▄▄▄$2🬸█▛ $3▃"##,
        r##"$6   ▟▛    $2▜$3▃▟🬕"##,
        r##"$6🬋🬋🬫█      $3█🬛🬋🬋"##,
        r##"$6 🬷▛🮃$5▙    $3▟▛"##,
        r##"$6 🮃 $5▟█🬴$4▀▀▀█🬴▀▀"##,
        r##"  $5▝▀ ▀▘   $4▀▘"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: NIXOS_CYAN,
        colors: vec![
            NIXOS_BLUE, NIXOS_CYAN, NIXOS_BLUE, NIXOS_CYAN, NIXOS_BLUE, NIXOS_CYAN,
        ],
        width,
    }
}

fn arch_small() -> Logo {
    let lines = vec![
        r##"      /\"##,
        r##"     /  \"##,
        r##"    /    \"##,
        r##"   /      \"##,
        r##"$2  /   ,,   \"##,
        r##" /   |  |   \"##,
        r##"/_-''    ''-_\"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: ARCH_CYAN,
        colors: vec![ARCH_CYAN, ARCH_LIGHT],
        width,
    }
}

fn fedora_small() -> Logo {
    let lines = vec![
        r##"        ,'''''."##,
        r##"       |   ,.  |"##,
        r##"       |  |  '_'"##,
        r##"  ,....|  |.."##,
        r##".'  ,_;|   ..'"##,
        r##"|  |   |  |"##,
        r##"|  ',_,'  |"##,
        r##" '.     ,'"##,
        r##"   '''''"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: FEDORA_BLUE,
        colors: vec![FEDORA_BLUE],
        width,
    }
}

fn ubuntu_small() -> Logo {
    let lines = vec![
        r##"       $2..;,; $1.,;,."##,
        r##"    $2.,lool: $1.ooooo,"##,
        r##"   $2;oo;:    $1.coool."##,
        r##" $1....         $1''' $2,l;"##,
        r##"$1:oooo,            $2'oo."##,
        r##"$1looooc            $2:oo'"##,
        r##" $1'::'             $2,oo:"##,
        r##"   $2,.,       $1.... $2co,"##,
        r##"    $2lo:;.   $1:oooo; $2."##,
        r##"     $2':ooo; $1cooooc"##,
        r##"        $2'''  $1''''"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: UBUNTU_ORANGE,
        colors: vec![WHITE, UBUNTU_ORANGE],
        width,
    }
}

fn debian_small() -> Logo {
    let lines = vec![
        r##"  _____"##,
        r##" /  __ \"##,
        r##"|  /    |"##,
        r##"|  \___-"##,
        r##"-_"##,
        r##"  --_"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: DEBIAN_RED,
        colors: vec![DEBIAN_RED],
        width,
    }
}

fn openbsd() -> Logo {
    let lines = vec![
        r##"$3                                     _"##,
        r##"                                    (_)"##,
        r##"$1              |    ."##,
        r##"$1          .   |L  /|   .         $3 _"##,
        r##"$1      _ . |\ _| \--+._/| .       $3(_)"##,
        r##"$1     / ||\| Y J  )   / |/| ./"##,
        r##"    J  |)'( |        ` F`.'/       $3 _"##,
        r##"$1  -<|  F         __     .-<        $3(_)"##,
        r##"$1    | /       .-'$3. $1`.  /$3-. $1L___"##,
        r##"    J \      <    $3\ $1 | | $5O$3\$1|.-' $3 _"##,
        r##"$1  _J \  .-    \$3/ $5O $3| $1| \  |$1F    $3(_)"##,
        r##"$1 '-F  -<_.     \   .-'  `-' L__"##,
        r##"__J  _   _.     >-'  $1)$4._.   $1|-'"##,
        r##"$1 `-|.'   /_.          $4\_|  $1 F"##,
        r##"  /.-   .                _.<"##,
        r##" /'    /.'             .'  `\"##,
        r##"  /L  /'   |/      _.-'-\"##,
        r##" /'J       ___.---'\|"##,
        r##"   |\  .--' V  | `. `"##,
        r##"   |/`. `-.     `._)"##,
        r##"      / .-.\"##,
        r##"      \ (  `\"##,
        r##"       `.\"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: OPENBSD_YELLOW,
        colors: vec![OPENBSD_BLUE, WHITE, OPENBSD_YELLOW],
        width,
    }
}

fn netbsd() -> Logo {
    let lines = vec![
        r##"$1                     `-/oshdmNMNdhyo+:-`"##,
        r##"$2y$1/s+:-``    `.-:+oydNMMMMNhs/-``"##,
        r##"$2-m+$1NMMMMMMMMMMMMMMMMMMMNdhmNMMMmdhs+/-`"##,
        r##" $2-m+$1NMMMMMMMMMMMMMMMMMMMMmy+:`"##,
        r##"  $2-N/$1dMMMMMMMMMMMMMMMds:`"##,
        r##"   $2-N/$1hMMMMMMMMMmho:`"##,
        r##"    $2-N/$1-:/++/:.`"##,
        r##"$2     :M+"##,
        r##"      :Mo"##,
        r##"       :Ms"##,
        r##"        :Ms"##,
        r##"         :Ms"##,
        r##"          :Ms"##,
        r##"           :Ms"##,
        r##"            :Ms"##,
        r##"             :Ms"##,
        r##"              :Ms"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: NETBSD_ORANGE,
        colors: vec![WHITE, NETBSD_ORANGE],
        width,
    }
}

fn macos() -> Logo {
    let lines = vec![
        r##"                     ..'"##,
        r##"                 ,xNMM."##,
        r##"               .OMMMMo"##,
        r##"               lMM""##,
        r##"     .;loddo:.  .olloddol;."##,
        r##"   cKMMMMMMMMMMNWMMMMMMMMMM0:"##,
        r##" $2.KMMMMMMMMMMMMMMMMMMMMMMMWd."##,
        r##" XMMMMMMMMMMMMMMMMMMMMMMMX."##,
        r##"$3;MMMMMMMMMMMMMMMMMMMMMMMM:"##,
        r##":MMMMMMMMMMMMMMMMMMMMMMMM:"##,
        r##"$4.MMMMMMMMMMMMMMMMMMMMMMMMX."##,
        r##" kMMMMMMMMMMMMMMMMMMMMMMMMWd."##,
        r##" $5'XMMMMMMMMMMMMMMMMMMMMMMMMMMk"##,
        r##"  'XMMMMMMMMMMMMMMMMMMMMMMMMK."##,
        r##"    $6kMMMMMMMMMMMMMMMMMMMMMMd"##,
        r##"     ;KMMMMMMMWXXWMMMMMMMk."##,
        r##"       "cooc*"    "*coo'""##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: MACOS_GREEN,
        colors: vec![
            MACOS_GREEN,
            MACOS_YELLOW,
            MACOS_ORANGE,
            MACOS_RED,
            MACOS_PURPLE,
            MACOS_BLUE,
        ],
        width,
    }
}

fn windows() -> Logo {
    let lines = vec![
        r##"$1        ,.=:!!t3Z3z.,"##,
        r##"       :tt:::tt333EE3"##,
        r##"$1       Et:::ztt33EEEL$2 @Ee.,      ..,"##,
        r##"$1      ;tt:::tt333EE7$2 ;EEEEEEttttt33#"##,
        r##"$1     :Et:::zt333EEQ.$2 $EEEEEttttt33QL"##,
        r##"$1     it::::tt333EEF$2 @EEEEEEttttt33F"##,
        r##"$1    ;3=*^```"*4EEV$2 :EEEEEEttttt33@."##,
        r##"$3    ,.=::::!t=., $1`$2 @EEEEEEtttz33QF"##,
        r##"$3   ;::::::::zt33)$2   "4EEEtttji3P*"##,
        r##"$3  :t::::::::tt33.$4:Z3z..$2  ``$4 ,..g."##,
        r##"$3  i::::::::zt33F$4 AEEEtttt::::ztF"##,
        r##"$3 ;:::::::::t33V$4 ;EEEttttt::::t3"##,
        r##"$3 E::::::::zt33L$4 @EEEtttt::::z3F"##,
        r##"$3{3=*^```"*4E3)$4 ;EEEtttt:::::tZ`"##,
        r##"$3             `$4 :EEEEtttt::::z7"##,
        r##"                 "VEzjt:;;z>*`"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: WINDOWS_BLUE,
        colors: vec![WINDOWS_RED, WINDOWS_GREEN, WINDOWS_BLUE, WINDOWS_YELLOW],
        width,
    }
}

fn garuda() -> Logo {
    let lines = vec![
        r##"                   .%;888:8898898:"##,
        r##"                 x;XxXB%89b8:b8%b88:"##,
        r##"              .8Xxd                8X:."##,
        r##"            .8Xx;                    8x:."##,
        r##"          .tt8x          .d            x88;"##,
        r##"       .@8x8;          .db:              xx@;"##,
        r##"     ,tSXX°          .bbbbbbbbbbbbbbbbbbbB8x@;"##,
        r##"   .SXxx            bBBBBBBBBBBBBBBBBBBBbSBX8;"##,
        r##" ,888S                                     pd!"##,
        r##"8X88/                                       q"##,
        r##"8X88/"##,
        r##"GBB."##,
        r##" x%88        d888@8@X@X@X88X@@XX@@X@8@X."##,
        r##"   dxXd    dB8b8b8B8B08bB88b998888b88x."##,
        r##"    dxx8o                      .@@;."##,
        r##"      dx88                   .t@x."##,
        r##"        d:SS@8ba89aa67a853Sxxad."##,
        r##"          .d988999889889899dd."##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: GARUDA_ORANGE,
        colors: vec![GARUDA_ORANGE, GARUDA_YELLOW],
        width,
    }
}

fn nobara() -> Logo {
    let lines = vec![
        r##"⢀⣤⣴⣶⣶⣶⣦⣤⡀⠀⣀⣠⣤⣴⣶⣶⣶⣶⣶⣶⣶⣶⣤⣤⣀⡀"##,
        r##"⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣤⡀"##,
        r##"⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄"##,
        r##"⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄"##,
        r##"⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧"##,
        r##"⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠋⠉⠁⠀⠀⠉⠉⠛⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧"##,
        r##"⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠁⠀⠀⠀⢀⣀⣀⡀⠀⠀⠀⠈⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇"##,
        r##"⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⠀⠀⠀⢠⣾⣿⣿⣿⣿⣷⡄⠀⠀⠀⠻⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿"##,
        r##"⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠁⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⣀⣀⣬⣽⣿⣿⣿⣿⣿⣿"##,
        r##"⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠈⠻⢿⣿⣿⡿⠟⠁⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿"##,
        r##"⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿"##,
        r##"⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣤⣤⣄⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿"##,
        r##"⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿"##,
        r##"⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣇⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿"##,
        r##"⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠟⠛⠉⠉⠛⠛⢿⣿⣿⠀⠀⠀⠀⠀⠸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿"##,
        r##"⠘⢿⣿⣿⣿⣿⣿⣿⣿⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠈⢿⠀⠀⠀⠀⠀⠀⠙⢿⣿⣿⣿⣿⣿⣿⣿⠟⠁"##,
        r##"  ⠈⠙⠛⠛⠛⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⠛⠛⠛⠉⠁"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: NOBARA_WHITE,
        colors: vec![NOBARA_WHITE],
        width,
    }
}

fn archcraft() -> Logo {
    let lines = vec![
        r##"$1⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄$1⢰⡆$1⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄"##,
        r##"$2⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄$1⢠⣿⣿⡄$2⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄"##,
        r##"$3⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄$1⢀⣾⣿⣿⣿⡀$3⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄"##,
        r##"$4⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄$1⣼⣿⣿⣿⣿⣷⡀$4⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄"##,
        r##"$5⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄$1⣼⣿⣿⣿⣿⣿⣿⣷$5⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄"##,
        r##"$6⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄$1⢼⣿⣿⣿⣿⣿⣿⣿⣿⣧$6⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄"##,
        r##"$1⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄$1⣰⣤⣈⠻⢿⣿⣿⣿⣿⣿⣿⣧$1⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄"##,
        r##"$2⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄$1⣰⣿⣿⣿⣿⣮⣿⣿⣿⣿⣿⣿⣿⣧$2⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄"##,
        r##"$3⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄$1⣰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧$3⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄"##,
        r##"$4⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄$1⣰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧$4⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄"##,
        r##"$5⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄$1⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧$5⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄"##,
        r##"$6⠄⠄⠄⠄⠄⠄⠄⠄⠄$1⣼⣿⣿⣿⣿⣿⡿⣿⣿⡟$6⠄⠄$1⠸⣿⣿⡿⣿⣿⣿⣿⣿⣷⡀$6⠄⠄⠄⠄⠄⠄⠄⠄"##,
        r##"$1⠄⠄⠄⠄⠄⠄⠄⠄$1⣼⣿⣿⣿⣿⣿⡏$1⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄$1⠈⣿⣿⣿⣿⣿⣷⡀$1⠄⠄⠄⠄⠄⠄⠄"##,
        r##"$2⠄⠄⠄⠄⠄⠄$1⢀⣼⣿⣿⣿⣿⣿⣿⡗$2⠄⠄⠄$1⢀⣠⣤⣀⠄⠄⠄$1⠸⣿⣿⣿⣿⣿⣿⣷⡀$2⠄⠄⠄⠄⠄⠄"##,
        r##"$3⠄⠄⠄⠄⠄$1⢀⣾⣿⣿⣿⣿⣿⡏⠁$3⠄⠄⠄$1⢠⣿⣿⣿⣿⡇$3⠄⠄⠄⠄$1⢙⣿⣿⣻⠿⣿⣷⡀$3⠄⠄⠄⠄⠄"##,
        r##"$4⠄⠄⠄⠄$1⢀⣾⣿⣿⣿⣿⣿⣿⣷⣤⡀$4⠄⠄⠄$1⠻⣿⣿⡿⠃$4⠄⠄⠄$1⢀⣼⣿⣿⣿⣿⣦⣌⠙⠄$4⠄⠄⠄⠄"##,
        r##"$5⠄⠄⠄$1⢠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⠏$5⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄$1⢿⣿⣿⣿⣿⣿⣿⣿⣿⣦⡀$5⠄⠄⠄"##,
        r##"$6⠄⠄$1⢠⣿⣿⣿⣿⣿⣿⣿⡿⠟⠋⠁$6⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄$1⠙⠻⣿⣿⣿⣿⣿⣿⣿⣿⡄$6⠄⠄"##,
        r##"$1⠄$1⣠⣿⣿⣿⣿⠿⠛⠋⠁$1⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄$1⠉⠙⠻⢿⣿⣿⣿⣿⣆$1⠄"##,
        r##"$1⡰⠟⠛⠉⠁$2⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄$1⠉⠙⠛⠿⢆"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: ARCHCRAFT_TEAL,
        colors: vec![ARCHCRAFT_TEAL, ARCH_CYAN, ARCH_LIGHT, WHITE],
        width,
    }
}

fn cat_mascot() -> Logo {
    let lines = vec![
        r##"       $1|\      _,,,---,,_"##,
        r##" $2ZZZzz $1/,`.-'`'    -.  ;-;;,_"##,
        r##"      $1|,4-  ) )-,_. ,\ (  `'-'"##,
        r##"     $2'---''(_/--'  `-'\_)"##,
        r##""##,
        r##"        $1/\_/\   $2.---."##,
        r##"       $1( $3o$1.$3o$1 ) $2/  $1RustFetch"##,
        r##"        $1> ^ <  $2'---'"##,
        r##"       $1/|   |\"##,
        r##"      $1(_|   |_)"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: CAT_AMBER,
        colors: vec![CAT_AMBER, CAT_CREAM, CAT_EMERALD],
        width,
    }
}

fn bengal_cat() -> Logo {
    let lines = vec![
        r##"         $1/\_____/\"##,
        r##"        $1/  $3o   o  $1\"##,
        r##"       $1( ==  $2^  $1== )"##,
        r##"        $1)  $2~   ~  $1("##,
        r##"       $1(  $2(  .  )  $1)"##,
        r##"      $1( (  )   (  ) )"##,
        r##"     $1(__(__)___(__)__)"##,
        r##"        $2"Bengal" v0.1.2"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: CAT_AMBER,
        colors: vec![CAT_AMBER, UBUNTU_ORANGE, CAT_EMERALD],
        width,
    }
}

fn maine_coon_cat() -> Logo {
    let lines = vec![
        r##"         $1/\_ _ _/\"##,
        r##"        $1/  $3>   <  $1\"##,
        r##"       $1( ==  $2*  $1== )"##,
        r##"        $1)  $2~   ~  $1("##,
        r##"       $1(  $2/|   |\ $1)"##,
        r##"      $1( (  )   (  ) )"##,
        r##"     $1(__(__)___(__)__)"##,
        r##"      $2"Maine Coon" v0.1.3"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: CAT_AMBER,
        colors: vec![CAT_AMBER, UBUNTU_ORANGE, CAT_EMERALD],
        width,
    }
}

fn siamese_cat() -> Logo {
    let lines = vec![
        r##"         $1/\_____/\"##,
        r##"        $1/  $3*   *  $1\"##,
        r##"       $1( ==  $2^  $1== )"##,
        r##"        $1)   $2.-.   $1("##,
        r##"       $1(   $2/   \   $1)"##,
        r##"      $1( (  )   (  ) )"##,
        r##"     $1(__(__)___(__)__)"##,
        r##"       $2"Siamese" v0.1.4"##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: CAT_CREAM,
        colors: vec![CAT_CREAM, UBUNTU_ORANGE, ARCH_LIGHT],
        width,
    }
}

fn turkish_van() -> Logo {
    let lines = vec![
        r##"         $3/\_____/\"##,
        r##"        $1/  $2o   $3o  $1\"##,
        r##"       $1( ==  ^  == )"##,
        r##"        $1)         ("##,
        r##"       $1(           )"##,
        r##"      $1( (  )   (  ) )"##,
        r##"     $1(__(__)___(__)__)"##,
        r##"       $2"Turkish Van""##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: WHITE,
        colors: vec![WHITE, ARCH_LIGHT, CAT_AMBER],
        width,
    }
}

fn turkish_angora() -> Logo {
    let lines = vec![
        r##"         $1/\_____/\"##,
        r##"        $1/  $2o   o  $1\"##,
        r##"       $1( ==  $3^  $1== )"##,
        r##"        $1)  ~   ~  ("##,
        r##"       $1(           )"##,
        r##"      $1( (  )   (  ) )"##,
        r##"     $1(__(__)___(__)__)"##,
        r##"      $2"Turkish Angora""##,
    ];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: CAT_CREAM,
        colors: vec![WHITE, NIXOS_CYAN, CAT_CREAM],
        width,
    }
}

fn cat_small() -> Logo {
    let lines = vec![r##" $1/\_/\"##, r##"$1( $2o$1.$2o$1 )"##, r##" $1> ^ <"##];
    let width = compute_logo_width(&lines);
    Logo {
        lines,
        color: CAT_AMBER,
        colors: vec![CAT_AMBER, CAT_EMERALD],
        width,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_logo_fedora() {
        let logo = get_logo("fedora");
        assert!(!logo.lines.is_empty());
        assert_eq!(logo.color, FEDORA_BLUE);
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
        assert!(!logo.lines.is_empty());
        assert_eq!(logo.color, ARCH_CYAN);
    }

    #[test]
    fn test_get_logo_unknown_returns_linux() {
        let logo = get_logo("totally_unknown_distro");
        assert!(!logo.lines.is_empty());
        assert_eq!(logo.color, WHITE);
    }

    #[test]
    fn test_colored_lines() {
        let logo = get_logo("fedora");
        let colored = logo.colored_lines();
        assert_eq!(colored.len(), logo.lines.len());
        assert!(colored[0].starts_with("\x1b[38;2;"));
    }

    #[test]
    fn test_all_expanded_distros_return_correct_logos() {
        assert_eq!(get_logo("Linux Mint").color, MINT_GREEN);
        assert_eq!(get_logo("Manjaro Linux").color, MANJARO_GREEN);
        assert_eq!(get_logo("Pop!_OS").color, POP_TEAL);
        assert_eq!(get_logo("Alpine Linux").color, ALPINE_BLUE);
        assert_eq!(get_logo("Void Linux").color, VOID_GREEN);
        assert_eq!(get_logo("Gentoo").color, GENTOO_PURPLE);
        assert_eq!(get_logo("EndeavourOS").color, ENDEAVOUR_PURPLE);
        assert_eq!(get_logo("Kali GNU/Linux").color, KALI_BLUE);
        assert_eq!(get_logo("SteamOS").color, STEAM_BLUE);
        assert_eq!(get_logo("Artix Linux").color, ARTIX_CYAN);
        assert_eq!(get_logo("Red Hat Enterprise Linux").color, REDHAT_RED);
        assert_eq!(get_logo("Rocky Linux").color, SUSE_GREEN);
        assert_eq!(get_logo("CentOS Stream").color, REDHAT_RED);
        assert_eq!(get_logo("AlmaLinux").color, DEBIAN_RED);
        assert_eq!(get_logo("Zorin OS").color, ZORIN_BLUE);
        assert_eq!(get_logo("Raspbian").color, RASPBIAN_RED);
        assert_eq!(get_logo("CachyOS").color, CACHYOS_CYAN);
        assert_eq!(get_logo("cachyos_small").color, CACHYOS_CYAN);
        assert_eq!(get_logo("FreeBSD").color, FREEBSD_RED);
        assert_eq!(get_logo("freebsd_small").color, FREEBSD_RED);
        assert_eq!(get_logo("OpenBSD").color, OPENBSD_YELLOW);
        assert_eq!(get_logo("NetBSD").color, NETBSD_ORANGE);
        assert_eq!(get_logo("Garuda Linux").color, GARUDA_ORANGE);
        assert_eq!(get_logo("Nobara Linux").color, NOBARA_WHITE);
        assert_eq!(get_logo("Archcraft").color, ARCHCRAFT_TEAL);
        assert_eq!(get_logo("macOS").color, MACOS_GREEN);
        assert_eq!(get_logo("Windows").color, WINDOWS_BLUE);
        assert_eq!(get_logo("nixos_small").color, NIXOS_CYAN);
        assert_eq!(get_logo("arch_small").color, ARCH_CYAN);
        assert_eq!(get_logo("fedora_small").color, FEDORA_BLUE);
        assert_eq!(get_logo("ubuntu_small").color, UBUNTU_ORANGE);
        assert_eq!(get_logo("debian_small").color, DEBIAN_RED);
    }
}
