# Custom ASCII Art & Color Placeholders

RustFetch allows using arbitrary custom ASCII art files in place of built-in distribution logos, with full support for ANSI TrueColor escape sequences, Neofetch multi-color placeholder variables, and instant extrusion into rotating 3D relief models.

---

## 🎨 Loading Custom ASCII Art

You can pass any plain text or ANSI-colored ASCII art file (`.txt`, `.ascii`) using the CLI or configuration file:

```bash
# Load a custom ASCII art logo
rustfetch --ascii ~/.config/rustfetch/logos/custom_dragon.txt

# Extrude your custom ASCII drawing into an interactive 3D model
rustfetch --ascii ~/.config/rustfetch/logos/custom_dragon.txt --3d --speed 1.4
```

In `~/.config/rustfetch/config.toml`:
```toml
[general.logo]
enabled = true
ascii_path = "~/.config/rustfetch/logos/custom_dragon.txt"
```

RustFetch automatically measures the maximum visual column width of your drawing (stripping ANSI codes to calculate actual character cells) and aligns the telemetry information neatly beside it.

---

## 🌈 Neofetch Color Variable Substitution (`$1..$6`)

Many community ASCII logos (including official Neofetch and Fastfetch distro files) use variable placeholders for colors rather than hardcoded ANSI codes. 

RustFetch natively recognizes and replaces both `$N` and `${cN}` color tokens:

| Token | Long Token | Default Meaning |
|---|---|---|
| `$1` | `${c1}` | Primary accent color |
| `$2` | `${c2}` | Secondary accent color |
| `$3` | `${c3}` | Tertiary color |
| `$4` | `${c4}` | Quaternary color |
| `$5` | `${c5}` | Highlight color |
| `$6` | `${c6}` | Shadow / background color |

### Example ASCII File (`arch_custom.txt`):
```text
${c1}      /\
${c1}     /  \
${c1}    /\   \
${c2}   /      \
${c2}  /   ,,   \
${c2} /   |  |  -\
${c2}/_-''    ''-_\
```

### Configuring Colors in `config.toml`:
```toml
[general.logo]
ascii_path = "arch_custom.txt"
colors = ["#1793d1", "#ffffff"]   # $1 = Arch Cyan, $2 = Pure White
```

RustFetch substitutes `$1` with `\x1b[38;2;23;147;209m` and `$2` with `\x1b[38;2;255;255;255m`.

---

## 🧊 Converting Custom 2D ASCII into Interactive 3D

Any custom ASCII text file can be extruded into a rotating 3D mesh model simply by adding `--3d`:

```bash
rustfetch --ascii my_art.txt --3d --shading-mode blocks --depth 1.5
```

The 3D engine analyzes the ink density of every character in your file (e.g. `' '` = empty, `'.'` = low, `'#'` = medium, `'█'` = full height) and constructs a continuous 3D relief mesh with surface normal lighting.

---

## ⚡ Built-in Distro Logo Overrides

If you prefer to display a different Linux distribution logo without loading an external file, use `--logo <NAME>`:

```bash
rustfetch --logo arch
rustfetch --logo gentoo
rustfetch --logo fedora
rustfetch --logo debian
rustfetch --logo nixos
rustfetch --logo void
```

Available distro logos: `arch`, `fedora`, `ubuntu`, `debian`, `gentoo`, `nixos`, `void`, `alpine`, `opensuse`, `manjaro`, `mint`, `pop`, `endeavouros`, `kali`, `artix`, `rhel`, `freebsd`, `macos`, `windows`, `linux`.
