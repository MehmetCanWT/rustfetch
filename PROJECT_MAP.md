# RustFetch — Proje Dosya ve Bağımlılık Haritası (`PROJECT_MAP.md`)

> **KURAL**: Herhangi bir kod değişikliği veya inceleme yapmadan önce ilgili bileşenin yerini, sorumluluğunu ve bağımlılıklarını buradan kontrol et. Dosya/fonksiyon/modül eklendiğinde, silindiğinde veya taşındığında bu haritayı **mutlaka güncelle**.

---

## 1. Workspace Mimarisi ve Crate Bağımlılık Grafiği

```
rustfetch (cli/)  ──┬──► rustfetch-core (core/)
                    └──► rustfetch-render (render/)
```

- **`rustfetch-core` (`core/`)**: Sistem bilgisi toplama (`Module` trait + 27 modül), yapılandırma (`Config`, TOML + Fastfetch JSON/JSONC içe aktarma), disk önbellekleme (`Cache`), ASCII logo ayrıştırma ve Neofetch renk kodları. `render` veya `cli`'ye **bağımlı değildir**.
- **`rustfetch-render` (`render/`)**: Terminal protokol tespiti (Kitty, Sixel, hücre piksel boyutu), görsel yeniden boyutlandırma (`resize_rgba`, `fit_dimensions`), Kitty/Sixel/Halfblock ve arka plan görsel işleme, 3B ASCII model ve ışıklandırma motoru (`three_d`), renk paleti/tema/baskın renk çıkarma (`color`). `core` veya `cli`'ye **bağımlı değildir**.
- **`rustfetch` (`cli/`)**: Komut satırı argümanları (`Cli`), çalışma modları (`run_once`, `run_3d`, `run_ascii_animation`, `run_live`, `run_json`, `run_benchmark`), yerleşim/kutu/kenarlık/palet/metin sarma motoru (`layout.rs`), gömülü dağıtım logoları (`logos.rs`) ve ikili giriş noktaları (`main.rs`, `bin/rfetch.rs`).

---

## 2. Kök Dizin ve Yardımcı Dosyalar

| Dosya | Ne İşe Yarar | Nereye Bağlı |
|---|---|---|
| `Cargo.toml` | Workspace tanımı (`core`, `render`, `cli`), ortak sürüm (`0.1.2`), ortak bağımlılıklar (`serde`, `toml`, `clap`, `crossterm`, `image`, `fast_image_resize`, `base64`, `miniz_oxide`, `unicode-width`, `libc`, `serde_json`, `shellexpand`) ve `release` profili (`lto = "fat"`, `codegen-units = 1`, `strip = "symbols"`). | Tüm crate'ler `workspace = true` ile buradan miras alır. |
| `config.toml` | Örnek/referans kullanıcı yapılandırma dosyası. | `core/src/config.rs` (`Config::to_default_toml`) ile uyumlu şema. |
| `scripts/test.sh` | Tam doğrulama betiği: `cargo fmt --check`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --release` ve `scripts/test-cli.sh`. | CI ve yerel doğrulama. |
| `scripts/test-cli.sh` | Derlenen `rustfetch` ve `rfetch` ikilileri üzerinde uçtan uca CLI bayrak/preset/JSON/config duman testleri. | `target/release/rustfetch`, `target/release/rfetch`. |
| `scripts/install.sh` / `Makefile` / `flake.nix` / `packaging/` | Kurulum, paketleme (AUR, deb, rpm, Nix) ve tamamlama/man dosyalarının kurulumu. | `cli/` ikilileri ve `completions/`, `man/`. |
| `images/` | Test ve varsayılan görsel varlıkları (örn. `images/arch.png`). | `render/src/image.rs` ve `cli/src/layout.rs` birim testleri. |

---

## 3. `rustfetch-core` (`core/`) Detaylı Harita

### `core/src/lib.rs`
- **Görevi**: Crate kök modülü; alt modülleri (`ascii`, `cache`, `config`, `modules`) dışa aktarır.
- **Dışa Aktarılanlar (`pub use`)**:
  - `ascii::{apply_neofetch_colors, parse_ascii_logo, AsciiLogo}`
  - `cache::Cache`
  - `config::{default_icon_for_module, distro_icon, AnimationConfig, BorderCharsConfig, BorderImageConfig, ColorsConfig, Config, GeneralConfig, LogoConfig, ModuleConfig, ThreeDConfig}`
  - `modules::{all_modules, detect_all, detect_all_cached, module_by_name, Module}`
- **Kim Kullanır**: `cli/src/lib.rs`, `cli/src/layout.rs`, `cli/src/logos.rs`.

### `core/src/config.rs`
- **Görevi**: Tüm TOML yapılandırma şeması, Fastfetch JSON/JSONC içe aktarıcı (`import_from_fastfetch`, `strip_jsonc`), preset profiller (`Config::from_preset`), varsayılan modül ikonları (`default_icon_for_module`, `distro_icon`) ve kenarlık karakter setleri (`BorderCharsConfig::from_style`).
- **Önemli Yapılar ve Fonksiyonlar**:
  - `Config`: `{ general: GeneralConfig, modules: Vec<ModuleConfig> }`
    - `Config::load()` -> `~/.config/rustfetch/config.toml` (veya `XDG_CONFIG_HOME`) okur; yoksa Fastfetch yapılandırmasını (`find_fastfetch_config`) içe aktarır veya `Config::default()` yazar.
    - `Config::load_from(path)` -> Dosya uzantısı `.json`/`.jsonc` ise `import_from_fastfetch`, değilse `from_toml` (başarısız olursa JSONC fallback) çalıştırır.
    - `Config::from_preset(name)` -> `"minimal"`, `"clean"`, `"dots"`, `"card"`/`"modern-card"`, `"brackets"`, `"neofetch"`, `"retro"`, `"modern"`, `"compact"`, `"default"` profillerini `modules_from(&[...])` yardımcısıyla üretir.
    - `Config::to_default_toml()` -> Açıklamalı varsayılan `config.toml` metni üretir (`--gen-config` ve `--init` tarafından kullanılır).
  - `GeneralConfig`: Ayırıcı, boşluk, ortalama, ikonlar, `BorderConfig` (`[general.border]` veya üst seviye `[border]` tablosu), `BorderImageConfig` (`border_image` + `border_images` -> `active_border_images()`), `ColorsConfig`, `LogoConfig`, tema ve gradyan ayarları.
  - `BorderCharsConfig::from_style(style)` -> `"rounded"`, `"single"`, `"double"`, `"thick"`, `"ascii"`, `"dots"`, `"blocks"`, `"brackets"` stillerini eşler.
  - `LogoConfig`, `ThreeDConfig` (bool veya tablo olarak deserialize edilebilir), `AnimationConfig`, `ModuleConfig::new(name)`.
- **Kim Kullanır**: `cli/src/lib.rs`, `cli/src/layout.rs`.

### `core/src/ascii.rs`
- **Görevi**: ASCII logo şablonlarını (`$1`..`$9` renk yer tutucuları) ve çok kareli ASCII animasyon dosyalarını ayrıştırma.
- **Önemli Yapılar ve Fonksiyonlar**:
  - `AsciiLogo`: `{ lines: &'static [&'static str], color: &'static str, width: usize }`
    - `colored_lines()` / `colored_lines_with_custom(primary_override, custom_palette)`
  - `parse_ascii_logo(raw, color)`
  - `apply_neofetch_colors(raw, colors)` -> `$1`..`$9` ve `${c1}`..`${c9}` etiketlerini ANSI renk kodlarıyla değiştirir.
  - `load_ascii_lines(path)` -> Dosyadan veya dizinden ASCII satırlarını yükler (`cli::run_3d` kullanır).
  - `load_ascii_frames(path)` -> `===FRAME===` ayırıcılı tek dosyadan veya dizindeki sıralı dosyalardan animasyon karelerini yükler (`cli::run_ascii_animation` kullanır).
- **Kim Kullanır**: `cli/src/logos.rs`, `cli/src/lib.rs`.

### `core/src/cache.rs`
- **Görevi**: Değişmeyen donanım bilgilerini (örn. `cpu`, `gpu`, `host`, `board`, `os`, `kernel`) `~/.cache/rustfetch/cache.json` altında önyükleme kimliği (`boot_id`) ve TTL (300 sn) ile önbellekleme.
- **Önemli Yapılar ve Fonksiyonlar**:
  - `Cache::load()`, `Cache::save()`, `Cache::get(key)`, `Cache::set(key, val)`.
- **Kim Kullanır**: `core/src/modules/mod.rs` (`detect_all_cached`).

### `core/src/modules/` (27 Sistem Modülü)
- **`core/src/modules/mod.rs`**:
  - `pub trait Module: Send + Sync { fn name(&self) -> &str; fn label(&self) -> &str; fn detect(&self) -> Option<String>; fn is_slow(&self) -> bool; fn usage_percentage(&self) -> Option<u8>; }`
  - `module_by_name(name)` -> Modül adını (`"os"`, `"cpu"`, `"break"`, `"divider"`, `"custom"`, vb.) ilgili `Box<dyn Module>` örneğine çözümler.
  - `all_modules()` -> Varsayılan 26 modül adını `module_by_name` üzerinden üretir.
  - `detect_all_cached(modules, use_cache)` -> Statik modülleri (`os`, `host`, `board`, `kernel`, `cpu`, `gpu`) `Cache` üzerinden, dinamik/hızlı modülleri (`std::thread::scope`) paralel thread'lerde çalıştırır.
- **Alt Modüller (`core/src/modules/*.rs`)**:
  - `os.rs` (`OsModule`): `/etc/os-release` + mimari (`uname -m`).
  - `host.rs` (`HostModule`): `/sys/devices/virtual/dmi/id/{sys_vendor,product_name,product_version}` + cihaz ağacı fallback (`parse_host` `board.rs` tarafından da kullanılır).
  - `board.rs` (`BoardModule`): `/sys/class/dmi/id/board_{vendor,name,version}` (`/sys/devices/virtual/dmi/id/` fallback ile `host::parse_host` kullanır).
  - `kernel.rs` (`KernelModule`): `/proc/sys/kernel/osrelease`.
  - `uptime.rs` (`UptimeModule`): `/proc/uptime`.
  - `packages.rs` (`PackagesModule`, `is_slow = true`): `pacman`, `dpkg`, `rpm`, `xbps`, `apk`, `flatpak`, `snap` paket sayılarını toplar.
  - `shell.rs` (`ShellModule`): `$SHELL` + sürüm bayrağı.
  - `desktop.rs` (`DesktopModule`): `$XDG_CURRENT_DESKTOP` / `$DESKTOP_SESSION` + Wayland/X11 protokolü.
  - `font.rs` (`FontModule`): GTK3/GTK4/gsettings/Xresources yazı tipi tespiti.
  - `terminal.rs` (`TerminalModule`): `$TERM_PROGRAM`, `$KITTY_WINDOW_ID`, `/proc/<ppid>/comm` yürüyüşü ve `$TERM`.
  - `cpu.rs` (`CpuModule`): `/proc/cpuinfo` model adı, çekirdek sayısı ve `/sys/devices/system/cpu` frekansı.
  - `gpu.rs` (`GpuModule`): `/sys/bus/pci/devices` + `/usr/share/hwdata/pci.ids` üzerinden harici/dahili GPU tespiti.
  - `memory.rs` (`MemoryModule`): `/proc/meminfo` + ZFS ARC (`/proc/spl/kstat/zfs/arcstats` -> `read_zfs_arc_kb` + `parse_meminfo_with_arc`) + `usage_percentage`.
  - `swap.rs` (`SwapModule`): `/proc/meminfo` (`SwapTotal`, `SwapFree`) + `usage_percentage`.
  - `disk.rs` (`DiskModule`): `libc::statvfs("/")` + `/proc/mounts` dosya sistemi türü + `usage_percentage`.
  - `battery.rs` (`BatteryModule`): `/sys/class/power_supply/BAT*` kapasite ve durum + `usage_percentage`.
  - `brightness.rs` (`BrightnessModule`): `/sys/class/backlight/*` parlaklık yüzdesi + `usage_percentage`.
  - `cpu_usage.rs` (`CpuUsageModule`): `/proc/stat` iki örneklemeli anlık CPU kullanımı + `usage_percentage`.
  - `temp.rs` (`TempModule`): `/sys/class/thermal` ve `/sys/class/hwmon` CPU sıcaklığı.
  - `sound.rs` (`SoundModule`): `wpctl` / `pactl` / `amixer` ses seviyesi + `usage_percentage`.
  - `media.rs` (`MediaModule`): `playerctl` medya bilgisi.
  - `display.rs` (`DisplayModule`): `/sys/class/drm` EDID + çözünürlük/yenileme hızı (`xrandr`/`hyprctl`/`swaymsg` fallback).
  - `wifi.rs` (`WifiModule`): `/sys/class/net` kablosuz arayüz + `iw` / `nmcli` (`parse_nmcli_dev_wifi` kaçışlı `:` destekli) SSID, frekans bandı ve sinyal yüzdesi.
  - `bluetooth.rs` (`BluetoothModule`): `bluetoothctl` bağlı cihazlar ve pil yüzdesi (`parse_bluetoothctl_info_battery`).
  - `local_ip.rs` (`LocalIpModule`): UDP soket (`8.8.8.8:80`) üzerinden yerel IPv4 adresi.
  - `locale.rs` (`LocaleModule`): `$LC_ALL` / `$LANG` yerel ayar.
  - `processes.rs` (`ProcessesModule`): `/proc/loadavg` veya `/proc` sayısal dizin sayımı.
  - `init.rs` (`InitModule`): `/proc/1/comm` (`systemd`, `openrc`, `runit`, `s6`, `dinit`) + sürüm.
  - `container.rs` (`ContainerModule`): Docker/Podman/LXC/WSL/systemd-nspawn ve QEMU/KVM/VMware/VirtualBox tespiti.
  - `custom.rs` (`CustomModule`, `BreakModule`, `DividerModule`): Özel metin ve boşluk/ayırıcı satırları.

---

## 4. `rustfetch-render` (`render/`) Detaylı Harita

### `render/src/lib.rs`
- **Görevi**: Crate kök modülü; `color`, `image`, `kitty`, `sixel`, `terminal`, `three_d` modüllerini dışa aktarır.
- **Dışa Aktarılanlar**:
  - `color::{apply_gradient_to_text, extract_color_palette, extract_dominant_color, get_theme, interpolate_color, interpolate_gradient, parse_color, Theme}`
  - `image::{apply_image_background_to_lines, apply_image_background_to_lines_ex, fit_dimensions, render_background, render_background_ex, render_halfblock, render_halfblock_ex, BackgroundOptions}`
  - `kitty::{clear_kitty_images, print_kitty, print_kitty_ex}`
  - `sixel::{encode_sixel, render_sixel_ex}`
  - `terminal::{cell_size_or_default, get_cell_size, supports_kitty, supports_sixel}`
  - `render_image_ex(path, target_cols, max_rows)`
  - `pub mod sixel; pub mod three_d;`

### `render/src/terminal.rs`
- **Görevi**: Terminal yeteneklerini ve hücre piksel boyutlarını tespit etme (`OnceLock` ile önbellekli).
- **Fonksiyonlar**:
  - `supports_kitty()` -> `$KITTY_WINDOW_ID`, `$TERM`, `$TERM_PROGRAM`, `$WEZTERM_EXECUTABLE`, `$GHOSTTY_RESOURCES_DIR` kontrolü.
  - `supports_sixel()` -> `$TERM` / `$TERM_PROGRAM` (foot, wezterm, mlterm, xterm, contour, vb.) kontrolü.
  - `get_cell_size()` / `cell_size_or_default()` -> `libc::ioctl(TIOCGWINSZ)` ile piksel/hücre oranını hesaplar (varsayılan `(10, 20)`).

### `render/src/image.rs`
- **Görevi**: Görsel boyutlandırma, Kitty (statik + GIF animasyon), Unicode half-block (`▀`/`▄`) ve arka plan görsel işleme.
- **Fonksiyonlar**:
  - `pub(crate) fn resize_rgba(raw, src_w, src_h, dst_w, dst_h, filter) -> Option<Vec<u8>>`: `image.rs`, `sixel.rs` ve `color.rs` tarafından kullanılan merkezi SIMD (`fast_image_resize`) boyutlandırıcı.
  - `fit_dimensions(...)`: Hücre en-boy oranını dikkate alarak `(cols, rows)` hesaplar.
  - `render_ex(path, target_cols, max_rows)`: GIF ise `kitty::print_kitty_animation`, statik görsel ise `kitty::print_kitty` ile basar.
  - `render_halfblock` / `render_halfblock_ex`: 24-bit ANSI renkli `▀`/`▄` karakter dizileri üretir.
  - `render_background` / `render_background_ex`: Kitty `z_index` ve imleç kaydet/geri yükle (`\x1b7`/`\x1b8`) ile kart arkasına görsel basar.
  - `apply_image_background_to_lines` / `apply_image_background_to_lines_ex`: Halfblock/ANSI arka plan rengini metin satırlarının arkasına işler.

### `render/src/kitty.rs`
- **Görevi**: Kitty Graphics Protocol (`\x1b_G...\x1b\\`) çıktısı üretme.
- **Fonksiyonlar**:
  - `compress_and_encode(rgba_data)` (dahili): `miniz_oxide` zlib sıkıştırma + base64 kodlama.
  - `print_kitty` / `print_kitty_ex`: 4096 baytlık parçalar halinde RGBA görsel gönderir.
  - `print_kitty_animation`: GIF karelerini Kitty animasyon protokolüyle gönderir.
  - `clear_kitty_images`: `\x1b_Ga=d,d=a\x1b\\` ile ekrandaki Kitty görsellerini temizler.

### `render/src/sixel.rs`
- **Görevi**: DEC Sixel (`\x1bPq...\x1b\\`) 232 renkli niceleme (6x6x6 küp + 16 gri ton) ve RLE kodlama.
- **Fonksiyonlar**: `encode_sixel(rgba_data, width, height)`, `render_sixel_ex(path, target_cols, max_rows)` (`crate::image::resize_rgba` kullanır).

### `render/src/color.rs`
- **Görevi**: Görselden baskın renk (`extract_dominant_color`) ve renk paleti (`extract_color_palette`) çıkarma (`crate::image::resize_rgba` kullanır), hazır temalar (`get_theme`), renk ayrıştırma (`parse_color`: `#rrggbb`, `#rgb` ve isimler) ve metin gradyanı (`apply_gradient_to_text`, `interpolate_gradient`).

### `render/src/three_d.rs`
- **Görevi**: 2B ASCII logolardan 3B voksel/nokta bulutu modeli (`Model3D::from_ascii_lines`) oluşturma ve Z-tamponlu + Lambertian aydınlatmalı döndürme (`Renderer3D::render_frame`).
- **Yapılar/Fonksiyonlar**: `ShadingMode`, `Vertex`, `Model3D`, `Renderer3D`, `distro_3d_colors`, `parse_color_to_ansi`.

---

## 5. `rustfetch` (`cli/`) Detaylı Harita

### `cli/src/main.rs` & `cli/src/bin/rfetch.rs`
- **Görevi**: İkili giriş noktaları (`rustfetch` ve `rfetch`); doğrudan `rustfetch::main()` çağırır.

### `cli/src/lib.rs`
- **Görevi**: CLI argüman ayrıştırma (`Cli`), yapılandırma yükleme ve birleştirme, çalışma modlarını yönetme ve bilgi satırlarını toplama.
- **Önemli Yapılar ve Fonksiyonlar**:
  - `Cli` (`clap::Parser`): Tüm komut satırı bayrakları (`--config`, `--preset`, `--theme`, `--logo`, `--image`, `--ascii`, `--ascii-anim`, `--3d`, `--live`, `--json`, `--benchmark`, `--doctor`, vb.).
  - `pub fn main()`: Yardımcı bayrakları (`--gen-config`, `--init`, `--import-fastfetch`, `--list-modules`, `--list-logos`, `--list-themes`, `--list-presets`, `--doctor`, `--clear-cache`) işler, `Config` yükler, modülleri `module_by_name` ile eşler, TTY güvenliğini kontrol eder ve uygun çalışma modunu başlatır.
  - `run_once(...)`: Tek seferlik standart çıktı; özel ASCII (`shellexpand::tilde` ile), dağıtım logosu veya görsel (Kitty/Sixel/Halfblock) + bilgi kartını `layout::render` / `layout::render_with_image` ile basar.
  - `TermGuard`: `run_3d` ve `run_ascii_animation` için ortak RAII terminal koruyucusu (`tcgetattr`/`tcsetattr` raw mode, `SIGINT`/`SIGTERM` sinyalleri ve imleç gizleme/gösterme).
  - `compose_animated_frame(...)`: `run_3d` ve `run_ascii_animation` için ortak dikey (stacked) ve yan yana (side-by-side) kare tamponu birleştiricisi.
  - `run_3d(...)`: Etkileşimli/animasyonlu 3B döndürme modu; arka plan thread'inde kullanıcının yapılandırdığı modülleri (`module_by_name` ile) saniyede bir günceller.
  - `run_ascii_animation(...)`: Çok kareli ASCII animasyon oynatıcısı (`shellexpand::tilde` destekli).
  - `run_live(...)`: `LiveGuard` RAII koruyucusu ile alternatif ekranda (`EnterAlternateScreen`) saniyede bir yenilenen canlı sistem izleyicisi.
  - `gather_info_lines(...)`: `detect_all_cached` sonuçlarını, özel format (`{value}`, `{bar}`), ilerleme çubuğu (`make_bar`), ikon ve tema/gradyan renkleriyle `Vec<layout::InfoLine>` listesine dönüştürür.

### `cli/src/layout.rs`
- **Görevi**: ANSI kaçış dizisi farkındalıklı genişlik/kırpma/sarma, kutu çizimi, kenarlık görselleri ve terminal yerleşimi.
- **Önemli Yapılar ve Fonksiyonlar**:
  - `color_code(name)`: `rustfetch_render::parse_color` + parlak renk adlarını ANSI dizisine çevirir (`TERM=linux` konsolunda 16-renk `rgb_to_ansi` fallback yapar).
  - `terminal_size()` / `terminal_width()`: `$COLUMNS`/`$LINES`, `crossterm::terminal::size()` ve `libc::ioctl(TIOCGWINSZ)` ile terminal boyutunu döndürür.
  - `build_color_palette(symbol, block, custom_palette)`: Renk paleti satırlarını oluşturur.
  - `format_info_lines(...)`: `InfoLine` listesini hizalı `etiket : değer` satırlarına dönüştürür.
  - `wrap_in_box_bounded(...)` / `wrap_in_box_advanced(...)`: Başlık (`border_title`), yatay ayırıcılar (`---DIVIDER---`) ve kelime sarma (`wrap_ansi_line`) ile kutu kenarlıklarını çizer.
  - `build_right_lines(...)` / `build_right_lines_bounded(...)`: Başlık, bilgi satırları, palet, kutu kenarlığı ve kenarlık görsellerini (`compose_with_border_image`, `compose_image_columns`, `apply_all_backgrounds`) birleştirir.
  - `render(logo, info_lines, opts)` / `render_with_logo(...)` / `render_without_logo(...)` / `render_with_image(...)`: Sol/sağ/üst/alt logo konumlarına ve terminal genişliğine göre duyarlı çıktı üretir.
  - `strip_ansi(s)`, `strip_ansi_width(s)`, `truncate_ansi(s, max_cols)`.

### `cli/src/logos.rs`
- **Görevi**: 25+ Linux/BSD/macOS/Windows dağıtımı için gömülü büyük ve küçük (`_small`) ASCII logo şablonları.
- **Fonksiyonlar**:
  - `get_logo(distro)` -> Büyük `AsciiLogo` döndürür.
  - `get_small_logo(distro)` -> Küçük `AsciiLogo` döndürür (terminal dar olduğunda `resolve_distro_logo` tarafından seçilir).
