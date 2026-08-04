//! Tiny dependency-free AirBattery logo renderer for the Windows system tray.

const SIZE: u32 = 32;
const CONNECTED: [u8; 4] = [52, 199, 89, 255];
const LOW: [u8; 4] = [255, 159, 10, 255];
const CRITICAL: [u8; 4] = [255, 69, 58, 255];
const UNAVAILABLE: [u8; 4] = [142, 142, 147, 255];
const DISCONNECTED: [u8; 4] = [99, 99, 102, 176];

fn set_pixel(buffer: &mut [u8], x: i32, y: i32, color: [u8; 4]) {
    if x < 0 || y < 0 || x >= SIZE as i32 || y >= SIZE as i32 {
        return;
    }
    let index = (((y as u32) * SIZE + x as u32) * 4) as usize;
    buffer[index..index + 4].copy_from_slice(&color);
}

fn fill_rect(buffer: &mut [u8], x: i32, y: i32, width: i32, height: i32, color: [u8; 4]) {
    for dy in 0..height {
        for dx in 0..width {
            set_pixel(buffer, x + dx, y + dy, color);
        }
    }
}

fn draw_line(
    buffer: &mut [u8],
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    thickness: i32,
    color: [u8; 4],
) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;
    let radius = thickness / 2;

    loop {
        for oy in -radius..=radius {
            for ox in -radius..=radius {
                set_pixel(buffer, x0 + ox, y0 + oy, color);
            }
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let doubled = 2 * error;
        if doubled >= dy {
            error += dy;
            x0 += sx;
        }
        if doubled <= dx {
            error += dx;
            y0 += sy;
        }
    }
}

fn draw_battery_logo(buffer: &mut [u8], color: [u8; 4]) {
    // Short cap and slim body match the approved Concept 07 silhouette.
    fill_rect(buffer, 14, 4, 4, 2, color);
    fill_rect(buffer, 13, 5, 6, 2, color);

    fill_rect(buffer, 11, 9, 2, 15, color);
    fill_rect(buffer, 19, 9, 2, 15, color);
    fill_rect(buffer, 13, 7, 6, 2, color);
    fill_rect(buffer, 13, 24, 6, 2, color);
    set_pixel(buffer, 12, 8, color);
    set_pixel(buffer, 19, 8, color);
    set_pixel(buffer, 12, 24, color);
    set_pixel(buffer, 19, 24, color);

    // Centered, simplified Bluetooth mark for 16–32 px readability.
    draw_line(buffer, 16, 11, 16, 22, 1, color);
    draw_line(buffer, 16, 11, 18, 13, 1, color);
    draw_line(buffer, 18, 13, 16, 16, 1, color);
    draw_line(buffer, 16, 16, 18, 19, 1, color);
    draw_line(buffer, 18, 19, 16, 22, 1, color);
    draw_line(buffer, 16, 16, 14, 13, 1, color);
    draw_line(buffer, 16, 16, 14, 19, 1, color);
}

fn status_color(percentage: u8) -> [u8; 4] {
    match percentage.min(100) {
        0..=10 => CRITICAL,
        11..=20 => LOW,
        _ => CONNECTED,
    }
}

fn render_colored_logo(color: [u8; 4]) -> Vec<u8> {
    let mut rgba = vec![0_u8; (SIZE * SIZE * 4) as usize];
    draw_battery_logo(&mut rgba, color);
    rgba
}

/// Renders the approved mark using the battery state as the mark color.
#[must_use]
pub fn render_percentage_icon(percentage: u8) -> Vec<u8> {
    let mut rgba = vec![0_u8; (SIZE * SIZE * 4) as usize];
    draw_battery_logo(&mut rgba, status_color(percentage));
    rgba
}

/// Renders the same mark in neutral gray when battery data is unavailable.
#[must_use]
pub fn render_unavailable_icon() -> Vec<u8> {
    render_colored_logo(UNAVAILABLE)
}

/// Renders a dim neutral mark for a disconnected device.
#[must_use]
pub fn render_disconnected_icon() -> Vec<u8> {
    render_colored_logo(DISCONNECTED)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pixel(buffer: &[u8], x: u32, y: u32) -> [u8; 4] {
        let index = ((y * SIZE + x) * 4) as usize;
        buffer[index..index + 4]
            .try_into()
            .unwrap_or_else(|_| panic!("pixel must contain four channels"))
    }

    #[test]
    fn all_tray_variants_have_the_expected_rgba_size() {
        assert_eq!(render_percentage_icon(79).len(), 32 * 32 * 4);
        assert_eq!(render_unavailable_icon().len(), 32 * 32 * 4);
        assert_eq!(render_disconnected_icon().len(), 32 * 32 * 4);
    }

    #[test]
    fn percentage_renderer_clamps_values_above_one_hundred() {
        assert_eq!(render_percentage_icon(255), render_percentage_icon(100));
    }

    #[test]
    fn the_approved_logo_itself_uses_the_live_status_color() {
        assert_eq!(pixel(&render_percentage_icon(80), 14, 5), CONNECTED);
        assert_eq!(pixel(&render_percentage_icon(15), 14, 5), LOW);
        assert_eq!(pixel(&render_percentage_icon(5), 14, 5), CRITICAL);
        assert_eq!(pixel(&render_unavailable_icon(), 16, 16), UNAVAILABLE);
        assert_eq!(pixel(&render_disconnected_icon(), 16, 16), DISCONNECTED);
    }

    #[test]
    fn tray_icon_has_no_separate_status_ring_or_dot() {
        assert_eq!(pixel(&render_percentage_icon(80), 16, 1), [0, 0, 0, 0]);
        assert_eq!(pixel(&render_unavailable_icon(), 2, 16), [0, 0, 0, 0]);
    }
}
