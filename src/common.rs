#[derive(Debug, Clone)]
pub enum ColorError {
    FormatErr(String),
    ValueErr(String),
}

pub fn calc_rgb_with_alpha(v: u8, alpha: f32) -> f32 {
    v as f32 * alpha + 255.0 * (1.0 - alpha)
}

pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (u32, u32, u32) {
    calc_rgb_to_hsl(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}
pub fn rgba_to_hsla(r: u8, g: u8, b: u8, a: f32) -> (u32, u32, u32, f32) {
    let (h, s, l) = rgb_to_hsl(r, g, b);
    (h, s, l, a)
}

pub fn calc_rgb_to_hsl(r: f32, g: f32, b: f32) -> (u32, u32, u32) {
    let c_max = r.max(g).max(b);
    let c_min = r.min(g).min(b);
    let delta = c_max - c_min;
    let mut h = if delta == 0.0 {
        0.0
    } else if c_max == r {
        60.0 * ((g - b) / delta % 6.0)
    } else if c_max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };
    if h < 0.0 {
        h += 360.0;
    }
    let l = (c_max + c_min) / 2.0;
    let s = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    };
    (
        h.round() as u32,
        (s * 100.0).round() as u32,
        (l * 100.0).round() as u32,
    )
}

pub fn hsl_to_rgb(h: u32, s: u32, l: u32) -> (u8, u8, u8) {
    let s = s as f32 / 100.0;
    let l = l as f32 / 100.0;
    let c = (1.0 - (l * 2.0 - 1.0).abs()) * s;
    let x = c * (1.0 - ((h as f32 / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (mut r, mut g, mut b) = match h {
        n if n < 60 => (c, x, 0.0),
        n if (60..120).contains(&n) => (x, c, 0.0),
        n if (120..180).contains(&n) => (0.0, c, x),
        n if (180..240).contains(&n) => (0.0, x, c),
        n if (240..300).contains(&n) => (x, 0.0, c),
        n if (300..360).contains(&n) => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };
    r = (r + m) * 255.0;
    g = (g + m) * 255.0;
    b = (b + m) * 255.0;
    (r.round() as u8, g as u8, b.round() as u8)
}

pub fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (u32, u32, u32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let c_max = r.max(g).max(b);
    let c_min = r.min(g).min(b);
    let delta = c_max - c_min;

    let mut h = if delta == 0.0 {
        0.0
    } else if c_max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if c_max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    if h < 0.0 {
        h += 360.0;
    }

    let s = if c_max == 0.0 { 0.0 } else { delta / c_max };

    let v = c_max;
    (
        h.round() as u32,
        (s * 100.0).round() as u32,
        (v * 100.0).round() as u32,
    )
}

pub fn hsv_to_rgb(h: u32, s: u32, v: u32) -> (u8, u8, u8) {
    let s = s as f32 / 100.0;
    let v = v as f32 / 100.0;
    let c = v * s;
    let x = c * (1.0 - ((h as f32 / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (mut r, mut g, mut b) = match h {
        n if n < 60 => (c, x, 0.0),
        n if (60..120).contains(&n) => (x, c, 0.0),
        n if (120..180).contains(&n) => (0.0, c, x),
        n if (180..240).contains(&n) => (0.0, x, c),
        n if (240..300).contains(&n) => (x, 0.0, c),
        n if (300..360).contains(&n) => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };
    r = (r + m) * 255.0;
    g = (g + m) * 255.0;
    b = (b + m) * 255.0;
    (r as u8, g as u8, b.round() as u8)
}

pub fn rgb_to_cmyk(r: u8, g: u8, b: u8) -> (u8, u8, u8, u8) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let k = 1.0 - r.max(g).max(b);
    let (c, m, y) = if k == 1.0 {
        (0.0, 0.0, 0.0)
    } else {
        (
            (1.0 - r - k) / (1.0 - k),
            (1.0 - g - k) / (1.0 - k),
            (1.0 - b - k) / (1.0 - k),
        )
    };
    (
        (c * 100.0).round() as u8,
        (m * 100.0).round() as u8,
        (y * 100.0).round() as u8,
        (k * 100.0).round() as u8,
    )
}

pub fn cmyk_to_rgb(c: u8, m: u8, y: u8, k: u8) -> (u8, u8, u8) {
    let c = c as f32 / 100.0;
    let m = m as f32 / 100.0;
    let y = y as f32 / 100.0;
    let k = k as f32 / 100.0;
    let t = 1.0 - k;
    let r = ((1.0 - c) * t * 255.0).round() as u8;
    let g = ((1.0 - m) * t * 255.0).round() as u8;
    let b = ((1.0 - y) * t * 255.0).round() as u8;
    (r, g, b)
}

pub fn process_hex(hex_str: &str, chunk_size: usize) -> Vec<u8> {
    hex_str
        .chars()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|c| c.iter().collect::<String>())
        .map(|v| u8::from_str_radix(&v.repeat(2)[0..2], 16))
        .filter_map(|v| v.ok())
        .collect::<Vec<_>>()
}
