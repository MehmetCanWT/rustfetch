//! ASCII distro logos for terminal display.

/// ANSI reset code.
pub const RESET: &str = "\x1b[0m";

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
    /// Number of lines in the logo.
    #[allow(dead_code)]
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    /// Return the logo lines pre-colored with ANSI codes.
    pub fn colored_lines(&self) -> Vec<String> {
        self.lines
            .iter()
            .map(|line| {
                let mut out = String::with_capacity(line.len() + 32);
                let mut chars = line.chars().peekable();
                let mut has_color = false;
                while let Some(c) = chars.next() {
                    if c == '$' {
                        if let Some(&next) = chars.peek() {
                            if let Some(digit) = next.to_digit(10) {
                                chars.next();
                                let idx = (digit as usize).saturating_sub(1);
                                let col = self.colors.get(idx).copied().unwrap_or(self.color);
                                out.push_str(col);
                                has_color = true;
                                continue;
                            }
                        }
                    }
                    out.push(c);
                }
                if has_color {
                    out.push_str(RESET);
                    out
                } else {
                    format!("{}{}{}", self.color, line, RESET)
                }
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
        color: "\x1b[34m",
        colors: vec!["\x1b[34m", "\x1b[37m"],
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
        color: "\x1b[36m",
        colors: vec!["\x1b[36m", "\x1b[96m"],
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
        color: "\x1b[31m",
        colors: vec!["\x1b[31m", "\x1b[37m"],
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
        color: "\x1b[33m",
        colors: vec!["\x1b[33m", "\x1b[37m"],
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
        color: "\x1b[32m",
        colors: vec!["\x1b[32m", "\x1b[37m"],
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
        color: "\x1b[32m",
        colors: vec!["\x1b[32m"],
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
        color: "\x1b[36m",
        colors: vec!["\x1b[36m", "\x1b[33m"],
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
        color: "\x1b[34m",
        colors: vec!["\x1b[34m", "\x1b[37m"],
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
        color: "\x1b[32m",
        colors: vec!["\x1b[32m", "\x1b[37m"],
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
        color: "\x1b[35m",
        colors: vec!["\x1b[35m", "\x1b[37m"],
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
        color: "\x1b[36m",
        colors: vec![
            "\x1b[34m", "\x1b[36m", "\x1b[94m", "\x1b[96m", "\x1b[34m", "\x1b[36m",
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
        color: "\x1b[32m",
        colors: vec!["\x1b[32m", "\x1b[37m"],
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
        color: "\x1b[35m",
        colors: vec!["\x1b[35m", "\x1b[31m", "\x1b[34m"],
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
        color: "\x1b[34m",
        colors: vec!["\x1b[34m", "\x1b[37m"],
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
        color: "\x1b[36m",
        colors: vec!["\x1b[36m", "\x1b[37m"],
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
        color: "\x1b[36m",
        colors: vec!["\x1b[36m", "\x1b[37m"],
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
        color: "\x1b[31m",
        colors: vec!["\x1b[31m", "\x1b[37m"],
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
        color: "\x1b[31m",
        colors: vec!["\x1b[31m", "\x1b[32m"],
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
        color: "\x1b[31m",
        colors: vec!["\x1b[31m", "\x1b[33m", "\x1b[32m", "\x1b[36m", "\x1b[34m"],
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
        color: "\x1b[31m",
        colors: vec!["\x1b[31m", "\x1b[32m", "\x1b[34m", "\x1b[35m", "\x1b[37m"],
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
        color: "\x1b[36m",
        colors: vec!["\x1b[36m", "\x1b[37m"],
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
        color: "\x1b[31m",
        colors: vec!["\x1b[32m", "\x1b[31m"],
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
        color: "\x1b[37m",
        colors: vec!["\x1b[37m", "\x1b[90m", "\x1b[33m"],
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
        assert_eq!(logo.color, "\x1b[34m");
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
        assert_eq!(logo.color, "\x1b[36m");
    }

    #[test]
    fn test_get_logo_unknown_returns_linux() {
        let logo = get_logo("totally_unknown_distro");
        assert!(!logo.lines.is_empty());
        assert_eq!(logo.color, "\x1b[37m");
    }

    #[test]
    fn test_colored_lines() {
        let logo = get_logo("fedora");
        let colored = logo.colored_lines();
        assert_eq!(colored.len(), logo.lines.len());
        assert!(colored[0].starts_with("\x1b["));
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
        assert_eq!(get_logo("Zorin OS").color, "\x1b[36m");
        assert_eq!(get_logo("Raspbian").color, "\x1b[31m");
    }
}
