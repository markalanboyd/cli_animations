use std::io::Write;

pub fn hide_cursor() {
    print!("\x1B[?25l");
    std::io::stdout().flush().unwrap();
}
