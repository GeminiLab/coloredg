extern crate colored;

use colored::{Color, Colorize, Style, Styles};

fn is_prime(i: i32) -> bool {
    if i < 2 {
        false
    } else {
        (2..=((i as f64).sqrt() as i32)).all(|m| i % m != 0)
    }
}

fn style_and_color_for(i: i32) -> (Color, Style) {
    if is_prime(i) {
        (Color::Red, Styles::Bold + Styles::Underline)
    } else {
        (Color::BrightBlack, Styles::Italic + Styles::Strikethrough)
    }
}

fn main() {
    let lines = (1..100).map(|i| {
        let (color, style) = style_and_color_for(i);
        let num_str = format!("{:02}", i).color(color).add_style(style);
        format!("line {}", num_str)
    }).collect::<Vec<String>>();

    println!("{}", lines.join("\n"))
}
