mod slug;

pub use slug::*;

use ratatui::style::Color;

/// Color palette for report display colors - warm, friendly tones
const REPORT_COLORS: &[Color] = &[
    Color::Rgb(100, 149, 237), // Cornflower blue
    Color::Rgb(143, 188, 143), // Sage green
    Color::Rgb(205, 133, 63),  // Peru/terracotta
    Color::Rgb(147, 112, 219), // Medium purple
    Color::Rgb(240, 128, 128), // Light coral
    Color::Rgb(72, 61, 139),   // Dark slate blue
    Color::Rgb(189, 183, 107), // Khaki
    Color::Rgb(178, 102, 102), // Dusty rose
    Color::Rgb(70, 130, 180),  // Steel blue
    Color::Rgb(102, 178, 102), // Soft green
];

/// Generate a display color from a name hash
/// Returns a Color that can be used for borders and text highlighting
pub fn color_from_name(name: &str) -> Color {
    let hash: u32 = name
        .bytes()
        .fold(0, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
    REPORT_COLORS[(hash as usize) % REPORT_COLORS.len()]
}

/// Parse a hex color string like "#6495ED" into a Color
pub fn parse_hex_color(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(Color::Rgb(r, g, b))
}

/// Get display color for a report - uses explicit color if set, otherwise generates from name
pub fn report_color(color: Option<&str>, name: &str) -> Color {
    color
        .and_then(parse_hex_color)
        .unwrap_or_else(|| color_from_name(name))
}
