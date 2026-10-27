# RustFetch — Proje Dosya ve Bağımlılık Haritası (`PROJECT_MAP.md`)

> **KURAL**: Herhangi bir kod değişikliği veya inceleme yapmadan önce ilgili bileşenin yerini, sorumluluğunu ve bağımlılıklarını buradan kontrol et. Dosya/fonksiyon/modül eklendiğinde, silindiğinde veya taşındığında bu haritayı **mutlaka güncelle**.

---

## 1. Workspace Mimarisi ve Crate Bağımlılık Grafiği

```
rustfetch (cli/)  ──┬──► rustfetch-core (core/)
                    └──► rustfetch-render (render/)
```

- **`rustfetch-core` (`core/`)**: Sistem bilgisi toplama (`Module` trait + 28 modül, GitHub güncelleme kontrolü `update` dahil), yapılandırma (`Config`, TOML + Fastfetch JSON/JSONC içe aktarma), disk önbellekleme (`Cache`), ASCII logo ayrıştırma ve Neofetch renk kodları. `render` veya `cli`'ye **bağımlı değildir**.
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
| `install.sh` / `Makefile` / `flake.nix` / `packaging/` | Kurulum (Bazzite `rpm-ostree` + Homebrew `brew` + `rustup` dahil, güncelleme sırasında mevcut `config.toml`'u korur), paketleme (AUR, deb, rpm, Nix) ve tamamlama/man dosyalarının kurulumu. | `cli/` ikilileri ve `completions/`, `man/`. |
| `.github/workflows/release.yml` & `scripts/build-packages.sh` & `scripts/release-notes.sh` | Otomatik GitHub Releases iş akışı: `unknown` içermeyen temiz platform arşivleri (`linux-gnu`, `linux-musl`, `nixos`), yerel dağıtım paketleri (`.pkg.tar.zst` Arch, `.rpm` Fedora/Bazzite, `.deb` Debian/Ubuntu), kedi cinsi sürüm kod adları ve otomatik `CHANGELOG.md` sürüm notu üretimi. | `Cargo.toml`, `CHANGELOG.md`, `packaging/`. |
| `CHANGELOG.md` | Semantik sürüm geçmişi ve Kedi Cinsi Kod Adları (`Turkish Van`, `Turkish Angora`, `Bengal`, `Maine Coon`, `Abyssinian`, vb.) kaydı. | `scripts/release-notes.sh`, `core/src/modules/update.rs`. |
| `images/` | Test ve varsayılan görsel varlıkları (örn. `images/arch.png`). | `render/src/image.rs` ve `cli/src/layout.rs` birim testleri. |

---

## 3. `rustfetch-core` (`core/`) Detaylı Harita

### `core/src/lib.rs`
- **Görevi**: Crate kök modülü; alt modülleri (`ascii`, `cache`, `config`, `modules`) dışa aktarır ve `Info`, `Module`, `detect_all` yapılarını tanımlar.
- **Dışa Aktarılanlar (`pub use`)**:
  - `ascii::apply_neofetch_colors`
  - `config::{default_icon_for_module, distro_icon, BorderCharsConfig, BorderImageConfig, Config}`
  - `modules::update::{codename_for_version, format_version_with_codename, CURRENT_VERSION}`
  - `modules::{all_modules, module_by_name}`
  - `Info`, `Module`, `detect_all`
- **Kim Kullanır**: `cli/src/lib.rs`, `cli/src/layout.rs`, `cli/src/logos.rs`.

### `core/src/config.rs`
- **Görevi**: Tüm TOML yapılandırma şeması, Fastfetch JSON/JSONC içe aktarıcı (`import_from_fastfetch`, `strip_jsonc`), preset profiller (`Config::from_preset`), varsayılan modül ikonları (`default_icon_for_module`, `distro_icon` — Fedora/Bazzite/Nobara dahil) ve kenarlık karakter setleri (`BorderCharsConfig::from_style`).
- **Önemli Yapılar ve Fonksiyonlar**:
  - `Config`: `{ general: GeneralConfig, modules: Vec<ModuleConfig> }`
    - `Config::load()` -> `~/.config/rustfetch/config.toml` (veya `XDG_CONFIG_HOME`) okur; mevcut yapılandırmanın üzerine asla yazmaz (`write_if_not_exists`); yoksa Fastfetch yapılandırmasını (`find_fastfetch_config`) içe aktarır veya `Config::to_default_toml()` yazar.
    - `Config::write_if_not_exists(path, content)` & `Config::backup_if_exists(path)` -> Kullanıcı yapılandırmasının güncelleme veya yeniden üretim (`--generate-config`, `--import-fastfetch`) sırasında silinmesini/bozulmasını önler (`config.toml.bak` yedeklemesi).
    - `Config::load_from(path)` & `Config::from_toml(content)` -> Dosya uzantısı `.json`/`.jsonc` ise `import_from_fastfetch`, değilse `from_toml` (hata durumunda `salvage_partial_toml` ile geçerli tüm kullanıcı ayarlarını kurtarır, ardından JSONC fallback) çalıştırır.
    - `Config::from_preset(name)` -> `"minimal"`, `"clean"`, `"dots"`, `"card"`/`"modern-card"`, `"brackets"`, `"neofetch"`, `"retro"`, `"modern"`, `"compact"`, `"default"` profillerini `modules_from(&[...])` yardımcısıyla üretir.
    - `Config::to_default_toml()` -> Açıklamalı varsayılan `config.toml` metni üretir (`--print-default-config`, `--dump-config` ve `--generate-config` tarafından kullanılır).
  - `GeneralConfig`: Ayırıcı, boşluk, ortalama, ikonlar, `check_updates` (GitHub güncelleme bildirimi aç/kapat), `border` (`[general.border]` veya üst seviye `[border]` tablosu), `BorderImageConfig` (`border_image` + `border_images` -> `active_border_images()`), `ColorsConfig`, `LogoConfig`, tema ve gradyan ayarları.
  - `BorderCharsConfig::from_style(style)` -> `"rounded"`, `"single"`, `"double"`, `"thick"`, `"ascii"`, `"dots"`, `"block"`, `"brackets"` stillerini eşler.
  - `LogoConfig`, `ThreeDConfig` (bool veya tablo olarak deserialize edilebilir), `AnimationConfig`, `ModuleConfig::new(name)`.
- **Kim Kullanır**: `cli/src/lib.rs`, `cli/src/layout.rs`.

### `core/src/ascii.rs`
- **Görevi**: ASCII logo şablonlarını (`$1`..`$6` renk yer tutucuları) ve çok kareli ASCII animasyon dosyalarını ayrıştırma.
- **Önemli Yapılar ve Fonksiyonlar**:
  - `apply_neofetch_colors(raw, colors)` -> `$1`..`$6` ve `${c1}`..`${c6}` etiketlerini ANSI renk kodlarıyla değiştirir.
  - `load_ascii_lines(path)` -> Dosyadan ASCII satırlarını yükler (`cli::run_3d` kullanır).
  - `load_ascii_frames(path)` -> `===FRAME===` ayırıcılı tek dosyadan veya dizindeki doğal sıralı (`natural_sort_key`) dosyalardan animasyon karelerini yükler (`cli::run_ascii_animation` kullanır).
- **Kim Kullanır**: `cli/src/logos.rs`, `cli/src/lib.rs`.

### `core/src/cache.rs`
- **Görevi**: Yavaş veya ağ/donanım tabanlı modül sonuçlarını (`packages`, `gpu`, `update`) `$XDG_CACHE_HOME/rustfetch/<module_name>` altında TTL ve atomik yazma-yeniden adlandırma (`write`) ile önbellekleme.
- **Önemli Yapılar ve Fonksiyonlar**:
  - `Cache::new(module_name)`, `Cache::read(ttl)`, `Cache::write(value)`.
- **Kim Kullanır**: `core/src/modules/packages.rs`, `core/src/modules/gpu.rs`, `core/src/modules/update.rs`.

### `core/src/modules/` (32 Sistem Modülü)
- **`core/src/modules/mod.rs`**:
  - `pub trait Module: Send + Sync { fn name(&self) -> &'static str; fn detect(&self) -> Option<Info>; }` (`core/src/lib.rs` içinde tanımlı).
  - `module_by_name(name)` & `module_from_config(mc)` -> Modül adını (`"os"`, `"os_age"`, `"power_profile"`, `"gpu_driver"`, `"net_io"`, `"disk:/home"`, `"update"`, vb.) ilgili `Box<dyn Module>` örneğine çözümler.
  - `all_modules()` -> Varsayılan 26 sistem modülünü `module_by_name` üzerinden üretir.
  - `detect_all(modules)` -> Tüm modülleri `std::thread::scope` ile paralel thread'lerde çalıştırır.
- **Alt Modüller (`core/src/modules/*.rs`)**:
  - `os.rs` (`Os`): `/etc/os-release` + mimari (`uname -m`).
  - `os_age.rs` (`OsAge`): `libc::statx` (`STATX_BTIME`) ile kök dosya sisteminin oluşturulma zamanından sistem kurulum yaşını hesaplar.
  - `host.rs` (`Host`): `/sys/devices/virtual/dmi/id/{sys_vendor,product_name,product_version}` + cihaz ağacı fallback.
  - `board.rs` (`Board`): `/sys/class/dmi/id/board_{vendor,name,version}`.
  - `kernel.rs` (`Kernel`): `/proc/sys/kernel/osrelease`.
  - `uptime.rs` (`Uptime`): `/proc/uptime`.
  - `packages.rs` (`PackagesModule`): `pacman`, `dpkg`, `rpm`, `xbps`, `apk`, `emerge`, `nix`, `flatpak`, `snap`, `brew` paket sayılarını toplar.
  - `shell.rs` (`Shell`): `$SHELL` + sürüm bayrağı.
  - `desktop.rs` (`DesktopModule`): `$XDG_CURRENT_DESKTOP` / `$DESKTOP_SESSION` + Wayland/X11 protokolü.
  - `font.rs` (`FontModule`): GTK3/GTK4/gsettings/Xresources yazı tipi tespiti.
  - `terminal.rs` (`Terminal`): `$TERM_PROGRAM`, `$KITTY_WINDOW_ID`, `/proc/<ppid>/comm` yürüyüşü ve `$TERM`.
  - `cpu.rs` (`Cpu`): `/proc/cpuinfo` model adı, çekirdek sayısı ve `/sys/devices/system/cpu` frekansı.
  - `gpu.rs` (`GpuModule`): `/sys/bus/pci/devices` + `/usr/share/hwdata/pci.ids` üzerinden harici/dahili GPU tespiti.
  - `gpu_driver.rs` (`GpuDriver`): `/proc/driver/nvidia/version` ve `/sys/class/drm/card*/device/driver` üzerinden aktif GPU çekirdek sürücüsü.
  - `memory.rs` (`Memory`): `/proc/meminfo` + ZFS ARC.
  - `swap.rs` (`Swap`): `/proc/meminfo` (`SwapTotal`, `SwapFree`).
  - `disk.rs` (`Disk`): `libc::statvfs` ile `/` (composefs/Bazzite için `/sysroot` & `/var` fallback) veya özel bağlama noktası (`Disk::with_mount`, `path = "/home"`) kullanımı.
  - `battery.rs` (`BatteryModule`): `/sys/class/power_supply/BAT*` kapasite ve durum.
  - `power_profile.rs` (`PowerProfile`): `/sys/firmware/acpi/platform_profile` ve CPU `scaling_governor` / `energy_performance_preference` durumu.
  - `brightness.rs` (`BrightnessModule`): `/sys/class/backlight/*` parlaklık yüzdesi.
  - `cpu_usage.rs` (`CpuUsage`): `/proc/stat` iki örneklemeli anlık CPU kullanımı.
  - `temp.rs` (`Temp`): `/sys/class/thermal` ve `/sys/class/hwmon` CPU sıcaklığı.
  - `sound.rs` (`Sound`): `wpctl` / `pactl` / `amixer` ses seviyesi.
  - `media.rs` (`Media`): `playerctl` medya bilgisi.
  - `display.rs` (`Display`): `/sys/class/drm` EDID + çözünürlük/yenileme hızı.
  - `wifi.rs` (`Wifi`): `/sys/class/net` kablosuz arayüz + `iw` / `nmcli` SSID, frekans bandı ve sinyal yüzdesi.
  - `net_io.rs` (`NetIo`): `/sys/class/net/*/statistics/{rx_bytes,tx_bytes}` üzerinden aktif ağ arayüzünün toplam indirilen/yüklenen veri miktarı.
  - `bluetooth.rs` (`BluetoothModule`): `bluetoothctl` bağlı cihazlar ve pil yüzdesi.
  - `local_ip.rs` (`LocalIpModule`): UDP soket (`8.8.8.8:80`) üzerinden yerel IPv4 adresi.
  - `locale.rs` (`Locale`): `$LC_ALL` / `$LANG` yerel ayar.
  - `processes.rs` (`Processes`): `/proc/loadavg` veya `/proc` sayısal dizin sayımı.
  - `init.rs` (`Init`): `/proc/1/comm` (`systemd`, `openrc`, `runit`, `s6`, `dinit`) + sürüm.
  - `container.rs` (`Container`): Docker/Podman/LXC/WSL/systemd-nspawn ve QEMU/KVM/VMware/VirtualBox tespiti.
  - `update.rs` (`Update`): GitHub deposundan (`MehmetCanWT/rustfetch`) 6 saatlik TTL önbelleklemesiyle (`Cache::new("update")`) en yeni sürümü ve GitHub Release başlığındaki kedi cinsi kod adını (`parse_github_release_info`, `extract_codename_from_release_title`, fallback `codename_for_version`) kontrol eder; güncelleme varsa modüllerin en altında `Update available (vX.Y.Z -> vA.B.C "Codename")` satırı döndürür (`[general] check_updates = false` veya `--no-update-check` ile kapatılabilir).
  - `custom.rs` (`Custom`) & `BreakLine`: Özel metin ve boşluk/ayırıcı satırları.

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
  - `image_display_dimensions(path, target_cols, max_rows)`: Görsel başlığını okuyarak gerçek `(cols, rows)` yerleşim boyutlarını ön-hesaplar.
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
- **Görevi**: İkili giriş noktaları (`rustfetch` ve `rfetch`); doğrudan `rustfetch::run()` çağırır.

### `cli/src/lib.rs` & `cli/src/tools.rs`
- **Görevi**: CLI argüman ayrıştırma (`Cli`), yapılandırma yükleme ve birleştirme, çalışma modlarını yönetme, keşif/teşhis/güncelleme/TUI araçları (`tools.rs`) ve bilgi satırlarını toplama.
- **Önemli Yapılar ve Fonksiyonlar**:
  - `Cli` (`clap::Parser`): Tüm komut satırı bayrakları (`--config`, `--preset`, `--theme`, `--logo`, `--image`, `--ascii`, `--ascii-anim`, `--3d`, `--live`, `--json`, `--benchmark`, `--no-update-check`, `--list-modules`, `--list-themes`, `--list-presets`, `--list-logos`, `--clean-cache`, `--doctor`, `--self-update`, `--setup`/`--tui`, `--install-hook`, `--uninstall-hook`, vb.).
  - `pub fn run()`: Yardımcı bayrakları (`tools::run_*`, `--print-default-config`, `--dump-config`, `--generate-config`, `--init`, `--install-hook`, `--uninstall-hook`, `--import-fastfetch`, `--completions`) işler, `Config` yükler, modülleri `module_from_config` ile eşler (`check_updates` açıksa en alta `update` modülünü ekler), TTY güvenliğini kontrol eder ve uygun çalışma modunu başlatır.
  - `tools.rs`: `run_list_modules`, `run_list_themes`, `run_list_presets`, `run_list_logos`, `run_clean_cache`, `run_doctor`, `run_self_update` (mevcut `config.toml`'u koruyarak GitHub Releases üzerinden günceller), `run_setup_tui` (canlı önizlemeli interaktif yapılandırma sihirbazı).
  - `run_once(...)`: Tek seferlik standart çıktı; özel ASCII (`shellexpand::tilde` ile), dağıtım/kedi logosu veya görsel (Kitty/Sixel/Halfblock) + bilgi kartını `layout::render` / `layout::render_with_image` ile basar.
  - `TermGuard`: `run_3d` ve `run_ascii_animation` için ortak RAII terminal koruyucusu (`tcgetattr`/`tcsetattr` raw mode, `SIGINT`/`SIGTERM` sinyalleri ve imleç gizleme/gösterme).
  - `compose_animated_frame(...)`: `run_3d` ve `run_ascii_animation` için ortak dikey (stacked) ve yan yana (side-by-side) kare tamponu birleştiricisi.
  - `run_3d(...)`: Etkileşimli/animasyonlu 3B döndürme modu; arka plan thread'inde kullanıcının yapılandırdığı modülleri saniyede bir günceller, herhangi bir tuş/komut/sinyal geldiğinde 3B sahneyi temizleyip aynı dağıtımın 2B statik ASCII logosuna düşer (fallback).
  - `run_ascii_animation(...)`: Çok kareli ASCII animasyon oynatıcısı (`shellexpand::tilde` destekli).
  - `run_live(...)`: `LiveGuard` RAII koruyucusu ile alternatif ekranda (`EnterAlternateScreen`) saniyede bir yenilenen, mini sparkline grafikli (`render_sparkline`) ve anlık kısayollu (`t` tema, `b` kenarlık, `q` çıkış) canlı sistem izleyicisi.
  - `gather_info_lines(...)`: `detect_all` sonuçlarını, OSC 8 tıklanabilir güncelleme bağlantısını, özel format (`{value}`), dinamik eşik renkli ilerleme çubuğu (`format_bar`), ikon ve tema/gradyan renkleriyle `Vec<layout::InfoLine>` listesine dönüştürür.

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
- **Görevi**: 25+ Linux/BSD/macOS/Windows dağıtımı (Bazzite dahil) için gömülü büyük ve küçük (`_small`) ASCII logo şablonları.
- **Fonksiyonlar**:
  - `get_logo(distro)` -> Büyük `AsciiLogo` döndürür.
  - `get_small_logo(distro)` -> Küçük `AsciiLogo` döndürür (terminal dar olduğunda `resolve_distro_logo` tarafından seçilir).
