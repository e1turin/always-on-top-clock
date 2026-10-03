#!/usr/bin/env python3
"""Generate dark and light PNG/ICNS icons for all PiP Clock applications.

This maintenance script requires Pillow. The normal application build uses the
generated files under assets/icons and does not invoke this script.
"""

from __future__ import annotations

import math
from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter

PROJECT_ROOT = Path(__file__).resolve().parent.parent
OUTPUT_DIR = PROJECT_ROOT / "assets" / "icons"
CANVAS_SIZE = 1024
BODY_BOUNDS = (72, 72, 952, 952)

ACCENTS = {
    "clock": "#4AA3FF",
    "vertical": "#9B7BFF",
    "pomodoro": "#F25555",
    "stopwatch": "#39C98A",
}

ICON_SIZES = ((16, 16), (32, 32), (64, 64), (128, 128), (256, 256), (512, 512), (1024, 1024))


def hex_color(value: str, alpha: int = 255) -> tuple[int, int, int, int]:
    value = value.lstrip("#")
    red = int(value[0:2], 16)
    green = int(value[2:4], 16)
    blue = int(value[4:6], 16)
    return red, green, blue, alpha


def create_base(variant: str, accent: str) -> tuple[Image.Image, ImageDraw.ImageDraw, tuple[int, ...], tuple[int, ...]]:
    canvas = Image.new("RGBA", (CANVAS_SIZE, CANVAS_SIZE), (0, 0, 0, 0))

    shadow = Image.new("RGBA", canvas.size, (0, 0, 0, 0))
    ImageDraw.Draw(shadow).rounded_rectangle(
        (84, 94, 940, 950), radius=204, fill=(0, 0, 0, 92 if variant == "dark" else 58)
    )
    canvas.alpha_composite(shadow.filter(ImageFilter.GaussianBlur(34)))

    mask = Image.new("L", canvas.size, 0)
    ImageDraw.Draw(mask).rounded_rectangle(BODY_BOUNDS, radius=198, fill=255)

    if variant == "dark":
        top, bottom = (31, 35, 45), (8, 10, 16)
        foreground = (246, 248, 252, 255)
        secondary = (163, 171, 186, 255)
    else:
        top, bottom = (255, 255, 255), (224, 229, 237)
        foreground = (25, 29, 37, 255)
        secondary = (89, 98, 113, 255)

    gradient = Image.new("RGBA", canvas.size)
    gradient_pixels = gradient.load()
    if gradient_pixels is None:
        raise RuntimeError("failed to access icon gradient pixels")
    for y in range(CANVAS_SIZE):
        progress = y / (CANVAS_SIZE - 1)
        color = tuple(round(top[i] + (bottom[i] - top[i]) * progress) for i in range(3)) + (255,)
        for x in range(CANVAS_SIZE):
            gradient_pixels[x, y] = color
    canvas.alpha_composite(Image.composite(gradient, Image.new("RGBA", canvas.size), mask))

    draw = ImageDraw.Draw(canvas, "RGBA")
    draw.rounded_rectangle(BODY_BOUNDS, radius=198, outline=hex_color(accent, 205), width=11)
    draw.rounded_rectangle((91, 91, 933, 933), radius=180, outline=(255, 255, 255, 20), width=4)
    return canvas, draw, foreground, secondary


def draw_ticks(
    draw: ImageDraw.ImageDraw,
    center: tuple[int, int],
    inner_radius: int,
    outer_radius: int,
    color: tuple[int, ...],
    count: int = 12,
) -> None:
    for index in range(count):
        angle = math.radians(index * 360 / count - 90)
        start = (
            center[0] + math.cos(angle) * inner_radius,
            center[1] + math.sin(angle) * inner_radius,
        )
        end = (
            center[0] + math.cos(angle) * outer_radius,
            center[1] + math.sin(angle) * outer_radius,
        )
        width = 20 if index % 3 == 0 else 11
        draw.line((start, end), fill=color, width=width)


def draw_clock(draw: ImageDraw.ImageDraw, accent: str, foreground: tuple[int, ...], secondary: tuple[int, ...]) -> None:
    accent_color = hex_color(accent)
    center = (512, 512)
    draw.ellipse((205, 205, 819, 819), fill=hex_color(accent, 28), outline=accent_color, width=38)
    draw_ticks(draw, center, 235, 265, secondary)
    draw.line((512, 512, 512, 326), fill=foreground, width=34)
    draw.line((512, 512, 659, 594), fill=foreground, width=34)
    draw.ellipse((480, 480, 544, 544), fill=accent_color)


SEGMENTS = {
    "0": "abcedf",
    "1": "bc",
    "2": "abged",
    "3": "abgcd",
    "4": "fgbc",
    "5": "afgcd",
    "6": "afgecd",
    "7": "abc",
    "8": "abcdefg",
    "9": "abfgcd",
}


def draw_digit(
    draw: ImageDraw.ImageDraw,
    digit: str,
    x: int,
    y: int,
    width: int,
    height: int,
    thickness: int,
    color: tuple[int, ...],
) -> None:
    middle = y + height // 2
    right = x + width
    bottom = y + height
    half = thickness // 2
    radius = max(3, thickness // 2)
    segment_bounds = {
        "a": (x + thickness, y, right - thickness, y + thickness),
        "b": (right - thickness, y + thickness, right, middle - half),
        "c": (right - thickness, middle + half, right, bottom - thickness),
        "d": (x + thickness, bottom - thickness, right - thickness, bottom),
        "e": (x, middle + half, x + thickness, bottom - thickness),
        "f": (x, y + thickness, x + thickness, middle - half),
        "g": (x + thickness, middle - half, right - thickness, middle + half),
    }
    for segment in SEGMENTS[digit]:
        draw.rounded_rectangle(segment_bounds[segment], radius=radius, fill=color)


def draw_vertical(draw: ImageDraw.ImageDraw, accent: str, foreground: tuple[int, ...], secondary: tuple[int, ...]) -> None:
    draw.rounded_rectangle((194, 175, 830, 486), radius=92, fill=hex_color(accent, 32))
    draw.rounded_rectangle((194, 538, 830, 849), radius=92, fill=hex_color(accent, 18))

    digit_width = 174
    digit_height = 252
    gap = 62
    start_x = (CANVAS_SIZE - digit_width * 2 - gap) // 2
    for row, value in enumerate(("12", "34")):
        y = 205 + row * 363
        color = foreground
        draw_digit(draw, value[0], start_x, y, digit_width, digit_height, 30, color)
        draw_digit(draw, value[1], start_x + digit_width + gap, y, digit_width, digit_height, 30, color)

    draw.rounded_rectangle((469, 493, 555, 531), radius=19, fill=secondary)


def draw_pomodoro(draw: ImageDraw.ImageDraw, accent: str, foreground: tuple[int, ...], secondary: tuple[int, ...]) -> None:
    tomato = hex_color(accent)
    leaf = (67, 190, 103, 255)
    draw.ellipse((206, 298, 818, 842), fill=tomato)
    draw.ellipse((242, 260, 530, 590), fill=tomato)
    draw.ellipse((494, 260, 782, 590), fill=tomato)

    draw.polygon(((512, 316), (410, 215), (475, 346), (320, 293), (445, 396)), fill=leaf)
    draw.polygon(((512, 316), (615, 215), (550, 346), (704, 293), (579, 396)), fill=leaf)
    draw.rounded_rectangle((491, 166, 533, 323), radius=20, fill=leaf)

    draw.ellipse((340, 420, 684, 764), fill=(255, 255, 255, 30), outline=foreground, width=24)
    draw_ticks(draw, (512, 592), 122, 146, (255, 255, 255, 205), count=8)
    draw.line((512, 592, 512, 483), fill=foreground, width=27)
    draw.line((512, 592, 603, 638), fill=foreground, width=27)
    draw.ellipse((488, 568, 536, 616), fill=foreground)



def draw_stopwatch(draw: ImageDraw.ImageDraw, accent: str, foreground: tuple[int, ...], secondary: tuple[int, ...]) -> None:
    accent_color = hex_color(accent)
    draw.rounded_rectangle((428, 143, 596, 232), radius=30, fill=foreground)
    draw.rounded_rectangle((471, 112, 553, 177), radius=25, fill=accent_color)
    draw.rounded_rectangle((722, 246, 805, 321), radius=28, fill=foreground)
    draw.line((732, 285, 681, 346), fill=foreground, width=32)

    draw.ellipse((190, 236, 834, 880), fill=hex_color(accent, 28), outline=accent_color, width=42)
    draw.ellipse((246, 292, 778, 824), outline=foreground, width=18)
    draw_ticks(draw, (512, 558), 208, 239, secondary)
    draw.line((512, 558, 618, 395), fill=foreground, width=34)
    draw.line((512, 558, 512, 702), fill=foreground, width=25)
    draw.ellipse((474, 520, 550, 596), fill=accent_color)


DRAWERS = {
    "clock": draw_clock,
    "vertical": draw_vertical,
    "pomodoro": draw_pomodoro,
    "stopwatch": draw_stopwatch,
}


def write_icns(master: Image.Image, output: Path) -> None:
    master.save(output, format="ICNS", sizes=ICON_SIZES)


def main() -> None:
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    for app, drawer in DRAWERS.items():
        for variant in ("dark", "light"):
            accent = ACCENTS[app]
            image, draw, foreground, secondary = create_base(variant, accent)
            drawer(draw, accent, foreground, secondary)

            stem = f"{app}-{variant}"
            png_path = OUTPUT_DIR / f"{stem}.png"
            icns_path = OUTPUT_DIR / f"{stem}.icns"
            image.save(png_path)
            write_icns(image, icns_path)
            print(f"Created {png_path.relative_to(PROJECT_ROOT)}")
            print(f"Created {icns_path.relative_to(PROJECT_ROOT)}")


if __name__ == "__main__":
    main()
