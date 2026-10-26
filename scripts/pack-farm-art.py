"""Pack farm art into static/farm/.

Minecraft-*inspired* blocky look using only CC0 packs (no Mojang textures):
  - Kenney Voxel Pack: grass/dirt/fence/wheat/sky/items
  - josehzz Farming crops 16x16: on-field crop growth stages
  - ChikenwingJJA Simple Farm Tiles: watering-can + seed-dot UI
"""
from __future__ import annotations

import random
from pathlib import Path

from PIL import Image, ImageDraw, ImageEnhance

ROOT = Path(__file__).resolve().parents[1]
VOX = ROOT / "tmp-assets" / "voxel-pack" / "PNG"
CROPS_SHEET = ROOT / "tmp-assets" / "farming-crops-16" / "Crop_Spritesheet.png"
SIMPLE = ROOT / "tmp-assets" / "farm_tiles.png"
DST = ROOT / "static" / "farm"

# Crop_Spritesheet: 2 crops per row; col0/6 = portrait, then 5 stages, last = seed dots.
# Index order matches README (Turnip=0 … Sunflower=19).
CROP_IDX = {
    "lettuce": 2,  # Cucumber (leafy green stand-in)
    "carrot": 14,  # Potato (root crop stand-in)
    "tomato": 4,
    "strawberry": 12,
    "apple": 16,  # Orange (tree-fruit stand-in)
    "corn": 18,
}


def vox_tile(name: str) -> Image.Image:
    return Image.open(VOX / "Tiles" / name).convert("RGBA")


def vox_item(name: str) -> Image.Image:
    return Image.open(VOX / "Items" / name).convert("RGBA")


def up(im: Image.Image, scale: int = 4) -> Image.Image:
    return im.resize((im.width * scale, im.height * scale), Image.NEAREST)


def crop_opaque(im: Image.Image, pad: int = 1) -> Image.Image:
    bbox = im.getbbox()
    if not bbox:
        return im
    x0, y0, x1, y1 = bbox
    return im.crop(
        (
            max(0, x0 - pad),
            max(0, y0 - pad),
            min(im.width, x1 + pad),
            min(im.height, y1 + pad),
        )
    )


def sheet_cell(sheet: Image.Image, crop_i: int, stage: int) -> Image.Image:
    """stage 0=portrait, 1=mature plant … 4=sprout, 5=seed dots."""
    row = crop_i // 2
    base = 6 if crop_i % 2 else 0
    col = base + stage
    return sheet.crop((col * 16, row * 16, col * 16 + 16, row * 16 + 16))


def make_farmland(dirt: Image.Image, size: int = 64) -> Image.Image:
    """Tilled brown plot with MC-like furrows (procedural, from dirt texture)."""
    base = dirt.resize((size, size), Image.NEAREST)
    base = ImageEnhance.Brightness(base).enhance(0.88)
    base = ImageEnhance.Color(base).enhance(0.92)
    px = base.load()
    # Darker horizontal furrows every few pixels
    for y in range(4, size - 4, 8):
        for x in range(2, size - 2):
            r, g, b, a = px[x, y]
            px[x, y] = (max(0, r - 28), max(0, g - 22), max(0, b - 18), a)
            if y + 1 < size - 2:
                r2, g2, b2, a2 = px[x, y + 1]
                px[x, y + 1] = (min(255, r2 + 12), min(255, g2 + 8), min(255, b2 + 6), a2)
    # Soft block outline
    draw = ImageDraw.Draw(base)
    draw.rectangle((0, 0, size - 1, size - 1), outline=(62, 42, 24, 220))
    draw.rectangle((1, 1, size - 2, size - 2), outline=(110, 78, 48, 90))
    return base


def make_wet(soil: Image.Image) -> Image.Image:
    wet = ImageEnhance.Brightness(soil).enhance(0.72)
    wet = ImageEnhance.Color(wet).enhance(0.75)
    # Cooler tint
    r, g, b, a = wet.split()
    b = ImageEnhance.Brightness(b).enhance(1.15)
    g = ImageEnhance.Brightness(g).enhance(0.95)
    return Image.merge("RGBA", (r, g, b, a))


def make_locked(soil: Image.Image) -> Image.Image:
    g = ImageEnhance.Color(soil).enhance(0.2)
    return ImageEnhance.Brightness(g).enhance(0.85)


def simple_cell(x: int, y: int) -> Image.Image:
    sheet = Image.open(SIMPLE).convert("RGBA")
    return sheet.crop((x, y, x + 16, y + 16))


def build_scene() -> Image.Image:
    """Top-down block field: sky strip + grass tiles + dirt path + wood fence."""
    rng = random.Random(42)
    tile = 32
    cols, rows = 22, 16
    sky_h = 72
    w, h = cols * tile, rows * tile + sky_h
    canvas = Image.new("RGBA", (w, h), (0, 0, 0, 255))

    # Sky gradient
    sky = Image.new("RGBA", (w, sky_h), (0, 0, 0, 255))
    sd = ImageDraw.Draw(sky)
    for y in range(sky_h):
        t = y / max(1, sky_h - 1)
        r = int(120 + 40 * t)
        g = int(180 + 30 * t)
        b = int(235 - 10 * t)
        sd.line([(0, y), (w, y)], fill=(r, g, b, 255))
    # Sun + clouds
    sun = vox_tile("grass_top.png")  # placeholder size; use Other/sun
    try:
        sun = Image.open(VOX / "Other" / "sun.png").convert("RGBA")
        sun = sun.resize((28, 28), Image.NEAREST)
        sky.alpha_composite(sun, (w - 56, 12))
    except FileNotFoundError:
        sd.ellipse((w - 54, 10, w - 26, 38), fill=(255, 230, 90, 255))
    try:
        clouds = Image.open(VOX / "Other" / "clouds.png").convert("RGBA")
        clouds = clouds.resize((120, 40), Image.NEAREST)
        sky.alpha_composite(clouds, (40, 18))
        sky.alpha_composite(clouds, (220, 8))
    except FileNotFoundError:
        pass
    canvas.paste(sky, (0, 0))

    grass = vox_tile("grass_top.png").resize((tile, tile), Image.NEAREST)
    dirt = vox_tile("dirt.png").resize((tile, tile), Image.NEAREST)
    fence = vox_tile("fence_wood.png").resize((tile, tile), Image.NEAREST)
    leaves = vox_tile("leaves.png").resize((tile, tile), Image.NEAREST)
    trunk = vox_tile("trunk_side.png").resize((tile, tile), Image.NEAREST)
    wheat = [
        vox_tile(f"wheat_stage{i}.png").resize((tile, tile), Image.NEAREST) for i in (2, 3, 4)
    ]
    tuft = [
        vox_tile(f"grass{i}.png").resize((tile, tile), Image.NEAREST) for i in (1, 2, 3, 4)
    ]

    # Grass underpaint so cover-crop never shows black
    field = Image.new("RGBA", (cols * tile, rows * tile), (98, 168, 58, 255))
    for gy in range(rows):
        for gx in range(cols):
            g = grass.copy()
            if rng.random() < 0.1:
                g = ImageEnhance.Brightness(g).enhance(0.9 + rng.random() * 0.18)
            field.paste(g, (gx * tile, gy * tile))

    # Dirt ring just inside the fence (path framing the yard)
    for gx in range(1, cols - 1):
        field.paste(dirt, (gx * tile, 1 * tile))
        field.paste(dirt, (gx * tile, (rows - 2) * tile))
    for gy in range(2, rows - 2):
        field.paste(dirt, (1 * tile, gy * tile))
        field.paste(dirt, ((cols - 2) * tile, gy * tile))

    # Outer fence ring
    for gx in range(cols):
        field.paste(fence, (gx * tile, 0))
        field.paste(fence, (gx * tile, (rows - 1) * tile))
    for gy in range(1, rows - 1):
        field.paste(fence, (0, gy * tile))
        field.paste(fence, ((cols - 1) * tile, gy * tile))

    # Edge wheat / tufts — leave center clear for the 3×3 plot UI
    for gy in range(2, rows - 2):
        for gx in range(2, cols - 2):
            in_center = 6 <= gx <= 15 and 4 <= gy <= 11
            if in_center:
                continue
            if rng.random() < 0.18:
                dec = wheat[rng.randrange(len(wheat))] if rng.random() < 0.4 else tuft[rng.randrange(len(tuft))]
                field.alpha_composite(dec, (gx * tile, gy * tile))

    # Corner trees
    for tx, ty in ((3, 3), (cols - 5, 3), (3, rows - 5), (cols - 5, rows - 5)):
        field.paste(trunk, (tx * tile, (ty + 1) * tile))
        field.alpha_composite(leaves, (tx * tile, ty * tile))
        field.alpha_composite(leaves, ((tx + 1) * tile, ty * tile))
        field.alpha_composite(leaves, (tx * tile, (ty - 1) * tile))

    canvas.paste(field, (0, sky_h))
    return canvas.resize((w * 2, h * 2), Image.NEAREST)


def main() -> None:
    if not (VOX / "Tiles").is_dir() or not CROPS_SHEET.is_file() or not SIMPLE.is_file():
        raise SystemExit(
            "Missing assets: need tmp-assets/voxel-pack, farming-crops-16, farm_tiles.png"
        )

    (DST / "crops").mkdir(parents=True, exist_ok=True)
    (DST / "ui").mkdir(parents=True, exist_ok=True)

    scene = build_scene()
    scene.save(DST / "scene.png")
    vox_tile("grass_top.png").resize((64, 64), Image.NEAREST).save(DST / "grass.png")

    dirt = vox_tile("dirt.png")
    soil = make_farmland(dirt, 96)
    soil.save(DST / "soil-mound.png")
    soil.save(DST / "soil-patch.png")
    soil.save(DST / "soil-plot.png")
    soil.save(DST / "soil-plot-b.png")
    make_wet(soil).save(DST / "soil-wet.png")
    make_locked(soil).save(DST / "soil-locked.png")

    sheet = Image.open(CROPS_SHEET).convert("RGBA")
    # Wilted: desaturated sprout
    wilt = sheet_cell(sheet, CROP_IDX["lettuce"], 4)
    wilt = ImageEnhance.Color(up(wilt, 5)).enhance(0.12)
    wilt = ImageEnhance.Brightness(wilt).enhance(0.75)
    wilt.save(DST / "crops" / "wilted.png")

    for crop, idx in CROP_IDX.items():
        # Growth: sheet goes mature→young after portrait, so reverse for sprout→ready
        sprout = up(sheet_cell(sheet, idx, 4), 5)
        grow = up(sheet_cell(sheet, idx, 3), 5)
        ready = up(sheet_cell(sheet, idx, 1), 5)
        portrait = up(sheet_cell(sheet, idx, 0), 5)
        seed_dots = up(sheet_cell(sheet, idx, 5), 5)
        sprout.save(DST / "crops" / f"{crop}-sprout.png")
        grow.save(DST / "crops" / f"{crop}-grow.png")
        ready.save(DST / "crops" / f"{crop}-ready.png")
        seed_dots.save(DST / "crops" / f"{crop}-seed.png")
        portrait.save(DST / "crops" / f"{crop}-crate.png")

    # UI chrome
    can = crop_opaque(simple_cell(80, 0))
    up(can, 5).save(DST / "ui" / "water-can.png")
    up(crop_opaque(simple_cell(64, 0)), 5).save(DST / "ui" / "seed-bag.png")
    up(vox_item("seed.png").resize((32, 32), Image.NEAREST), 3).save(DST / "ui" / "sunflower.png")
    up(vox_tile("wood.png").resize((32, 32), Image.NEAREST), 3).save(DST / "ui" / "barrel.png")
    up(vox_item("bowl.png").resize((32, 32), Image.NEAREST), 3).save(DST / "ui" / "bucket.png")
    up(vox_item("wheat.png").resize((32, 32), Image.NEAREST), 3).save(DST / "ui" / "sack.png")
    up(vox_tile("fence_wood.png").resize((48, 32), Image.NEAREST), 3).save(DST / "ui" / "fence.png")
    up(vox_tile("leaves.png").resize((32, 32), Image.NEAREST), 3).save(DST / "ui" / "bush.png")
    up(vox_tile("trunk_side.png").resize((24, 32), Image.NEAREST), 3).save(DST / "ui" / "post.png")
    # tree = trunk under leaves
    tree = Image.new("RGBA", (48, 64), (0, 0, 0, 0))
    tree.alpha_composite(vox_tile("trunk_side.png").resize((20, 28), Image.NEAREST), (14, 36))
    tree.alpha_composite(vox_tile("leaves.png").resize((48, 40), Image.NEAREST), (0, 0))
    up(tree, 3).save(DST / "ui" / "tree.png")

    yard = DST / "yard.png"
    if yard.exists():
        yard.unlink()

    credits = """Farm art — Minecraft-*inspired* blocky look (CC0 / public domain)

NO official Minecraft / Mojang textures were used.

Kenney Voxel Pack (CC0)
  https://kenney.nl/assets/voxel-pack
  https://opengameart.org/content/voxel-pack
  Used for: grass/dirt blocks, wood fence, wheat decor, sky/sun/clouds,
  bowl/seed/wheat UI icons, procedural farmland base.

Farming crops 16x16 by josehzz (CC0)
  https://opengameart.org/content/farming-crops-16x16
  Used for: on-field crop growth stages, seed tray & harvest icons.
  (lettuce←cucumber, carrot←potato, apple←orange stand-ins)

Simple Farm Tiles by ChikenwingJJA (CC0)
  https://opengameart.org/content/simple-farm-tiles
  Used for: watering-can and seed-bag UI glyphs.

Credit optional. Rebuild: python scripts/pack-farm-art.py
"""
    (DST / "CREDITS.txt").write_text(credits, encoding="utf-8")

    print("packed into", DST)
    print("scene", Image.open(DST / "scene.png").size)


if __name__ == "__main__":
    main()
