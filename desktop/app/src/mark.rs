// SPDX-License-Identifier: GPL-3.0-or-later
//! The Panal mark (`assets/brand/icon.svg`): a honeycomb cell, the link
//! between the phone and the PC, in electric cyan around a violet core, on
//! a deep-space tile with a cyan glow (the Nova palette). Drawn in code at
//! any pixel size: the tray, window and toast icons at runtime, and the
//! executable's icon at build time (`build.rs` includes this file), so every
//! size is sharp without bitmap assets. At 32 px and below the strokes are
//! heavier and the tile fills more of the square.

/// Deep-space tile, its glow, the cyan rim, the cell and its core
/// (docs/design/tokens.json, Nova palette).
const TILE: [f32; 3] = rgb(0x07, 0x0A, 0x12);
const GLOW: [f32; 3] = rgb(0x0A, 0x3B, 0x4C);
const RIM: [f32; 3] = rgb(0x5C, 0xE1, 0xFF);
const CELL: [f32; 3] = rgb(0x5C, 0xE1, 0xFF);
const CORE: [f32; 3] = rgb(0xB3, 0x88, 0xFF);

/// The cell's and the core's circumradius in the 100×100 design grid
/// (pointy-top hexagons around the centre).
const CELL_RADIUS: f32 = 30.0;

/// Supersampling grid per pixel (SS×SS samples) for smooth edges.
const SS: usize = 4;

const fn rgb(r: u8, g: u8, b: u8) -> [f32; 3] {
    [r as f32, g as f32, b as f32]
}

/// Sizes in the design grid: the small sizes are drawn bolder.
struct Geometry {
    inset: f32,
    corner: f32,
    stroke: f32,
    core: f32,
}

impl Geometry {
    fn for_size(size: usize) -> Self {
        if size <= 32 {
            Geometry { inset: 2.0, corner: 20.0, stroke: 11.0, core: 13.0 }
        } else {
            Geometry { inset: 4.0, corner: 22.0, stroke: 9.0, core: 11.0 }
        }
    }
}

/// A pointy-top hexagon's corners around the centre of the design grid.
fn hexagon(radius: f32) -> Vec<(f32, f32)> {
    (0..=6)
        .map(|k| {
            let a = (-90.0 + 60.0 * (k % 6) as f32).to_radians();
            (50.0 + radius * a.cos(), 50.0 + radius * a.sin())
        })
        .collect()
}

/// Whether (x, y) is inside a convex polygon given as a closed polyline.
fn inside(polygon: &[(f32, f32)], (x, y): (f32, f32)) -> bool {
    polygon.windows(2).all(|w| (w[1].0 - w[0].0) * (y - w[0].1) - (w[1].1 - w[0].1) * (x - w[0].0) >= 0.0)
}

/// Renders the mark as straight-alpha RGBA, row by row, `size`×`size`.
pub fn render(size: usize) -> Vec<u8> {
    let g = Geometry::for_size(size);
    let unit = 100.0 / size as f32;
    let cell = hexagon(CELL_RADIUS);
    let core = hexagon(g.core);
    // The rim is at least about a pixel wide, so the tile keeps an edge on dark taskbars.
    let rim = (1.5f32).max(unit * 0.6);

    let mut out = vec![0u8; size * size * 4];
    for py in 0..size {
        for px in 0..size {
            let mut color = [0f32; 3];
            let mut covered = 0usize;
            for sy in 0..SS {
                for sx in 0..SS {
                    // The sample in the design grid.
                    let x = (px as f32 + (sx as f32 + 0.5) / SS as f32) * unit;
                    let y = (py as f32 + (sy as f32 + 0.5) / SS as f32) * unit;
                    let edge = rounded_square_distance(x, y, g.inset, g.corner);
                    if edge > 0.0 {
                        continue;
                    }
                    covered += 1;
                    let (dx, dy) = (x - 50.0, y - 50.0);
                    let paint = if inside(&core, (x, y)) {
                        CORE
                    } else if distance_to(&cell, (x, y)) <= g.stroke / 2.0 {
                        CELL
                    } else if -edge <= rim {
                        mix(TILE, RIM, 0.34)
                    } else {
                        // The glow fades from the middle to the tile color at 40 units.
                        let t = ((dx * dx + dy * dy).sqrt() / 40.0).min(1.0);
                        mix(GLOW, TILE, t)
                    };
                    for c in 0..3 {
                        color[c] += paint[c];
                    }
                }
            }
            if covered == 0 {
                continue;
            }
            let i = (py * size + px) * 4;
            for c in 0..3 {
                out[i + c] = (color[c] / covered as f32).round() as u8;
            }
            out[i + 3] = ((covered as f32 / (SS * SS) as f32) * 255.0).round() as u8;
        }
    }
    out
}

fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

/// How far (x, y) is outside the rounded square from `inset` to
/// `100 - inset` with corner radius `r` (negative inside).
fn rounded_square_distance(x: f32, y: f32, inset: f32, r: f32) -> f32 {
    let (lo, hi) = (inset + r, 100.0 - inset - r);
    let qx = (x.clamp(lo, hi) - x).abs();
    let qy = (y.clamp(lo, hi) - y).abs();
    if x > lo && x < hi || y > lo && y < hi {
        // Along a straight side.
        let to_x = (x - inset).min(100.0 - inset - x);
        let to_y = (y - inset).min(100.0 - inset - y);
        return -to_x.min(to_y);
    }
    (qx * qx + qy * qy).sqrt() - r
}

fn distance_to(polyline: &[(f32, f32)], p: (f32, f32)) -> f32 {
    polyline.windows(2).map(|w| dist_to_segment(p, w[0], w[1])).fold(f32::INFINITY, f32::min)
}

fn dist_to_segment(p: (f32, f32), a: (f32, f32), b: (f32, f32)) -> f32 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let len2 = dx * dx + dy * dy;
    let t = if len2 == 0.0 { 0.0 } else { (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / len2).clamp(0.0, 1.0) };
    let (qx, qy) = (a.0 + t * dx, a.1 + t * dy);
    ((p.0 - qx).powi(2) + (p.1 - qy).powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pixel(img: &[u8], size: usize, x: usize, y: usize) -> [u8; 4] {
        let i = (y * size + x) * 4;
        [img[i], img[i + 1], img[i + 2], img[i + 3]]
    }

    #[test]
    fn corners_are_transparent_and_the_core_sits_in_the_middle() {
        let size = 64;
        let img = render(size);
        assert_eq!(img.len(), size * size * 4);
        assert_eq!(pixel(&img, size, 0, 0)[3], 0, "rounded corner");
        let middle = pixel(&img, size, 32, 32);
        assert_eq!(middle, [0xB3, 0x88, 0xFF, 255], "the violet core");
        // Near the top edge, above the cell: the dark tile.
        let tile = pixel(&img, size, 32, 6);
        assert!(tile[0] < 0x40 && tile[3] == 255, "{tile:?}");
    }

    #[test]
    fn the_cell_is_drawn_on_both_sides() {
        let size = 128;
        let img = render(size);
        let cyan = |p: [u8; 4]| p == [0x5C, 0xE1, 0xFF, 255];
        let left = (16..44).any(|x| cyan(pixel(&img, size, x, 64)));
        let right = (84..112).any(|x| cyan(pixel(&img, size, x, 64)));
        assert!(left && right);
    }

    #[test]
    fn renders_every_tray_size() {
        for size in [16, 20, 24, 32, 48, 64, 256] {
            let img = render(size);
            assert!(img.chunks(4).any(|p| p[3] == 255), "{size}px has an opaque tile");
        }
    }
}
