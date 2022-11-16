use std::ops::Range;

fn main() {
    let pixels = draw_curve(3f64..40f64, -1f64..1f64, 348, 10, |x: f64| x.sin());

    let highest_x = get_highest_x(&pixels);
    let out = render(&pixels, highest_x);
    println!("{}", out);
}

fn draw_curve<F>(
    x_range: Range<f64>,
    y_range: Range<f64>,
    x_pixels: usize,
    y_pixels: usize,
    func: F,
) -> Vec<Vec<bool>>
where
    F: Fn(f64) -> f64,
{
    let mut output: Vec<Vec<bool>> = vec![vec![false; x_pixels]; y_pixels + 1];
    for i in 0..x_pixels {
        let mut y_val =
            func(i as f64 * (x_range.end - x_range.start) / x_pixels as f64 + x_range.start);
        y_val = (y_val.max(y_range.start).min(y_range.end) - y_range.start) * y_pixels as f64
            / (y_range.end - y_range.start);
        output[y_val.round() as usize][i] = true;
    }
    output
}

fn render(pixels: &[Vec<bool>], highest_x: usize) -> String {
    let mut char_col = 0;
    let mut char_row = 0;
    let mut output = String::new();
    loop {
        let mut i = 0;
        let mut block_pixels: [bool; 6] = [false; 6];
        loop {
            let y = i / 2 + char_row * 3;
            if y >= pixels.len() {
                break;
            }
            block_pixels[i] = pixels[y]
                .get(i % 2 + char_col * 2)
                .unwrap_or_else(|| &false)
                .to_owned();
            i += 1;
            if i >= 6 {
                break;
            }
        }
        let bit_int = bool_arr_to_int(&block_pixels);
        output.push(get_char_for_pixels(bit_int));

        char_col += 1;
        if char_col * 2 >= highest_x {
            char_col = 0;
            char_row += 1;
            output.push('\n');
            if char_row * 3 >= pixels.len() {
                break;
            }
        }
    }

    output
}

fn get_highest_x(pixels: &[Vec<bool>]) -> usize {
    let mut max = 0;

    for row in pixels {
        max = max.max(row.len());
    }
    max
}

fn get_char_for_pixels(block_pixels: u32) -> char {
    let offset = match block_pixels {
        0..=19 => 1,
        20..=39 => 2,
        40.. => 3,
    };
    match block_pixels {
        255 => '█',
        0 => ' ',
        20 => '▌',
        40 => '▐',
        block_pixels => char::from_u32(0x1fb00 + block_pixels - offset)
            .expect(&format!("not char: {}", block_pixels)),
    }
}

fn bool_arr_to_int(arr: &[bool]) -> u32 {
    let mut out = 0;
    for (i, pixel) in arr.iter().enumerate() {
        if *pixel {
            out |= 1 << i;
        }
    }
    out
}
