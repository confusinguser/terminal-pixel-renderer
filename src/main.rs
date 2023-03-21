fn main() {
    for i in 0..64 {
        print!("{}", terminal_pixel_renderer::get_char_for_pixels(i));
    }
}