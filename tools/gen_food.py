#!/usr/bin/env python3
"""Generate food textures and board background for wormzone."""

from PIL import Image, ImageDraw
import math
import random

ASSET_DIR = "assets/textures"
SIZE = 64
C = SIZE // 2


def circle(draw, cx, cy, r, **kw):
    draw.ellipse([cx - r, cy - r, cx + r, cy + r], **kw)


def make_cherry():
    img = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    d.line([(C - 6, C - 12), (C + 2, C - 22)], fill=(100, 70, 30, 255), width=2)
    d.line([(C + 6, C - 10), (C + 2, C - 22)], fill=(100, 70, 30, 255), width=2)
    d.ellipse([C - 2, C - 26, C + 10, C - 18], fill=(80, 160, 60, 255))
    circle(d, C - 7, C + 4, 14, fill=(200, 30, 30, 255))
    circle(d, C - 7, C + 4, 10, fill=(220, 50, 40, 255))
    circle(d, C - 12, C - 2, 4, fill=(255, 120, 120, 100))
    circle(d, C + 9, C + 6, 13, fill=(190, 25, 25, 255))
    circle(d, C + 9, C + 6, 9, fill=(210, 45, 35, 255))
    circle(d, C + 5, C, 3, fill=(255, 120, 120, 100))
    img.save(f"{ASSET_DIR}/food_cherry.png")


def make_banana():
    img = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    # Banana body — rotated thick crescent
    # Draw with polygon for smooth shape
    points = []
    for i in range(30):
        t = i / 29.0
        angle = math.radians(200 + t * 150)  # arc from 200 to 350 degrees
        r_outer = 24
        x = C + math.cos(angle) * r_outer
        y = C + math.sin(angle) * r_outer
        points.append((x, y))
    # Inner arc (reverse)
    for i in range(29, -1, -1):
        t = i / 29.0
        angle = math.radians(210 + t * 130)
        r_inner = 14
        x = C + math.cos(angle) * r_inner
        y = C + math.sin(angle) * r_inner
        points.append((x, y))
    d.polygon(points, fill=(240, 210, 40, 255))
    # Lighter inner stripe
    pts2 = []
    for i in range(30):
        t = i / 29.0
        angle = math.radians(215 + t * 120)
        r = 20
        x = C + math.cos(angle) * r
        y = C + math.sin(angle) * r
        pts2.append((x, y))
    for i in range(29, -1, -1):
        t = i / 29.0
        angle = math.radians(218 + t * 114)
        r = 16
        x = C + math.cos(angle) * r
        y = C + math.sin(angle) * r
        pts2.append((x, y))
    d.polygon(pts2, fill=(255, 235, 80, 255))
    # Brown tip (bottom)
    a1 = math.radians(200)
    tx, ty = C + math.cos(a1) * 24, C + math.sin(a1) * 24
    circle(d, int(tx), int(ty), 3, fill=(140, 110, 30, 255))
    # Brown tip (top)
    a2 = math.radians(350)
    tx2, ty2 = C + math.cos(a2) * 24, C + math.sin(a2) * 24
    circle(d, int(tx2), int(ty2), 3, fill=(140, 110, 30, 255))
    img.save(f"{ASSET_DIR}/food_banana.png")


def make_apple():
    img = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    circle(d, C, C + 4, 22, fill=(180, 30, 30, 255))
    circle(d, C, C + 4, 18, fill=(210, 45, 35, 255))
    d.ellipse([C - 12, C - 8, C - 2, C + 4], fill=(240, 100, 90, 120))
    d.line([(C, C - 16), (C + 3, C - 24)], fill=(100, 70, 30, 255), width=2)
    d.ellipse([C + 2, C - 26, C + 14, C - 18], fill=(70, 150, 50, 255))
    img.save(f"{ASSET_DIR}/food_apple.png")


def make_grape():
    img = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    d.line([(C, 4), (C, C - 10)], fill=(100, 80, 40, 255), width=2)
    d.ellipse([C - 1, 2, C + 6, 10], fill=(70, 140, 50, 255))
    positions = [
        (C, C - 6), (C - 8, C - 2), (C + 8, C - 2),
        (C - 4, C + 6), (C + 4, C + 6), (C - 10, C + 8),
        (C + 10, C + 8), (C, C + 12), (C - 6, C + 16),
        (C + 6, C + 16), (C, C + 20),
    ]
    for x, y in positions:
        circle(d, x, y, 7, fill=(120, 60, 160, 255))
        circle(d, x, y, 5, fill=(150, 90, 190, 255))
        circle(d, x - 2, y - 2, 2, fill=(180, 140, 210, 100))
    img.save(f"{ASSET_DIR}/food_grape.png")


def make_watermelon():
    img = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    d.pieslice([6, 8, 58, 60], start=180, end=360, fill=(40, 140, 50, 255))
    d.pieslice([9, 11, 55, 57], start=180, end=360, fill=(60, 170, 70, 255))
    d.pieslice([12, 14, 52, 54], start=180, end=360, fill=(220, 50, 60, 255))
    d.pieslice([12, 14, 52, 54], start=200, end=340, fill=(235, 70, 75, 255))
    for sx, sy in [(24, 28), (32, 26), (40, 28), (28, 22), (36, 22)]:
        d.ellipse([sx - 1, sy - 2, sx + 1, sy + 2], fill=(40, 30, 20, 255))
    img.save(f"{ASSET_DIR}/food_watermelon.png")


def make_strawberry():
    img = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    d.polygon(
        [(C, C - 14), (C + 16, C + 4), (C + 10, C + 18), (C, C + 22),
         (C - 10, C + 18), (C - 16, C + 4)],
        fill=(210, 40, 40, 255),
    )
    circle(d, C, C + 4, 16, fill=(220, 50, 45, 255))
    d.ellipse([C - 10, C - 8, C - 2, C + 2], fill=(250, 110, 100, 100))
    for sx, sy in [(C - 6, C), (C + 6, C), (C - 3, C + 8), (C + 3, C + 8),
                   (C, C + 4), (C - 8, C + 6), (C + 8, C + 6)]:
        circle(d, sx, sy, 1, fill=(255, 220, 80, 200))
    for angle_off in [-20, -10, 0, 10, 20]:
        a = math.radians(90 + angle_off)
        lx = C + math.cos(a) * 4
        ly = C - 14 - abs(math.sin(a)) * 4
        d.ellipse([lx - 5, ly - 3, lx + 5, ly + 5], fill=(60, 150, 50, 255))
    img.save(f"{ASSET_DIR}/food_strawberry.png")


def make_donut():
    img = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    circle(d, C, C, 22, fill=(210, 160, 120, 255))
    circle(d, C, C, 18, fill=(230, 180, 140, 255))
    d.pieslice([C - 18, C - 18, C + 18, C + 18], start=180, end=360, fill=(240, 130, 170, 255))
    circle(d, C, C, 8, fill=(0, 0, 0, 0))
    for sx, sy, sc in [(C - 10, C - 10, (255, 255, 80)), (C + 8, C - 8, (80, 200, 255)),
                       (C - 6, C - 14, (100, 255, 100)), (C + 4, C - 14, (255, 150, 50))]:
        d.line([(sx, sy), (sx + 3, sy + 2)], fill=(*sc, 255), width=2)
    img.save(f"{ASSET_DIR}/food_donut.png")


def make_cookie():
    img = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    circle(d, C, C, 20, fill=(180, 140, 80, 255))
    circle(d, C, C, 17, fill=(200, 165, 100, 255))
    for cx, cy in [(C - 8, C - 6), (C + 6, C - 4), (C - 3, C + 6),
                   (C + 8, C + 8), (C - 6, C + 10), (C + 2, C - 10)]:
        circle(d, cx, cy, 3, fill=(80, 50, 20, 255))
        circle(d, cx, cy, 2, fill=(100, 65, 30, 255))
    img.save(f"{ASSET_DIR}/food_cookie.png")


def make_board_bg():
    """Minecraft-meets-code dark board background.
    Dark stone blocks with subtle code symbols etched in.
    """
    size = 200
    random.seed(42)
    img = Image.new("RGBA", (size, size), (32, 34, 32, 255))
    d = ImageDraw.Draw(img)

    # Minecraft-style blocks with slight shade variation
    block = 20
    for bx in range(0, size, block):
        for by in range(0, size, block):
            v = random.randint(28, 38)
            d.rectangle([bx, by, bx + block - 1, by + block - 1], fill=(v, v + 1, v, 255))
            for _ in range(8):
                px = bx + random.randint(1, block - 2)
                py = by + random.randint(1, block - 2)
                nv = v + random.randint(-4, 4)
                d.point((px, py), fill=(nv, nv + 1, nv, 255))

    # Dark grooves between blocks
    for x in range(0, size + 1, block):
        d.line([(x, 0), (x, size - 1)], fill=(22, 24, 22, 255), width=1)
    for y in range(0, size + 1, block):
        d.line([(0, y), (size - 1, y)], fill=(22, 24, 22, 255), width=1)

    # Scattered code symbols (very subtle, etched into stone)
    symbols = ["{}", "//", "=>", "fn", "01", "++", "&&", "[]", "<>", "##",
               "::", "if", "->", "0x", "();", "///"]
    sym_color = (40, 44, 40, 255)
    for _ in range(12):
        sx = random.randint(4, size - 20)
        sy = random.randint(4, size - 14)
        sym = random.choice(symbols)
        for ci, ch in enumerate(sym):
            cx = sx + ci * 4
            if ch in "{}[]()<>":
                d.line([(cx + 1, sy), (cx + 1, sy + 4)], fill=sym_color, width=1)
            elif ch == "/":
                d.line([(cx + 2, sy), (cx, sy + 4)], fill=sym_color, width=1)
            elif ch in "#=":
                d.line([(cx, sy + 1), (cx + 2, sy + 1)], fill=sym_color, width=1)
                d.line([(cx, sy + 3), (cx + 2, sy + 3)], fill=sym_color, width=1)
            elif ch == "+":
                d.line([(cx, sy + 2), (cx + 2, sy + 2)], fill=sym_color, width=1)
                d.point((cx + 1, sy + 1), fill=sym_color)
                d.point((cx + 1, sy + 3), fill=sym_color)
            elif ch == "-":
                d.line([(cx, sy + 2), (cx + 2, sy + 2)], fill=sym_color, width=1)
            elif ch == ">":
                d.line([(cx, sy), (cx + 2, sy + 2)], fill=sym_color, width=1)
                d.line([(cx, sy + 4), (cx + 2, sy + 2)], fill=sym_color, width=1)
            else:
                d.rectangle([cx, sy + 1, cx + 2, sy + 3], fill=sym_color)

    img.save(f"{ASSET_DIR}/board_bg.png")


def make_worm_head(size, body_color, name):
    """Better worm head with gradient shading and eyes."""
    total = size + 6  # glow padding
    img = Image.new("RGBA", (total, total), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    cx, cy = total // 2, total // 2
    r = size // 2

    # Soft glow
    for i in range(3, 0, -1):
        alpha = int(30 * (1 - i / 3))
        circle(d, cx, cy, r + i, fill=(*body_color, alpha))

    # Body
    darker = tuple(max(c - 30, 0) for c in body_color)
    circle(d, cx, cy, r, fill=(*darker, 255))
    circle(d, cx, cy, r - 2, fill=(*body_color, 255))

    # Top highlight
    lighter = tuple(min(c + 50, 255) for c in body_color)
    d.ellipse([cx - r + 4, cy - r + 3, cx + r - 4, cy - 2], fill=(*lighter, 60))

    # Eyes
    eye_r = max(size // 7, 3)
    pupil_r = max(eye_r * 2 // 3, 2)
    eye_y = cy - size // 7

    for ex in [cx - size // 4, cx + size // 4]:
        # White
        circle(d, ex, eye_y, eye_r, fill=(240, 240, 240, 255))
        # Pupil
        circle(d, ex, eye_y, pupil_r, fill=(20, 20, 20, 255))
        # Glint
        circle(d, ex - 1, eye_y - 1, max(pupil_r // 2, 1), fill=(255, 255, 255, 200))

    img.save(f"{ASSET_DIR}/worm_head_{name}.png")


def make_worm_segment(size, color, name):
    """Better worm segment with shading."""
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    cx, cy = size // 2, size // 2
    r = size // 2 - 1

    # Darker edge
    darker = tuple(max(c - 25, 0) for c in color)
    circle(d, cx, cy, r, fill=(*darker, 240))

    # Main body
    circle(d, cx, cy, r - 2, fill=(*color, 245))

    # Top highlight
    lighter = tuple(min(c + 40, 255) for c in color)
    d.ellipse([cx - r + 4, cy - r + 3, cx + r - 4, cy - 2], fill=(*lighter, 50))

    img.save(f"{ASSET_DIR}/worm_segment_{name}.png")


# Gruvbox colors
COLORS = {
    "green": (152, 151, 26),
    "red": (204, 36, 29),
    "yellow": (215, 153, 33),
    "blue": (69, 133, 136),
    "purple": (177, 98, 134),
    "aqua": (104, 157, 106),
    "orange": (214, 93, 14),
}


if __name__ == "__main__":
    # Food
    for fn, name in [
        (make_cherry, "cherry"), (make_banana, "banana"), (make_apple, "apple"),
        (make_grape, "grape"), (make_watermelon, "watermelon"),
        (make_strawberry, "strawberry"), (make_donut, "donut"),
        (make_cookie, "cookie"),
    ]:
        fn()
        print(f"  food_{name}.png")

    # Board bg
    make_board_bg()
    print("  board_bg.png")

    # Worm heads + segments (better quality)
    for name, color in COLORS.items():
        make_worm_head(40, color, name)
        make_worm_segment(32, color, name)
        print(f"  worm_{name} head+segment")

    print("Done!")
