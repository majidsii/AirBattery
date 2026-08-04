//! Tiny dependency-free percentage icon renderer for the Windows system tray.

const DIGITS: [[u8; 7]; 10] = [
    [1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 0, 0, 0, 0],
    [1, 1, 0, 1, 1, 0, 1],
    [1, 1, 1, 1, 0, 0, 1],
    [0, 1, 1, 0, 0, 1, 1],
    [1, 0, 1, 1, 0, 1, 1],
    [1, 0, 1, 1, 1, 1, 1],
    [1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 0, 1, 1],
];

fn set_pixel(buffer: &mut [u8], size: u32, x: u32, y: u32) {
    if x >= size || y >= size {
        return;
    }
    let index = ((y * size + x) * 4) as usize;
    buffer[index..index + 4].copy_from_slice(&[255, 255, 255, 255]);
}

fn fill_rect(buffer: &mut [u8], size: u32, x: u32, y: u32, width: u32, height: u32) {
    for dy in 0..height {
        for dx in 0..width {
            set_pixel(buffer, size, x + dx, y + dy);
        }
    }
}

fn draw_digit(buffer: &mut [u8], size: u32, digit: u8, origin_x: u32) {
    let segments = DIGITS[usize::from(digit)];
    let x = origin_x;
    let y = 7;
    let width = 7;
    let height = 17;
    let thickness = 2;
    if segments[0] == 1 {
        fill_rect(
            buffer,
            size,
            x + thickness,
            y,
            width - 2 * thickness,
            thickness,
        );
    }
    if segments[1] == 1 {
        fill_rect(
            buffer,
            size,
            x + width - thickness,
            y + thickness,
            thickness,
            height / 2 - thickness,
        );
    }
    if segments[2] == 1 {
        fill_rect(
            buffer,
            size,
            x + width - thickness,
            y + height / 2,
            thickness,
            height / 2 - thickness,
        );
    }
    if segments[3] == 1 {
        fill_rect(
            buffer,
            size,
            x + thickness,
            y + height - thickness,
            width - 2 * thickness,
            thickness,
        );
    }
    if segments[4] == 1 {
        fill_rect(
            buffer,
            size,
            x,
            y + height / 2,
            thickness,
            height / 2 - thickness,
        );
    }
    if segments[5] == 1 {
        fill_rect(
            buffer,
            size,
            x,
            y + thickness,
            thickness,
            height / 2 - thickness,
        );
    }
    if segments[6] == 1 {
        fill_rect(
            buffer,
            size,
            x + thickness,
            y + height / 2 - 1,
            width - 2 * thickness,
            thickness,
        );
    }
}

/// Renders a 32×32 transparent RGBA icon containing a clamped integer percentage.
#[must_use]
pub fn render_percentage_icon(percentage: u8) -> Vec<u8> {
    let size = 32_u32;
    let mut rgba = vec![0_u8; (size * size * 4) as usize];
    let percentage = percentage.min(100);
    let digits = if percentage == 100 {
        vec![1, 0, 0]
    } else if percentage >= 10 {
        vec![percentage / 10, percentage % 10]
    } else {
        vec![percentage]
    };
    let total_width = digits.len() as u32 * 7 + digits.len().saturating_sub(1) as u32;
    let mut x = (size - total_width) / 2;
    for digit in digits {
        draw_digit(&mut rgba, size, digit, x);
        x += 8;
    }
    rgba
}

/// Renders the unavailable-state dash used after disconnect or missing data.
#[must_use]
pub fn render_unavailable_icon() -> Vec<u8> {
    let size = 32_u32;
    let mut rgba = vec![0_u8; (size * size * 4) as usize];
    fill_rect(&mut rgba, size, 10, 15, 12, 3);
    rgba
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentage_and_unavailable_icons_have_the_expected_rgba_size() {
        assert_eq!(render_percentage_icon(79).len(), 32 * 32 * 4);
        assert_eq!(render_unavailable_icon().len(), 32 * 32 * 4);
    }

    #[test]
    fn percentage_renderer_clamps_values_above_one_hundred() {
        assert_eq!(render_percentage_icon(255), render_percentage_icon(100));
    }
}
