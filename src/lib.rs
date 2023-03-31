extern crate crossterm;

use crossterm::{Command, cursor, ExecutableCommand, execute};
use std::io::stdout;
use std::ops::Range;
use std::sync::Mutex;
use crossterm::style::Color;


// fn main() {
//     const HEIGHT: u16 = 12;
//     const ROW_COUNT: u16 = HEIGHT / 3 + (HEIGHT % 3 != 0) as u16;
//     println!("{:\n<1$}", "", ROW_COUNT as usize);
//     let mut t = 0;
//     loop {
//         let pixels = draw_function(
//             3f64..40f64,
//             -1f64..1f64,
//             termsize::get().unwrap().cols as usize * 2,
//             HEIGHT as usize,
//             |x: f64| (4f64 * x + t as f64 / 8f64).sin(),
//         );
//         let out = render(&pixels);
//         print!("{}", out);
//         t += 1;
//         thread::sleep(Duration::from_millis(300));
//         stdout().execute(cursor::MoveUp(ROW_COUNT)).unwrap();
//     }
// }

pub struct TerminalDisplay {
    num_rows: usize,
    lock: Mutex<()>,
}

impl Default for TerminalDisplay {
    fn default() -> Self {
        Self {
            num_rows: 0,
            lock: Mutex::new(()),
        }
    }
}

impl TerminalDisplay {
    pub fn draw_function<F>(
        x_range: Range<f64>,
        y_range: Range<f64>,
        x_pixels: usize,
        y_pixels: usize,
        func: F,
    ) -> Vec<Vec<bool>>
        where
            F: Fn(f64) -> f64,
    {
        let mut output: Vec<Vec<bool>> = vec![vec![false; x_pixels]; y_pixels];
        let mut y_vals: Vec<usize> = vec![0; x_pixels];
        for i in 0..x_pixels {
            let mut y_val =
                func(i as f64 * (x_range.end - x_range.start) / x_pixels as f64 + x_range.start);
            y_val = (y_val.clamp(y_range.start, y_range.end) - y_range.start) * (y_pixels - 1) as f64
                / (y_range.end - y_range.start);
            let y_val = y_val.round() as usize;
            y_vals[i] = y_val;
            output[y_val][i] = true;
        }
        for i in 1..x_pixels - 1 {
            let y_val = y_vals[i];
            let range = if y_vals[i - 1] > y_val {
                y_val..y_vals[i - 1]
            } else {
                y_val..y_vals[i + 1]
            };
            for pixel_y in range {
                output[pixel_y][i] = true;
            }
        }
        output
    }

    pub fn render(pixels: &[Vec<bool>]) -> String {
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
                    .unwrap_or(&false)
                    .to_owned();
                i += 1;
                if i >= 6 {
                    break;
                }
            }
            let bit_int = bool_arr_to_int(&block_pixels);
            output.push(get_char_for_pixels(bit_int));

            char_col += 1;
            if char_col * 2 >= get_highest_x(pixels) {
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

    pub fn render_full_block(pixels: &[Vec<bool>]) -> String {
        let mut char_col = 0;
        let mut char_row = 0;
        let mut output = String::new();
        loop {
            if char_row >= pixels.len() {
                break;
            }
            loop {
                if char_col >= get_highest_x(pixels) {
                    break;
                }
                if char_col >= pixels[char_row].len() || !pixels[char_row][char_col] {
                    output.push_str(" ")
                } else {
                    output.push_str("█");
                }
                char_col += 1;
            }
            output.push_str("\n");
            char_row += 1;
            char_col = 0;
        }

        output
    }

    pub fn render_full_block_color(pixels: &[Option<crossterm::style::Color>], width: u16) -> String {
        let mut output = String::new();
        for (i, pixel) in pixels.iter().enumerate() {
            match pixel {
                None => {
                    output.push_str(" ")
                }
                Some(color) => {
                    if i > 0 && pixels[i - 1] != Some(*color) {
                        output.push_str(&*crossterm::style::SetForegroundColor(*color).to_string());
                    }
                    output.push_str("█");
                }
            }
            if i == 0 && i % width as usize == 1 {
                output.push_str("\n");
            }
        }
        output
    }

    pub fn update_display(&mut self, string: String) {
        let _lock = self.lock.lock().expect("Locking for printing");
        let raw_mode = crossterm::terminal::is_raw_mode_enabled().unwrap();
        crossterm::terminal::enable_raw_mode();
        let rows: Vec<&str> = string.split('\n').collect();
        if self.num_rows != 0 {
            stdout().execute(cursor::MoveUp(self.num_rows as u16)).unwrap();
        }
        for row in &rows {
            print!("{}", row);
            execute!(stdout(), cursor::MoveToColumn(0), cursor::MoveDown(1));
        }
        if self.num_rows > rows.len() {
            for _ in 0..(self.num_rows - rows.len()) {
                print!("")
            }
        }
        self.num_rows = rows.len();
        if !raw_mode {
            crossterm::terminal::disable_raw_mode();
        }
    }

    pub fn move_cursor_up(&self, num_rows: u16) {
        let _lock = self.lock.lock().expect("Locking for cursor move");
        stdout().execute(cursor::MoveUp(num_rows)).unwrap();
    }
}

fn get_highest_x<T>(pixels: &[Vec<T>]) -> usize {
    let mut max = 0;

    for row in pixels {
        max = max.max(row.len());
    }
    max
}

pub fn get_char_for_pixels(block_pixels: u8) -> char {
    let offset = match block_pixels {
        0..=20 => 1,
        21..=41 => 2,
        42.. => 3,
    };
    match block_pixels {
        63 => '█',
        0 => ' ',
        21 => '▌',
        42 => '▐',
        block_pixels => char::from_u32(0x1fb00 + block_pixels as u32 - offset)
            .unwrap_or_else(|| panic!("not char: {}", block_pixels)),
    }
}

fn bool_arr_to_int(arr: &[bool]) -> u8 {
    let mut out = 0;
    for (i, pixel) in arr.iter().enumerate() {
        if *pixel {
            out |= 1 << i;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use crate::get_char_for_pixels;

    #[test]
    fn get_char_for_pixels_returns_correct_character() {
        let correct = " 🬀🬁🬂🬃🬄🬅🬆🬇🬈🬉🬊🬋🬌🬍🬎🬏🬐🬑🬒🬓▌🬔🬕🬖🬗🬘🬙🬚🬛🬜🬝🬞🬟🬠🬡🬢🬣🬤🬥🬦🬧▐🬨🬩🬪🬫🬬🬭🬮🬯🬰🬱🬲🬳🬴🬵🬶🬷🬸🬹🬺🬻█";
        for (i, char) in correct.chars().enumerate() {
            assert_eq!(get_char_for_pixels(i as u8), char);
        }
    }
}