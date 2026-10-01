# 🎬 RustFetch ASCII & Animasyon Kılavuzu (ASCII & Animation Guide)

RustFetch, sadece dahili dağıtım logolarını veya statik görselleri değil, **kullanıcıların kendi hazırladığı ASCII çizimlerini** ve **kare kare (frame-by-frame) hareketli ASCII animasyonlarını** sıfır gecikmeyle terminalde oynatmayı destekler.

---

## 📑 İçindekiler
1. [Özel ASCII Çizimi Kullanımı (2D & 3D)](#1-özel-ascii-çizimi-kullanımı-2d--3d)
2. [Kare Kare ASCII Animasyonu Oluşturma](#2-kare-kare-ascii-animasyonu-oluşturma)
   - [Yöntem A: Tek Dosya Formatı (Ayraçlı)](#yöntem-a-tek-dosya-formatı-ayraçlı)
   - [Yöntem B: Klasör Formatı (Kare Dosyaları)](#yöntem-b-klasör-formatı-kare-dosyaları)
3. [Renklendirme ve ANSI Desteği](#3-renklendirme-ve-ansi-desteği)
4. [Akıllı Klavye Devri (`exit_on_key`)](#4-akıllı-klavye-devri-exit_on_key)
5. [Konfigürasyon Referansı (`config.toml`)](#5-konfigürasyon-referansı-configtoml)
6. [CLI Parametreleri](#6-cli-parametreleri)
7. [Hazır Örnekler](#7-hazır-örnekler)

---

## 1. Özel ASCII Çizimi Kullanımı (2D & 3D)

Kendi hazırladığınız veya internetten bulduğunuz herhangi bir ASCII sanatını bir `.txt` veya `.ascii` dosyasına kaydedip RustFetch'e verebilirsiniz.

### A. 2D Statik Gösterim
```bash
rustfetch --ascii /yol/benim_logom.txt
```
RustFetch bu çizimi okur, genişliğini otomatik hesaplar ve sistem telemetrinizin soluna hizalar.

### B. 3D Gerçek Zamanlı Döndürme
Kendi ASCII çiziminizi gerçek zamanlı 3 boyutlu bir kabartma modele dönüştürüp döndürmek için:
```bash
rustfetch --ascii /yol/benim_logom.txt --3d
```
RustFetch, çiziminizdeki her karakterin mürekkep yoğunluğunu (`char_weight_utf8`) analiz ederek derinlik haritası (heightmap) oluşturur ve çiziminizi uzayda 3 eksende döndürerek gölgelendirir.

---

## 2. Kare Kare ASCII Animasyonu Oluşturma

RustFetch'te hareketli bir ASCII animasyonu oynatmak için iki yöntem mevcuttur:

### Yöntem A: Tek Dosya Formatı (Ayraçlı)
Tüm animasyon karelerini tek bir metin dosyasında toplayabilirsiniz. Karelerin arasına şu ayraçlardan birini koymanız yeterlidir:
- `===FRAME===`
- `===`
- `---FRAME---`
- `---`
- `[frame]`
- veya VT terminal standartı Form Feed (`\x0c`)

**Örnek `animasyon.txt`**:
```text
  ( o.o )
   > ^ <
===FRAME===
  ( -.- )
   > ^ <
===FRAME===
  ( o.- )
   > ^ <
===FRAME===
  ( -.o )
   > ^ <
```

Çalıştırmak için:
```bash
rustfetch --ascii-anim animasyon.txt --fps 10
```

---

### Yöntem B: Klasör Formatı (Kare Dosyaları)
Animasyon karelerinizi ayrı ayrı dosyalarda tutmak isterseniz bir klasör oluşturup içine dosyaları koyun:
```text
animasyon_klasoru/
├── 01.txt
├── 02.txt
├── 03.txt
└── 04.txt
```

> [!TIP]
> **Doğal Numerik Sıralama (Natural Sort):** Dosya isimleriniz `frame_1.txt`, `frame_2.txt`, ..., `frame_10.txt` şeklinde olsa bile RustFetch akıllı numerik algoritması sayesinde 10'u 2'den sonraya koyar. Sıralama asla bozulmaz.

Çalıştırmak için klasör yolunu vermeniz yeterlidir:
```bash
rustfetch --ascii-anim animasyon_klasoru/ --fps 15
```

---

## 3. Renklendirme ve ANSI Desteği

RustFetch, ASCII karelerinizin içindeki tüm ANSI TrueColor (`\x1b[38;2;R;G;Bm`) ve standart terminal renk kaçış dizilerini (escape codes) destekler.
- Kitty veya modern terminallerin temaları renklerinizi bozmaz.
- Her satırda farklı renkler veya karakter bazında gradyanlar kullanabilirsiniz.

---

## 4. Akıllı Klavye Devri (`exit_on_key`)

Terminal başlangıcında (`.bashrc` / `.zshrc`) animasyon oynatırken yaşanan en büyük sorun kullanıcının komut yazmasının engellenmesidir.

RustFetch bu sorunu **POSIX stdin yoklaması (`ioctl(FIONREAD)`)** ile çözer:
- Animasyon terminalinizi açtığınızda arka planda akıcı şekilde oynamaya başlar.
- **Klavyede herhangi bir tuşa bastığınız (komut yazmaya başladığınız) milisaniyede animasyon durur.**
- Bastığınız tuşlar yutulmaz; doğrudan kabuğunuza (Bash/Zsh/Fish) akar.
- Ekranda son animasyon karesi ve telemetri şık bir şekilde kalır.

> Animasyonun sadece `Ctrl+C` veya `q` ile çıkmasını isterseniz `--hold` bayrağını ekleyebilirsiniz:
> ```bash
> rustfetch --ascii-anim animasyon.txt --hold
> ```

---

## 5. Konfigürasyon Referansı (`config.toml`)

Animasyonunuzu veya özel logonuzu kalıcı hale getirmek için `~/.config/rustfetch/config.toml` dosyanıza ekleyebilirsiniz:

```toml
[general.logo]
enabled = true
# Statik özel ASCII dosyası:
ascii_path = "~/.config/rustfetch/my_logo.txt"

# Veya kare kare ASCII animasyonu:
[general.logo.animation]
enabled = true
path = "~/.config/rustfetch/animations/spinner.txt" # veya bir klasör yolu
fps = 15.0
exit_on_key = true # Tuşa basılınca kabuğu devret
infinite = true    # Sürekli döngü
```

---

## 6. CLI Parametreleri

| Parametre | Açıklama | Varsayılan |
| :--- | :--- | :--- |
| `--ascii <YOL>` | Statik özel ASCII sanat dosyası (2D veya 3D) | Yok |
| `--ascii-anim <YOL>` / `--anim` | Kare kare ASCII animasyon dosyası veya klasörü | Yok |
| `--fps <FLOAT>` | Animasyon saniyedeki kare hızı (1.0 - 60.0) | `15.0` |
| `--frames <N>` | Belirtilen kare sayısı kadar oynatıp dur | Sınırsız |
| `--hold` | Klavye yazımında durmayı engeller (`Ctrl+C` veya `q` ile çıkılır) | `false` |
| `--exit-on-key` | Tuşa basıldığında anında kabuğa devreder | `true` |

---

## 7. Hazır Örnekler

Depo içerisinde hemen test edebileceğiniz örnek animasyonlar mevcuttur:

1. **Tek Dosyalı Dönen Gösterge (`spinner.txt`)**:
   ```bash
   rustfetch --ascii-anim assets/animations/spinner.txt --fps 15
   ```

2. **Klasör Bazlı Nabız Animasyonu (`pulse/`)**:
   ```bash
   rustfetch --ascii-anim assets/animations/pulse/ --fps 12
   ```

3. **Özel Çizimi 3D Olarak Döndürme**:
   ```bash
   rustfetch --ascii assets/animations/pulse/01.txt --3d
   ```
