extern crate colored;

use colored::{Color, Colorize, NO_STYLE, Style, Styles};

static STYLES: [(Styles, &str); 8] = [
    (Styles::Bold, "BLD"),
    (Styles::Italic, "ITA"),
    (Styles::Underline, "UDL"),
    (Styles::Strikethrough, "STH"),
    (Styles::Dimmed, "DIM"),
    (Styles::Blink, "BLK"),
    (Styles::Reversed, "REV"),
    (Styles::Hidden, "HDN"),
];

static SENTENCE: &str = " The quick brown fox jumps over the lazy dog. ";

fn main() {
    for i in 0u8..=255 {
        let mut style = NO_STYLE;
        let mut line = String::new();

        for b in 0u8..8 {
            if i & (1 << b) != 0 {
                line.push_str(format!("{}+ ", STYLES[b as usize].1).as_str().color(Color::BrightWhite).to_string().as_str());
                style += STYLES[b as usize].0;
            } else {
                line.push_str(format!("{}- ", STYLES[b as usize].1).as_str().color(Color::BrightBlack).to_string().as_str());
            }
        }

        line.push_str("    ");
        line.push_str(SENTENCE.add_style(style).to_string().as_str());

        println!("{}", line);
    }
}
