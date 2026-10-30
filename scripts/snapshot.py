#!/usr/bin/env python3
"""
RustFetch Visual Snapshot & Terminal Eye Harness
Captures pixel-perfect terminal renders across arbitrary dimensions (Cols x Rows)
and saves them as PNG images for multimodal AI visual inspection and human verification.
"""

import argparse
import base64
import datetime
import io
import os
import re
import subprocess
import sys
import zlib
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

ROOT_DIR = Path(__file__).resolve().parent.parent
BIN_PATH = ROOT_DIR / "target" / "release" / "rustfetch"
SCRATCH_DIR = ROOT_DIR / "scratch" / "snapshots"

# Simulated terminal profiles for realistic rendering and environment verification
TERMINAL_PROFILES = {
    "kitty": {
        "env": {"TERM": "xterm-kitty", "KITTY_WINDOW_ID": "1", "KITTY_PID": "12345"},
        "default_protocol": "kitty",
    },
    "ghostty": {
        "env": {"TERM": "xterm-ghostty", "TERM_PROGRAM": "ghostty", "GHOSTTY_RESOURCES_DIR": "/tmp"},
        "default_protocol": "kitty",
    },
    "wezterm": {
        "env": {"TERM": "xterm-256color", "TERM_PROGRAM": "WezTerm", "WEZTERM_PANE": "1"},
        "default_protocol": "kitty",
    },
    "alacritty": {
        "env": {"TERM": "alacritty", "ALACRITTY_WINDOW_ID": "1", "ALACRITTY_SOCKET": "/tmp/alacritty.sock"},
        "default_protocol": "halfblocks",
    },
    "foot": {
        "env": {"TERM": "foot", "FOOT_SERVER_PATH": "/tmp/foot.sock"},
        "default_protocol": "sixel",
    },
    "konsole": {
        "env": {"TERM": "xterm-256color", "KONSOLE_VERSION": "240200", "COLORTERM": "truecolor"},
        "default_protocol": "iterm",
    },
    "ptyxis": {
        "env": {"TERM": "xterm-256color", "PTYXIS_VERSION": "47", "COLORTERM": "truecolor"},
        "default_protocol": "halfblocks",
    },
    "gnome": {
        "env": {"TERM": "xterm-256color", "VTE_VERSION": "7600", "COLORTERM": "truecolor"},
        "default_protocol": "halfblocks",
    },
}

def extract_clean_label(cli_args, preset=None):
    if preset:
        return preset
    label_parts = []
    i = 0
    while i < len(cli_args):
        arg = cli_args[i]
        if arg in ("-c", "--config") and i + 1 < len(cli_args):
            cfg_name = Path(cli_args[i + 1]).stem.replace("config_", "")
            label_parts.append(cfg_name)
            i += 2
            continue
        elif arg in ("--image", "-i") and i + 1 < len(cli_args):
            img_name = Path(cli_args[i + 1]).stem.replace("_nobg", "")
            if len(img_name) > 16:
                img_name = img_name[:8]
            label_parts.append(img_name)
            i += 2
            continue
        elif arg in ("-p", "--preset") and i + 1 < len(cli_args):
            label_parts.append(cli_args[i + 1])
            i += 2
            continue
        elif arg in ("--protocol",) and i + 1 < len(cli_args):
            i += 2
            continue
        elif not arg.startswith("-"):
            clean_arg = re.sub(r"[^a-zA-Z0-9_\-]", "_", Path(arg).stem)
            if clean_arg:
                label_parts.append(clean_arg)
        i += 1
    label = "_".join(label_parts) if label_parts else "default"
    label = re.sub(r"[^a-zA-Z0-9_\-]", "_", label)
    label = re.sub(r"_+", "_", label).strip("_")
    return label or "default"
FONT_CANDIDATES = [
    Path.home() / ".local/share/fonts/0xProto/0xProtoNerdFontMono-Regular.ttf",
    Path("/usr/share/fonts/fira-code/FiraCode-Regular.ttf"),
    Path("/usr/share/fonts/google-noto/NotoSansMono-Regular.ttf"),
]

def find_font():
    for f in FONT_CANDIDATES:
        if f.is_file():
            return str(f)
    return None

def parse_ansi_to_grid(ansi_text):
    lines = ansi_text.splitlines()
    grid = []
    ansi_re = re.compile(r'\x1b\[([0-9;]*)m')
    
    # 16-color ANSI palette (Catppuccin Mocha themed)
    standard_colors = [
        (69, 71, 90),     # 30: black
        (243, 139, 168),  # 31: red
        (166, 227, 161),  # 32: green
        (249, 226, 175),  # 33: yellow
        (137, 180, 250),  # 34: blue
        (245, 194, 231),  # 35: magenta
        (148, 226, 213),  # 36: cyan
        (186, 194, 222),  # 37: white
    ]
    bright_colors = [
        (88, 91, 112),    # 90: bright black
        (235, 111, 146),  # 91: bright red
        (156, 207, 216),  # 92: bright green
        (246, 193, 119),  # 93: bright yellow
        (140, 170, 238),  # 94: bright blue
        (196, 167, 231),  # 95: bright magenta
        (235, 188, 186),  # 96: bright cyan
        (224, 222, 244),  # 97: bright white
    ]

    default_fg = (205, 214, 244) # #cdd6f4

    for line in lines:
        row = []
        cur_fg = default_fg
        cur_bg = None
        pos = 0
        while pos < len(line):
            if line[pos] == '\x1b':
                m = ansi_re.match(line[pos:])
                if m:
                    codes = [int(c) for c in m.group(1).split(';') if c] if m.group(1) else [0]
                    i = 0
                    while i < len(codes):
                        c = codes[i]
                        if c == 0:
                            cur_fg = default_fg
                            cur_bg = None
                        elif 30 <= c <= 37:
                            cur_fg = standard_colors[c - 30]
                        elif 90 <= c <= 97:
                            cur_fg = bright_colors[c - 90]
                        elif 40 <= c <= 47:
                            cur_bg = standard_colors[c - 40]
                        elif 100 <= c <= 107:
                            cur_bg = bright_colors[c - 100]
                        elif c == 39:
                            cur_fg = default_fg
                        elif c == 49:
                            cur_bg = None
                        elif c == 38 and i + 4 < len(codes) and codes[i+1] == 2:
                            cur_fg = (codes[i+2], codes[i+3], codes[i+4])
                            i += 4
                        elif c == 48 and i + 4 < len(codes) and codes[i+1] == 2:
                            cur_bg = (codes[i+2], codes[i+3], codes[i+4])
                            i += 4
                        i += 1
                    pos += len(m.group(0))
                    continue
                else:
                    pos += 1
                    continue
            ch = line[pos]
            row.append((ch, cur_fg, cur_bg))
            pos += 1
        grid.append(row)
    return grid

def capture_tmux(cols, rows, cli_args, extra_env=None):
    """Run rustfetch inside an isolated tmux session with exact cols x rows."""
    session_id = f"rf_snap_{os.getpid()}"
    args_str = " ".join(cli_args)
    env_prefix = ""
    if extra_env:
        env_prefix = " ".join(f"{k}='{v}'" for k, v in extra_env.items()) + " "
    cmd = f"bash -c '{env_prefix}{BIN_PATH} {args_str}; sleep 2'"
    
    subprocess.run(["tmux", "kill-session", "-t", session_id], stderr=subprocess.DEVNULL)
    run_res = subprocess.run(
        ["tmux", "new-session", "-d", "-x", str(cols), "-y", str(rows), "-s", session_id, cmd],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.PIPE,
        text=True,
    )
    if run_res.returncode != 0:
        # Fallback to direct execution with COLUMNS/LINES environment variables
        env = os.environ.copy()
        env["COLUMNS"] = str(cols)
        env["LINES"] = str(rows)
        env["TERM"] = "xterm-256color"
        if extra_env:
            env.update(extra_env)
        direct_cmd = [str(BIN_PATH)] + cli_args
        res = subprocess.run(direct_cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=env, text=True)
        return res.stdout

    import time
    # Poll until tmux pane has content (fast for text, waits for large GIFs up to 1.5s)
    cap_stdout = ""
    for _ in range(15):
        time.sleep(0.1)
        cap = subprocess.run(
            ["tmux", "capture-pane", "-t", session_id, "-p", "-e"],
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
            text=True,
        )
        if cap.stdout.strip():
            cap_stdout = cap.stdout
            break

    subprocess.run(["tmux", "kill-session", "-t", session_id], stderr=subprocess.DEVNULL)
    return cap_stdout

def extract_graphic_image(cols, rows, cli_args, extra_env=None):
    """Run rustfetch to capture Kitty, iTerm2, or Sixel graphic stream if generated."""
    env = os.environ.copy()
    env["COLUMNS"] = str(cols)
    env["LINES"] = str(rows)
    env["TERM"] = "xterm-kitty"
    env["KITTY_PID"] = str(os.getpid())
    if extra_env:
        env.update(extra_env)

    cmd = [str(BIN_PATH)] + list(cli_args)
    try:
        p = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=env)
        out, _ = p.communicate()
    except Exception:
        return None

    # 1. Try Kitty Protocol (\x1b_G ... \x1b\)
    sep_start = b"\x1b_G"
    sep_end = b"\x1b\\"
    chunks = []
    start = 0
    while True:
        idx = out.find(sep_start, start)
        if idx == -1:
            break
        end_idx = out.find(sep_end, idx)
        if end_idx == -1:
            break
        chunks.append(out[idx + len(sep_start):end_idx])
        start = end_idx + len(sep_end)

    if chunks:
        try:
            frame0_chunks = []
            for c in chunks:
                parts = c.split(b";", 1)
                hdr = parts[0].decode("latin1")
                frame0_chunks.append(parts[1])
                if "m=0" in hdr:
                    break

            header = chunks[0].split(b";", 1)[0].decode("latin1")
            meta = dict(kv.split("=") for kv in header.split(",") if "=" in kv)
            w = int(meta.get("s", 0))
            v = int(meta.get("v", 0))
            c = int(meta.get("c", 0))
            r = int(meta.get("r", 0))
            all_b64 = b"".join(frame0_chunks)
            raw_b64 = base64.b64decode(all_b64)
            use_compression = "o=z" in header
            if use_compression:
                decompressed = zlib.decompress(raw_b64)
            else:
                decompressed = raw_b64
            img = Image.frombytes("RGBA", (w, v), decompressed)
            return {"image": img, "cols": c, "rows": r, "protocol": "kitty"}
        except Exception:
            pass

    # 2. Try iTerm2 Protocol (\x1b]1337;File=...:\x07 or \x1b\)
    iterm_marker = b"1337;File="
    idx = out.find(iterm_marker)
    if idx != -1:
        try:
            colon = out.find(b":", idx)
            end = out.find(b"\x07", colon)
            if end == -1:
                end = out.find(b"\x1b\\", colon)
            if colon != -1 and end != -1:
                meta_str = out[idx:colon].decode("latin1", errors="replace")
                meta = dict(kv.split("=", 1) for kv in meta_str.split(";") if "=" in kv)
                c = int(meta.get("width", 34))
                r = int(meta.get("height", 20))
                b64_data = out[colon + 1:end]
                raw = base64.b64decode(b64_data)
                img = Image.open(io.BytesIO(raw)).convert("RGBA")
                return {"image": img, "cols": c, "rows": r, "protocol": "iterm"}
        except Exception:
            pass

    # 3. Try Sixel Protocol (\x1bPq"1;1;... \x1b\)
    sixel_match = re.search(rb"\x1bPq\"1;1;(\d+);(\d+)(.*?)\x1b\\", out, re.DOTALL)
    if sixel_match:
        try:
            w, h = int(sixel_match.group(1)), int(sixel_match.group(2))
            payload = sixel_match.group(3).decode("latin1", errors="replace")
            palette = {}
            pixels = bytearray(w * h * 4)
            pos = 0
            cur_color = (0, 0, 0, 255)
            x, y = 0, 0
            while pos < len(payload):
                ch = payload[pos]
                if ch == "#":
                    pos += 1
                    s_idx = pos
                    while pos < len(payload) and payload[pos] in "0123456789;":
                        pos += 1
                    cmd = payload[s_idx:pos].split(";")
                    color_id = int(cmd[0])
                    if len(cmd) >= 5 and cmd[1] == "2":
                        r_col = int(int(cmd[2]) * 255 / 100)
                        g_col = int(int(cmd[3]) * 255 / 100)
                        b_col = int(int(cmd[4]) * 255 / 100)
                        palette[color_id] = (r_col, g_col, b_col, 255)
                    else:
                        cur_color = palette.get(color_id, (0, 0, 0, 255))
                    continue
                elif ch == "$":
                    x = 0
                    pos += 1
                    continue
                elif ch == "-":
                    x = 0
                    y += 6
                    pos += 1
                    continue
                elif ch == "!":
                    pos += 1
                    s_idx = pos
                    while pos < len(payload) and payload[pos].isdigit():
                        pos += 1
                    count = int(payload[s_idx:pos])
                    repeat_ch = payload[pos]
                    pos += 1
                    chars_to_process = repeat_ch * count
                elif "?" <= ch <= "~":
                    chars_to_process = ch
                    pos += 1
                else:
                    pos += 1
                    continue

                for c_item in chars_to_process:
                    val = ord(c_item) - 63
                    if 0 <= x < w:
                        for dy in range(6):
                            if (val >> dy) & 1:
                                py = y + dy
                                if py < h:
                                    p_idx = (py * w + x) * 4
                                    pixels[p_idx] = cur_color[0]
                                    pixels[p_idx + 1] = cur_color[1]
                                    pixels[p_idx + 2] = cur_color[2]
                                    pixels[p_idx + 3] = cur_color[3]
                    x += 1
            img = Image.frombytes("RGBA", (w, h), bytes(pixels))
            c = max(1, w // 9)
            r = max(1, h // 20)
            return {"image": img, "cols": c, "rows": r, "protocol": "sixel"}
        except Exception:
            pass

    return None

def render_grid_to_png(grid, cols, rows, output_path, title=None, graphic_data=None):
    output_path.parent.mkdir(parents=True, exist_ok=True)
    font_file = find_font()
    font_size = 14
    if font_file:
        font = ImageFont.truetype(font_file, font_size)
    else:
        font = ImageFont.load_default()

    char_w = 9
    char_h = 20
    pad_x = 24
    pad_y = 36 # Header bar room

    img_w = max(cols * char_w + pad_x * 2, 320)
    actual_rows = max(len(grid), rows)
    img_h = actual_rows * char_h + pad_y + pad_x

    # Dark terminal canvas (#181825 Catppuccin Mantle)
    img = Image.new("RGB", (img_w, img_h), color=(24, 24, 37))
    draw = ImageDraw.Draw(img)

    # Window title bar decoration (macOS / GNOME style dots)
    draw.ellipse([pad_x, 14, pad_x + 10, 24], fill=(243, 139, 168))     # Red
    draw.ellipse([pad_x + 16, 14, pad_x + 26, 24], fill=(249, 226, 175)) # Yellow
    draw.ellipse([pad_x + 32, 14, pad_x + 42, 24], fill=(166, 227, 161)) # Green

    if title:
        draw.text((pad_x + 60, 11), title, font=font, fill=(166, 173, 200))

    # Composite graphic protocol image (Kitty, iTerm2, Sixel) if present
    if graphic_data and graphic_data.get("image"):
        g_img = graphic_data["image"]
        g_cols = graphic_data.get("cols", 34)
        g_rows = graphic_data.get("rows", 20)
        target_w = g_cols * char_w
        target_h = g_rows * char_h

        card_start_col = None
        for row in grid[:max(g_rows, 5)]:
            for c_idx, (ch, fg, bg) in enumerate(row):
                if ch not in (" ", ""):
                    if card_start_col is None or c_idx < card_start_col:
                        card_start_col = c_idx
                    break

        gap = 4
        if card_start_col and card_start_col > g_cols:
            logo_x_col = max(0, card_start_col - gap - g_cols)
        else:
            logo_x_col = 0

        logo_x = pad_x + logo_x_col * char_w
        logo_y = pad_y

        resized_img = g_img.resize((target_w, target_h), Image.Resampling.LANCZOS)
        img.paste(resized_img, (logo_x, logo_y), resized_img)

    # Grid character drawing
    for r_idx, row in enumerate(grid):
        y = pad_y + r_idx * char_h
        for c_idx, (ch, fg, bg) in enumerate(row):
            x = pad_x + c_idx * char_w
            if bg:
                draw.rectangle([x, y, x + char_w, y + char_h], fill=bg)
            if ch != " ":
                draw.text((x, y), ch, font=font, fill=fg)

    img.save(output_path)
    return output_path

def snapshot(cols=80, rows=24, cli_args=None, output=None, title=None, hd=False, term=None):
    if cli_args is None:
        cli_args = []

    if not BIN_PATH.is_file():
        print(f"Building release binary {BIN_PATH}...")
        subprocess.run(["cargo", "build", "--release", "-p", "rustfetch"], cwd=ROOT_DIR, check=True)

    # Determine simulated terminal profile
    term_key = (term or "").lower()
    profile = TERMINAL_PROFILES.get(term_key)

    if not profile:
        args_str = " ".join(cli_args)
        if hd or "--protocol kitty" in args_str:
            term_name = term or "kitty"
            profile = TERMINAL_PROFILES["kitty"]
        elif "--protocol sixel" in args_str:
            term_name = term or "foot"
            profile = TERMINAL_PROFILES["foot"]
        elif "--protocol iterm" in args_str:
            term_name = term or "konsole"
            profile = TERMINAL_PROFILES["konsole"]
        elif "--protocol halfblock" in args_str or "--protocol braille" in args_str:
            term_name = term or "alacritty"
            profile = TERMINAL_PROFILES["alacritty"]
        else:
            term_name = term or "kitty"
            profile = TERMINAL_PROFILES["kitty"]
    else:
        term_name = term_key

    extra_env = profile["env"] if profile else {}

    effective_args = list(cli_args)
    if "--protocol" not in effective_args and profile:
        effective_args.extend(["--protocol", profile["default_protocol"]])
    elif hd:
        if "--protocol" in effective_args:
            idx = effective_args.index("--protocol")
            if idx + 1 < len(effective_args):
                effective_args[idx + 1] = "kitty"
        else:
            effective_args.extend(["--protocol", "kitty"])

    # Attempt to extract graphic image (Kitty, iTerm2, or Sixel)
    graphic_data = extract_graphic_image(cols, rows, effective_args, extra_env=extra_env)

    # Use effective_args for tmux capture if graphic image exists so layout offsets match
    tmux_args = effective_args if graphic_data else cli_args
    raw_output = capture_tmux(cols, rows, tmux_args, extra_env=extra_env)
    grid = parse_ansi_to_grid(raw_output)

    now = datetime.datetime.now()
    time_str = now.strftime("%Y%m%d_%H%M%S")
    human_time = now.strftime("%Y-%m-%d %H:%M:%S")

    label = extract_clean_label(cli_args)

    if output is None:
        # FLAT directly in scratch/snapshots: snap_YYYYMMDD_HHMMSS_{term}_{cols}x{rows}_{label}.png
        out_name = f"snap_{time_str}_{term_name}_{cols}x{rows}_{label}.png"
        output = SCRATCH_DIR / out_name
    else:
        output = Path(output)

    proto_name = graphic_data["protocol"].upper() if graphic_data else "ANSI"
    win_title = title or f"RustFetch [{term_name.upper()} • {proto_name}] — {cols}x{rows} [{label}] ({human_time})"
    saved = render_grid_to_png(grid, cols, rows, output, title=win_title, graphic_data=graphic_data)
    print(f"[OK] Visual snapshot saved to: {saved}")
    return saved

def main():
    parser = argparse.ArgumentParser(description="RustFetch Visual Snapshot Tester")
    parser.add_argument("-c", "--cols", type=int, default=80, help="Terminal width in columns (default: 80)")
    parser.add_argument("-r", "--rows", type=int, default=24, help="Terminal height in rows (default: 24)")
    parser.add_argument("-t", "--term", "--terminal", type=str, help="Simulated terminal (kitty, ghostty, alacritty, foot, wezterm, konsole, ptyxis, gnome)")
    parser.add_argument("-p", "--preset", type=str, help="Preset name (e.g. card, minimal, compact, dots)")
    parser.add_argument("--hd", action="store_true", help="Force True HD Kitty graphics image protocol")
    parser.add_argument("-o", "--output", type=str, help="Output PNG path")
    parser.add_argument("--matrix", action="store_true", help="Capture a full matrix of dimensions and presets")
    parser.add_argument("extra_args", nargs=argparse.REMAINDER, help="Additional arguments forwarded to rustfetch")

    args = parser.parse_args()

    cli_args = []
    if args.preset:
        cli_args.extend(["--preset", args.preset])
    if args.extra_args:
        cli_args.extend([a for a in args.extra_args if a != "--"])

    if args.matrix:
        batch_ts = datetime.datetime.now().strftime("%Y%m%d_%H%M%S")
        terms = ["ghostty", "kitty", "alacritty", "foot"]
        sizes = [
            (60, 20, "quarter-tile"),
            (80, 24, "standard"),
            (100, 32, "half-tile"),
        ]
        presets = ["card", "minimal", "compact"]
        for t in terms:
            for c, r, label in sizes:
                for p in presets:
                    out = SCRATCH_DIR / f"snap_{batch_ts}_{t}_{c}x{r}_{p}.png"
                    snapshot(cols=c, rows=r, cli_args=["--preset", p], output=out, term=t)
        print(f"\n[Matrix Complete] All snapshots generated directly in: {SCRATCH_DIR}")
        return

    snapshot(cols=args.cols, rows=args.rows, cli_args=cli_args, output=args.output, hd=args.hd, term=args.term)

if __name__ == "__main__":
    main()
