#!/usr/bin/env python3
"""Generate wormzone game textures (Gruvbox themed)."""

from PIL import Image, ImageDraw, ImageFilter
import math
import os

OUT = os.path.join(os.path.dirname(__file__), '..', 'assets', 'textures')
os.makedirs(OUT, exist_ok=True)

# Gruvbox palette
BG      = (0x28, 0x28, 0x28)
FG      = (0xeb, 0xdb, 0xb2)
RED     = (0xcc, 0x24, 0x1d)
GREEN   = (0x98, 0x97, 0x1a)
YELLOW  = (0xd7, 0x99, 0x21)
BLUE    = (0x45, 0x85, 0x88)
PURPLE  = (0xb1, 0x62, 0x86)
AQUA    = (0x68, 0x9d, 0x6a)
ORANGE  = (0xd6, 0x5d, 0x0e)
GRAY    = (0x92, 0x83, 0x74)
BG_SOFT = (0x32, 0x30, 0x2f)

def circle(size, color, outline=None, outline_w=2):
    """Filled circle with optional outline."""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    pad = outline_w if outline else 0
    draw.ellipse([pad, pad, size-1-pad, size-1-pad], fill=(*color, 255))
    if outline:
        draw.ellipse([0, 0, size-1, size-1], outline=(*outline, 255), width=outline_w)
    return img

def ring(size, color, thickness=4):
    """Ring (donut shape)."""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    draw.ellipse([0, 0, size-1, size-1], outline=(*color, 255), width=thickness)
    return img

def glow_circle(size, color, glow_radius=4):
    """Circle with soft glow."""
    total = size + glow_radius * 2
    img = Image.new('RGBA', (total, total), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    # Glow
    for i in range(glow_radius, 0, -1):
        alpha = int(40 * (1 - i / glow_radius))
        draw.ellipse(
            [glow_radius - i, glow_radius - i, total - 1 - glow_radius + i, total - 1 - glow_radius + i],
            fill=(*color, alpha)
        )
    # Core
    draw.ellipse(
        [glow_radius, glow_radius, total - 1 - glow_radius, total - 1 - glow_radius],
        fill=(*color, 255)
    )
    return img

def worm_head(size, body_color, eye_color=FG):
    """Worm head with two eyes."""
    img = glow_circle(size, body_color, glow_radius=3)
    draw = ImageDraw.Draw(img)
    total = size + 6  # account for glow
    cx, cy = total // 2, total // 2

    # Eyes (positioned at ~60% from center, upper half)
    eye_r = max(size // 8, 3)
    pupil_r = max(eye_r // 2, 1)

    # Left eye
    ex1, ey1 = cx - size // 5, cy - size // 6
    draw.ellipse([ex1-eye_r, ey1-eye_r, ex1+eye_r, ey1+eye_r], fill=(*eye_color, 255))
    draw.ellipse([ex1-pupil_r, ey1-pupil_r, ex1+pupil_r, ey1+pupil_r], fill=(0x1d, 0x20, 0x21, 255))

    # Right eye
    ex2, ey2 = cx + size // 5, cy - size // 6
    draw.ellipse([ex2-eye_r, ey2-eye_r, ex2+eye_r, ey2+eye_r], fill=(*eye_color, 255))
    draw.ellipse([ex2-pupil_r, ey2-pupil_r, ex2+pupil_r, ey2+pupil_r], fill=(0x1d, 0x20, 0x21, 255))

    return img

def worm_segment(size, color):
    """Body segment — circle with subtle inner highlight."""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    # Outer
    draw.ellipse([0, 0, size-1, size-1], fill=(*color, 230))
    # Inner highlight
    hl = size // 4
    lighter = tuple(min(c + 30, 255) for c in color)
    draw.ellipse([hl, hl, size-1-hl, size-1-hl], fill=(*lighter, 180))
    return img

def cherry(size):
    """Cherry — small red circle with green stem."""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    # Berry
    r = size // 3
    cx, cy = size // 2, size // 2 + r // 2
    draw.ellipse([cx-r, cy-r, cx+r, cy+r], fill=(*RED, 255))
    # Highlight
    hr = r // 3
    draw.ellipse([cx-r+hr, cy-r+hr, cx-r+hr*2, cy-r+hr*2], fill=(255, 100, 100, 150))
    # Stem
    draw.line([(cx, cy-r), (cx+r//2, cy-r-r)], fill=(*GREEN, 255), width=2)
    return img

def cookie(size):
    """Cookie — brown circle with darker spots."""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    r = size // 2 - 2
    cx, cy = size // 2, size // 2
    # Base
    draw.ellipse([cx-r, cy-r, cx+r, cy+r], fill=(*YELLOW, 255))
    # Chocolate chips
    chip_color = (0x50, 0x30, 0x10, 255)
    spots = [(cx-r//3, cy-r//4), (cx+r//4, cy+r//3), (cx-r//5, cy+r//5),
             (cx+r//3, cy-r//3), (cx, cy)]
    cr = max(r // 6, 2)
    for sx, sy in spots:
        draw.ellipse([sx-cr, sy-cr, sx+cr, sy+cr], fill=chip_color)
    return img

def donut(size):
    """Donut — ring with sprinkles."""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    cx, cy = size // 2, size // 2
    r_outer = size // 2 - 2
    r_inner = r_outer // 2
    # Pink frosting ring
    draw.ellipse([cx-r_outer, cy-r_outer, cx+r_outer, cy+r_outer], fill=(*PURPLE, 255))
    draw.ellipse([cx-r_inner, cy-r_inner, cx+r_inner, cy+r_inner], fill=(0, 0, 0, 0))
    # Sprinkles
    import random
    random.seed(42)
    sprinkle_colors = [RED, GREEN, YELLOW, AQUA, ORANGE]
    for _ in range(8):
        angle = random.uniform(0, 2 * math.pi)
        dist = random.uniform(r_inner + 2, r_outer - 2)
        sx = int(cx + math.cos(angle) * dist)
        sy = int(cy + math.sin(angle) * dist)
        sc = random.choice(sprinkle_colors)
        draw.rectangle([sx-1, sy-1, sx+1, sy+1], fill=(*sc, 255))
    return img

def joystick_base(size):
    """Virtual joystick base — translucent circle."""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    draw.ellipse([2, 2, size-3, size-3], fill=(*BG_SOFT, 100), outline=(*GRAY, 150), width=2)
    return img

def joystick_knob(size):
    """Virtual joystick knob."""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    draw.ellipse([2, 2, size-3, size-3], fill=(*GREEN, 180), outline=(*AQUA, 200), width=2)
    return img

def particle_glow(size):
    """Soft glow particle for effects."""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    cx, cy = size // 2, size // 2
    for y in range(size):
        for x in range(size):
            dist = math.sqrt((x - cx)**2 + (y - cy)**2)
            if dist < cx:
                alpha = int(200 * (1 - dist / cx) ** 2)
                img.putpixel((x, y), (255, 255, 255, alpha))
    return img

def grid_tile(size):
    """Subtle grid tile for arena background."""
    img = Image.new('RGBA', (size, size), (*BG, 255))
    draw = ImageDraw.Draw(img)
    draw.line([(0, 0), (size-1, 0)], fill=(*BG_SOFT, 255), width=1)
    draw.line([(0, 0), (0, size-1)], fill=(*BG_SOFT, 255), width=1)
    return img

# ============================================================================
# Generate all textures
# ============================================================================

print("Generating wormzone textures...")

# Worm heads (one per color)
colors = {
    'green': GREEN,
    'red': RED,
    'yellow': YELLOW,
    'blue': BLUE,
    'purple': PURPLE,
    'aqua': AQUA,
    'orange': ORANGE,
}

for name, color in colors.items():
    worm_head(40, color).save(os.path.join(OUT, f'worm_head_{name}.png'))
    worm_segment(32, color).save(os.path.join(OUT, f'worm_segment_{name}.png'))
    print(f"  worm_{name} head+segment")

# Food
donut(32).save(os.path.join(OUT, 'food_donut.png'))
cookie(32).save(os.path.join(OUT, 'food_cookie.png'))
cherry(32).save(os.path.join(OUT, 'food_cherry.png'))
print("  food: donut, cookie, cherry")

# UI
joystick_base(128).save(os.path.join(OUT, 'joystick_base.png'))
joystick_knob(48).save(os.path.join(OUT, 'joystick_knob.png'))
print("  joystick: base, knob")

# Effects
particle_glow(32).save(os.path.join(OUT, 'particle_glow.png'))
print("  particle_glow")

# Arena
grid_tile(100).save(os.path.join(OUT, 'grid_tile.png'))
print("  grid_tile")

print(f"\nDone! {len(os.listdir(OUT))} textures in {OUT}")
