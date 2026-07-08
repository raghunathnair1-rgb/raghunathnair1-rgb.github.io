#!/usr/bin/env python3
"""Generate the blog's Open Graph / social card (1200x630) in the terminal/CRT aesthetic."""
from PIL import Image, ImageDraw, ImageFont, ImageFilter

W, H = 1200, 630
BOLD = "/tmp/ogfonts/jbmono-bold.ttf"
REG  = "/tmp/ogfonts/jbmono-reg.ttf"

BG      = (9, 13, 12)
GREEN   = (126, 255, 176)
GREEN_D = (78, 190, 130)
CYAN    = (34, 211, 238)
MUTED   = (120, 150, 140)
WHITE   = (222, 240, 232)
AMBER   = (255, 176, 92)

def font(path, sz): return ImageFont.truetype(path, sz)

# --- base with a subtle vertical gradient (darker top & bottom) ---
base = Image.new("RGB", (W, H), BG)
for y in range(H):
    d = abs(y - H/2) / (H/2)          # 0 center -> 1 edges
    f = 1.0 - 0.35 * d
    base.putpixel  # noqa (placeholder to keep linters quiet)
px = base.load()
for y in range(H):
    d = abs(y - H/2) / (H/2)
    f = 1.0 - 0.30 * (d ** 1.4)
    row = (int(BG[0]*f), int(BG[1]*f), int(BG[2]*f))
    for x in range(W):
        px[x, y] = row
img = base.convert("RGBA")
draw = ImageDraw.Draw(img)

# --- rounded terminal frame ---
m = 26
draw.rounded_rectangle([m, m, W-m, H-m], radius=18, outline=(40, 70, 60), width=2)
# title bar
bar_y = m + 54
draw.line([m+2, bar_y, W-m-2, bar_y], fill=(28, 48, 42), width=2)
for i, c in enumerate([(255,95,86), (255,189,46), (39,201,63)]):
    draw.ellipse([m+26 + i*30, m+22, m+26+16 + i*30, m+22+16], fill=c)
draw.text((m+140, m+22), "raghu@dark-factory: ~/blog", font=font(REG, 22), fill=MUTED)

# --- glow helper: blurred bright copy under sharp text ---
def glow_text(xy, text, fnt, color, glow=GREEN, radius=10, passes=2):
    layer = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    ld = ImageDraw.Draw(layer)
    ld.text(xy, text, font=fnt, fill=glow + (255,))
    blur = layer.filter(ImageFilter.GaussianBlur(radius))
    for _ in range(passes):
        img.alpha_composite(blur)
    draw.text(xy, text, font=fnt, fill=color)

def center_x(text, fnt):
    b = draw.textbbox((0, 0), text, font=fnt)
    return (W - (b[2]-b[0])) // 2 - b[0]

# --- prompt + title block ---
pf = font(REG, 26)
draw.text((m+40, bar_y+38), "$ whoami", font=pf, fill=GREEN_D)

tf = font(BOLD, 96)
title = "the dark factory"
glow_text((center_x(title, tf), bar_y+80), title, tf, GREEN, glow=GREEN, radius=12, passes=2)

sf = font(REG, 33)
sub = "an AI that writes, curates & ships this blog"
draw.text((center_x(sub, sf), bar_y+205), sub, font=sf, fill=WHITE)
sub2 = "— while I sleep."
draw.text((center_x(sub2, sf), bar_y+248), sub2, font=sf, fill=MUTED)

# --- tech chips line ---
chf = font(REG, 25)
chips = "rust · wasm · yew · litellm router · 2× GB10 cluster"
draw.text((center_x(chips, chf), H-150), chips, font=chf, fill=CYAN)

# --- bottom prompt / url with cursor ---
uf = font(BOLD, 27)
url = "raghunathnair1-rgb.github.io"
ux = center_x(url + "  ", uf)
draw.text((ux, H-95), url, font=uf, fill=GREEN)
ub = draw.textbbox((0,0), url + " ", font=uf)
draw.rectangle([ux + (ub[2]-ub[0]), H-95, ux + (ub[2]-ub[0]) + 15, H-95+30], fill=GREEN)

# --- CRT scanlines + vignette overlay ---
scan = Image.new("RGBA", (W, H), (0, 0, 0, 0))
sd = ImageDraw.Draw(scan)
for y in range(0, H, 3):
    sd.line([0, y, W, y], fill=(0, 0, 0, 46), width=1)
img = Image.alpha_composite(img, scan)

vig = Image.new("L", (W, H), 0)
vd = ImageDraw.Draw(vig)
vd.ellipse([-W//3, -H//3, W+W//3, H+H//3], fill=255)
vig = vig.filter(ImageFilter.GaussianBlur(120))
dark = Image.new("RGBA", (W, H), (0, 0, 0, 120))
dark.putalpha(Image.eval(vig, lambda v: 120 - int(v/255*120)))
img = Image.alpha_composite(img, dark)

img.convert("RGB").save("/tmp/og.png", "PNG")
print("wrote /tmp/og.png", img.size)
