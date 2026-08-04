//! Tiny dependency-free AirBattery logo renderer for the Windows system tray.

const SIZE: u32 = 32;
const WHITE: [u8; 4] = [255, 255, 255, 255];
const CONNECTED: [u8; 4] = [86, 211, 100, 255];
const LOW: [u8; 4] = [255, 159, 10, 255];
const CRITICAL: [u8; 4] = [255, 69, 58, 255];
const UNAVAILABLE: [u8; 4] = [142, 142, 147, 255];

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

fn draw_status_ring(buffer: &mut [u8], color: [u8; 4]) {
    let center = 16.0_f32;
    let radius = 14.0_f32;
    for y in 0..SIZE as i32 {
        for x in 0..SIZE as i32 {
            let dx = x as f32 + 0.5 - center;
            let dy = y as f32 + 0.5 - center;
            let distance = (dx * dx + dy * dy).sqrt();
            if (radius - 1.35..=radius + 0.35).contains(&distance) {
                set_pixel(buffer, x, y, color);
            }
        }
    }
}

fn draw_battery_logo(buffer: &mut [u8]) {
    // Battery cap.
    fill_rect(buffer, 14, 4, 4, 2, WHITE);
    fill_rect(buffer, 13, 5, 6, 2, WHITE);

    // Rounded-enough 12×20 battery outline at tray scale.
    fill_rect(buffer, 10, 8, 2, 17, WHITE);
    fill_rect(buffer, 20, 8, 2, 17, WHITE);
    fill_rect(buffer, 12, 6, 8, 2, WHITE);
    fill_rect(buffer, 12, 25, 8, 2, WHITE);
    set_pixel(buffer, 11, 7, WHITE);
    set_pixel(buffer, 20, 7, WHITE);
    set_pixel(buffer, 11, 25, WHITE);
    set_pixel(buffer, 20, 25, WHITE);

    // Bluetooth glyph centered inside the battery.
    draw_line(buffer, 16, 10, 16, 23, 1, WHITE);
    draw_line(buffer, 16, 10, 19, 13, 1, WHITE);
    draw_line(buffer, 19, 13, 16, 16, 1, WHITE);
    draw_line(buffer, 16, 16, 19, 19, 1, WHITE);
    draw_line(buffer, 19, 19, 16, 23, 1, WHITE);
    draw_line(buffer, 16, 16, 13, 13, 1, WHITE);
    draw_line(buffer, 16, 16, 13, 20, 1, WHITE);
}

fn status_color(percentage: u8) -> [u8; 4] {
    match percentage.min(100) {
        0..=10 => CRITICAL,
        11..=20 => LOW,
        _ => CONNECTED,
    }
}

/// Renders the approved battery-and-Bluetooth mark with a battery-level status ring.
#[must_use]
pub fn render_percentage_icon(percentage: u8) -> Vec<u8> {
    let mut rgba = vec![0_u8; (SIZE * SIZE * 4) as usize];
    draw_status_ring(&mut rgba, status_color(percentage));
    draw_battery_logo(&mut rgba);
    rgba
}

/// Renders the same mark with a neutral ring when battery data is unavailable.
#[must_use]
pub fn render_unavailable_icon() -> Vec<u8> {
    let mut rgba = vec![0_u8; (SIZE * SIZE * 4) as usize];
    draw_status_ring(&mut rgba, UNAVAILABLE);
    draw_battery_logo(&mut rgba);
    rgba
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
    }

    #[test]
    fn percentage_renderer_clamps_values_above_one_hundred() {
        assert_eq!(render_percentage_icon(255), render_percentage_icon(100));
    }

    #[test]
    fn tray_ring_uses_connected_low_and_critical_status_colors() {
        assert_eq!(pixel(&render_percentage_icon(80), 16, 1), CONNECTED);
        assert_eq!(pixel(&render_percentage_icon(15), 16, 1), LOW);
        assert_eq!(pixel(&render_percentage_icon(5), 16, 1), CRITICAL);
        assert_eq!(pixel(&render_unavailable_icon(), 16, 1), UNAVAILABLE);
    }

    #[test]
    fn tray_icon_always_contains_the_white_battery_mark() {
        assert_eq!(pixel(&render_percentage_icon(80), 14, 5), WHITE);
        assert_eq!(pixel(&render_unavailable_icon(), 16, 16), WHITE);
    }
}
