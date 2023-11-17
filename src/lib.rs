mod utils;

use std::f64::consts::PI;
use std::process::Command;
use std::{thread, time::Duration};

pub fn sine_horizontal(display_char: char) {
    let mut count: f64 = 0.0;
    utils::hide_cursor();
    loop {
        Command::new("clear").status().unwrap();
        let sine_value = count.sin();
        let scaled_width = ((sine_value + 1.0) / 2.0) * 80.0;
        let display_string = std::iter::repeat(display_char)
            .take(scaled_width as usize)
            .collect::<String>();
        println!("{}", display_string);
        count += 2.0 * PI / 100.0;
        if count > 2.0 * PI {
            count -= 2.0 * PI;
        }
        thread::sleep(Duration::from_millis(10));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        sine_horizontal('█');
    }
}
