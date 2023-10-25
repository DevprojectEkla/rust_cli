extern crate termcolor;

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub const RED: Color = Color::Red;
pub const CYAN: Color = Color::Cyan;
pub const YELLOW: Color = Color::Yellow;
pub const WHITE: Color = Color::White;
pub const GREEN: Color = Color::Green;

pub fn print_c(string: &str, color: Color) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color)).set_bold(true)).unwrap();
    writeln!(&mut stdout, "\r{}", string).unwrap();
    stdout.reset().unwrap();
}
// same as above but for the spinner we don't want newlines so we use write!
pub fn print_c_no_nl(string: &str, color: Color) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color)).set_bold(true)).unwrap();
    write!(&mut stdout, "\r{}", string).unwrap();
    stdout.reset().unwrap();
}

pub fn error(string:&str){
    print_c(string, RED)

}

pub fn warn(string:&str){
    print_c(string, YELLOW)

}

pub fn info(string:&str){
    print_c(string, CYAN)

}

pub fn success(string:&str){
    print_c(string, GREEN)
}

pub fn text(string:&str){
    print_c(string, WHITE)
}
