fn main() {
    let pixels = sine_curve();

    let highest_x = get_highest_x(&pixels);
    println!("highest_x: {}", highest_x);
    let out = render(&pixels, highest_x);
    println!("{}", out);
}

fn sine_curve() -> [Vec<bool>; 101] {
    const INIT: Vec<bool> = Vec::new();
    let mut output: [Vec<bool>; 101] = [INIT; 101];
    for i in 0..300 {
        let mut y_val = (((i as f32 / 10_f32).sin()) * 40_f32 + 50_f32).round() as usize;
        y_val = y_val.max(0).min(100);
        if output[y_val] == INIT {
            output[y_val] = vec![false; 300];
        }
        output[y_val][i] = true;
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
