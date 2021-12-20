//!Coloring terminal so simple, you already know how to do it !
//!
//!    use colored::Colorize;
//!
//!    "this is blue".blue();
//!    "this is red".red();
//!    "this is red on blue".red().on_blue();
//!    "this is also red on blue".on_blue().red();
//!    "you can use truecolor values too!".truecolor(0, 255, 136);
//!    "background truecolor also works :)".on_truecolor(135, 28, 167);
//!    "you can also make bold comments".bold();
//!    println!("{} {} {}", "or use".cyan(), "any".italic().yellow(), "string type".cyan());
//!    "or change advice. This is red".yellow().blue().red();
//!    "or clear things up. This is default color and style".red().bold().clear();
//!    "purple and magenta are the same".purple().magenta();
//!    "bright colors are also allowed".bright_blue().on_bright_white();
//!    "you can specify color by string".color("blue").on_color("red");
//!    "and so are normal and clear".normal().clear();
//!    String::from("this also works!").green().bold();
//!    format!("{:30}", "format works as expected. This will be padded".blue());
//!    format!("{:.3}", "and this will be green but truncated to 3 chars".green());
//!
//!
//! See [the `Colorize` trait](./trait.Colorize.html) for all the methods.
//!
#![warn(missing_docs)]

extern crate atty;
#[macro_use]
extern crate lazy_static;
extern crate paste;
#[cfg(windows)]
extern crate winapi;

#[cfg(test)]
extern crate rspec;

mod color;
pub mod control;
mod style;

pub use color::*;
pub use style::{Style, Styles, NO_STYLE, ALL_STYLE};

use std::{borrow::Cow, fmt, ops::Deref};
use paste::paste;

/// A string that may have color and/or style applied to it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColoredString {
    input: String,
    fgcolor: Option<Color>,
    bgcolor: Option<Color>,
    style: style::Style,
}

macro_rules! colors_for_colorize {
    ($($color:ident $(| $alias:ident)*),+ $(,)?) => {
        paste! {
            $(
                #[doc = concat!("Set the foreground color to `Color::", stringify!($color), "`.")]
                fn [< $color:snake:lower >](self) -> ColoredString where Self: Sized {
                    self.color(Color::$color)
                }

                #[doc = concat!("Set the background color to `Color::", stringify!($color), "`.")]
                fn [< on_ $color:snake:lower >](self) -> ColoredString where Self: Sized {
                    self.on_color(Color::$color)
                }

                #[doc = concat!("Set the foreground color to `Color::Bright", stringify!($color), "`.")]
                fn [< bright_ $color:snake:lower >](self) -> ColoredString where Self: Sized {
                    self.color(Color::[< Bright $color >])
                }

                #[doc = concat!("Set the background color to `Color::Bright", stringify!($color), "`.")]
                fn [< on_bright_ $color:snake:lower >](self) -> ColoredString where Self: Sized {
                    self.on_color(Color::[< Bright $color >])
                }

                $(
                    #[deprecated = concat!("Please use `", stringify!([< $color:snake:lower >]), "` instead")]
                    #[doc = concat!("Set the foreground color to `Color::", stringify!($color), "`.")]
                    fn [< $alias:snake:lower >](self) -> ColoredString where Self: Sized {
                        self.color(Color::$color)
                    }

                    #[deprecated = concat!("Please use `", stringify!([< on_ $color:snake:lower >]), "` instead")]
                    #[doc = concat!("Set the background color to `Color::", stringify!($color), "`.")]
                    fn [< on_ $alias:snake:lower >](self) -> ColoredString where Self: Sized {
                        self.on_color(Color::$color)
                    }

                    #[deprecated = concat!("Please use `", stringify!([< bright_ $color:snake:lower >]), "` instead")]
                    #[doc = concat!("Set the foreground color to `Color::Bright", stringify!($color), "`.")]
                    fn [< bright_ $alias:snake:lower >](self) -> ColoredString where Self: Sized {
                        self.color(Color::[< Bright $color >])
                    }

                    #[deprecated = concat!("Please use `", stringify!([< on_bright_ $color:snake:lower >]), "` instead")]
                    #[doc = concat!("Set the background color to `Color::Bright", stringify!($color), "`.")]
                    fn [< on_bright_ $alias:snake:lower >](self) -> ColoredString where Self: Sized {
                        self.on_color(Color::[< Bright $color >])
                    }
                )*
            )+
        }
    };
}

macro_rules! styles_for_colorize {
    ($($style:ident $(| $alias:ident)*),+ $(,)?) => {
        paste! {
            $(
                fn [< $style:snake:lower >](self) -> ColoredString where Self: Sized {
                    self.add_style(Styles::$style)
                }

                fn [< remove_ $style:snake:lower >](self) -> ColoredString where Self: Sized {
                    self.remove_style(Styles::$style)
                }

                $(
                    fn [< $alias:snake:lower >](self) -> ColoredString where Self: Sized {
                        self.add_style(Styles::$style)
                    }

                    fn [< on_ $alias:snake:lower >](self) -> ColoredString where Self: Sized {
                        self.remove_style(Styles::$style)
                    }
                )*
            )+
        }
    };
}

/// The trait that enables something to be given color.
///
/// You can use `colored` effectively simply by importing this trait
/// and then using its methods on `String` and `&str`.
#[allow(missing_docs)]
pub trait Colorize {
    fn color<S: Into<Color>>(self, color: S) -> ColoredString;
    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString;
    fn add_style<S: Into<Style>>(self, style: S) -> ColoredString;
    fn remove_style<S: Into<Style>>(self, style: S) -> ColoredString;
    fn clear(self) -> ColoredString;

    colors_for_colorize!(
        Black,
        Red,
        Green,
        Yellow,
        Blue,
        Magenta | Purple,
        Cyan | Wtf,
        White,
    );

    styles_for_colorize!(
        Bold,
        Dimmed,
        Italic,
        Underline,
        Blink,
        Reversed | Reverse,
        Hidden,
        Strikethrough,
    );

    fn truecolor(self, r: u8, g: u8, b: u8) -> ColoredString where Self: Sized {
        self.color(Color::TrueColor { r, g, b })
    }

    fn on_truecolor(self, r: u8, g: u8, b: u8) -> ColoredString where Self: Sized {
        self.on_color(Color::TrueColor { r, g, b })
    }

    fn normal(self) -> ColoredString where Self: Sized {
        self.clear()
    }
}

impl ColoredString {
    /// Get the current background color applied.
    ///
    /// ```rust
    /// # use colored::*;
    /// let cstr = "".blue();
    /// assert_eq!(cstr.fgcolor(), Some(Color::Blue));
    /// let cstr = cstr.clear();
    /// assert_eq!(cstr.fgcolor(), None);
    /// ```
    pub fn fgcolor(&self) -> Option<Color> {
        self.fgcolor.as_ref().copied()
    }

    /// Get the current background color applied.
    ///
    /// ```rust
    /// # use colored::*;
    /// let cstr = "".on_blue();
    /// assert_eq!(cstr.bgcolor(), Some(Color::Blue));
    /// let cstr = cstr.clear();
    /// assert_eq!(cstr.bgcolor(), None);
    /// ```
    pub fn bgcolor(&self) -> Option<Color> {
        self.bgcolor.as_ref().copied()
    }

    /// Get the current [`Style`] which can be check if it contains a [`Styles`].
    ///
    /// ```rust
    /// # use colored::*;
    /// let colored = "".bold().italic();
    /// assert_eq!(colored.style().contains(Styles::Bold), true);
    /// assert_eq!(colored.style().contains(Styles::Italic), true);
    /// assert_eq!(colored.style().contains(Styles::Dimmed), false);
    /// ```
    pub fn style(&self) -> style::Style {
        self.style
    }

    /// Checks if the colored string has no color or styling.
    ///
    /// ```rust
    /// # use colored::*;
    /// let cstr = "".red();
    /// assert_eq!(cstr.is_plain(), false);
    /// let cstr = cstr.clear();
    /// assert_eq!(cstr.is_plain(), true);
    /// ```
    pub fn is_plain(&self) -> bool {
        self.bgcolor.is_none() && self.fgcolor.is_none() && self.style == NO_STYLE
    }

    #[cfg(not(feature = "no-color"))]
    fn has_colors(&self) -> bool {
        control::SHOULD_COLORIZE.should_colorize()
    }

    #[cfg(feature = "no-color")]
    fn has_colors(&self) -> bool {
        false
    }

    fn compute_style(&self) -> String {
        if !self.has_colors() || self.is_plain() {
            return String::new();
        }

        let mut res = String::from("\x1B[");
        let mut has_wrote = if self.style != NO_STYLE {
            res.push_str(&self.style.to_str());
            true
        } else {
            false
        };

        if let Some(ref bgcolor) = self.bgcolor {
            if has_wrote {
                res.push(';');
            }

            res.push_str(&bgcolor.to_bg_str());
            has_wrote = true;
        }

        if let Some(ref fgcolor) = self.fgcolor {
            if has_wrote {
                res.push(';');
            }

            res.push_str(&fgcolor.to_fg_str());
        }

        res.push('m');
        res
    }

    fn escape_inner_reset_sequences(&self) -> Cow<str> {
        if !self.has_colors() || self.is_plain() {
            return self.input.as_str().into();
        }

        // TODO: BoyScoutRule
        let reset = "\x1B[0m";
        let style = self.compute_style();
        let matches: Vec<usize> = self
            .input
            .match_indices(reset)
            .map(|(idx, _)| idx)
            .collect();
        if matches.is_empty() {
            return self.input.as_str().into();
        }

        let mut input = self.input.clone();
        input.reserve(matches.len() * style.len());

        for (idx_in_matches, offset) in matches.into_iter().enumerate() {
            // shift the offset to the end of the reset sequence and take in account
            // the number of matches we have escaped (which shift the index to insert)
            let mut offset = offset + reset.len() + idx_in_matches * style.len();

            for cchar in style.chars() {
                input.insert(offset, cchar);
                offset += 1;
            }
        }

        input.into()
    }
}

impl Default for ColoredString {
    fn default() -> Self {
        ColoredString {
            input: String::default(),
            fgcolor: None,
            bgcolor: None,
            style: NO_STYLE,
        }
    }
}

impl Deref for ColoredString {
    type Target = str;
    fn deref(&self) -> &str {
        &self.input
    }
}

impl<'a> From<&'a str> for ColoredString {
    fn from(s: &'a str) -> Self {
        ColoredString {
            input: String::from(s),
            ..ColoredString::default()
        }
    }
}

impl Colorize for ColoredString {
    fn color<S: Into<Color>>(mut self, color: S) -> ColoredString {
        self.fgcolor = Some(color.into());
        self
    }
    fn on_color<S: Into<Color>>(mut self, color: S) -> ColoredString {
        self.bgcolor = Some(color.into());
        self
    }

    fn add_style<S: Into<Style>>(mut self, style: S) -> ColoredString {
        self.style += style.into();
        self
    }
    fn remove_style<S: Into<Style>>(mut self, style: S) -> ColoredString {
        self.style -= style.into();
        self
    }
    fn clear(self) -> ColoredString {
        ColoredString {
            input: self.input,
            ..ColoredString::default()
        }
    }
}

impl<'a> Colorize for &'a str {
    fn color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            fgcolor: Some(color.into()),
            input: String::from(self),
            ..ColoredString::default()
        }
    }

    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            bgcolor: Some(color.into()),
            input: String::from(self),
            ..ColoredString::default()
        }
    }

    fn add_style<S: Into<Style>>(self, style: S) -> ColoredString {
        ColoredString::from(self).add_style(style)
    }

    fn remove_style<S: Into<Style>>(self, style: S) -> ColoredString {
        ColoredString::from(self).remove_style(style)
    }

    fn clear(self) -> ColoredString {
        ColoredString {
            input: String::from(self),
            style: NO_STYLE,
            ..ColoredString::default()
        }
    }
}

impl fmt::Display for ColoredString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.has_colors() || self.is_plain() {
            return <String as fmt::Display>::fmt(&self.input, f);
        }

        // XXX: see tests. Useful when nesting colored strings
        let escaped_input = self.escape_inner_reset_sequences();

        f.write_str(&self.compute_style())?;
        escaped_input.fmt(f)?;
        f.write_str("\x1B[0m")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatting() {
        // respect the formatting. Escape sequence add some padding so >= 40
        assert!(format!("{:40}", "".blue()).len() >= 40);
        // both should be truncated to 1 char before coloring
        assert_eq!(
            format!("{:1.1}", "toto".blue()).len(),
            format!("{:1.1}", "1".blue()).len()
        )
    }

    #[test]
    fn it_works() {
        let toto = "toto";
        println!("{}", toto.red());
        println!("{}", String::from(toto).red());
        println!("{}", toto.blue());

        println!("blue style ****");
        println!("{}", toto.bold());
        println!("{}", "yeah ! Red bold !".red().bold());
        println!("{}", "yeah ! Yellow bold !".bold().yellow());
        println!("{}", toto.bold().blue());
        println!("{}", toto.blue().bold());
        println!("{}", toto.blue().bold().underline());
        println!("{}", toto.blue().italic());
        println!("******");
        println!("test clearing");
        println!("{}", "red cleared".red().clear());
        println!("{}", "bold cyan cleared".bold().cyan().clear());
        println!("******");
        println!("Bg tests");
        println!("{}", toto.green().on_blue());
        println!("{}", toto.on_magenta().yellow());
        println!("{}", toto.purple().on_yellow());
        println!("{}", toto.magenta().on_white());
        println!("{}", toto.cyan().on_green());
        println!("{}", toto.black().on_white());
        println!("******");
        println!("{}", toto.green());
        println!("{}", toto.yellow());
        println!("{}", toto.purple());
        println!("{}", toto.magenta());
        println!("{}", toto.cyan());
        println!("{}", toto.white());
        println!("{}", toto.white().red().blue().green());
        println!("{}", toto.truecolor(255, 0, 0));
        println!("{}", toto.truecolor(255, 255, 0));
        println!("{}", toto.on_truecolor(0, 80, 80));
        // uncomment to see term output
        // assert!(false)
    }

    #[test]
    fn compute_style_empty_string() {
        assert_eq!("", "".clear().compute_style());
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn compute_style_simple_fg_blue() {
        let blue = "\x1B[34m";

        assert_eq!(blue, "".blue().compute_style());
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn compute_style_simple_bg_blue() {
        let on_blue = "\x1B[44m";

        assert_eq!(on_blue, "".on_blue().compute_style());
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn compute_style_blue_on_blue() {
        let blue_on_blue = "\x1B[44;34m";

        assert_eq!(blue_on_blue, "".blue().on_blue().compute_style());
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn compute_style_simple_fg_bright_blue() {
        let blue = "\x1B[94m";

        assert_eq!(blue, "".bright_blue().compute_style());
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn compute_style_simple_bg_bright_blue() {
        let on_blue = "\x1B[104m";

        assert_eq!(on_blue, "".on_bright_blue().compute_style());
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn compute_style_bright_blue_on_bright_blue() {
        let blue_on_blue = "\x1B[104;94m";

        assert_eq!(
            blue_on_blue,
            "".bright_blue().on_bright_blue().compute_style()
        );
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn compute_style_simple_bold() {
        let bold = "\x1B[1m";

        assert_eq!(bold, "".bold().compute_style());
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn compute_style_blue_bold() {
        let blue_bold = "\x1B[1;34m";

        assert_eq!(blue_bold, "".blue().bold().compute_style());
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn compute_style_blue_bold_on_blue() {
        let blue_bold_on_blue = "\x1B[1;44;34m";

        assert_eq!(
            blue_bold_on_blue,
            "".blue().bold().on_blue().compute_style()
        );
    }

    #[test]
    fn escape_reset_sequence_spec_should_do_nothing_on_empty_strings() {
        let style = ColoredString::default();
        let expected = String::new();

        let output = style.escape_inner_reset_sequences();

        assert_eq!(expected, output);
    }

    #[test]
    fn escape_reset_sequence_spec_should_do_nothing_on_string_with_no_reset() {
        let style = ColoredString {
            input: String::from("hello world !"),
            ..ColoredString::default()
        };

        let expected = String::from("hello world !");
        let output = style.escape_inner_reset_sequences();

        assert_eq!(expected, output);
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn escape_reset_sequence_spec_should_replace_inner_reset_sequence_with_current_style() {
        let input = format!("start {} end", String::from("hello world !").red());
        let style = input.blue();

        let output = style.escape_inner_reset_sequences();
        let blue = "\x1B[34m";
        let red = "\x1B[31m";
        let reset = "\x1B[0m";
        let expected = format!("start {}hello world !{}{} end", red, reset, blue);
        assert_eq!(expected, output);
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn escape_reset_sequence_spec_should_replace_multiple_inner_reset_sequences_with_current_style()
    {
        let italic_str = String::from("yo").italic();
        let input = format!(
            "start 1:{} 2:{} 3:{} end",
            italic_str, italic_str, italic_str
        );
        let style = input.blue();

        let output = style.escape_inner_reset_sequences();
        let blue = "\x1B[34m";
        let italic = "\x1B[3m";
        let reset = "\x1B[0m";
        let expected = format!(
            "start 1:{}yo{}{} 2:{}yo{}{} 3:{}yo{}{} end",
            italic, reset, blue, italic, reset, blue, italic, reset, blue
        );

        println!("first: {}\nsecond: {}", expected, output);

        assert_eq!(expected, output);
    }

    #[test]
    fn color_fn() {
        assert_eq!("blue".blue(), "blue".color("blue"))
    }

    #[test]
    fn on_color_fn() {
        assert_eq!("blue".on_blue(), "blue".on_color("blue"))
    }

    #[test]
    fn bright_color_fn() {
        assert_eq!("blue".bright_blue(), "blue".color("bright blue"))
    }

    #[test]
    fn on_bright_color_fn() {
        assert_eq!("blue".on_bright_blue(), "blue".on_color("bright blue"))
    }

    #[test]
    fn exposing_tests() {
        let cstring = "".red();
        assert_eq!(cstring.fgcolor(), Some(Color::Red));
        assert_eq!(cstring.bgcolor(), None);

        let cstring = cstring.clear();
        assert_eq!(cstring.fgcolor(), None);
        assert_eq!(cstring.bgcolor(), None);

        let cstring = cstring.blue().on_bright_yellow();
        assert_eq!(cstring.fgcolor(), Some(Color::Blue));
        assert_eq!(cstring.bgcolor(), Some(Color::BrightYellow));

        let cstring = cstring.bold().italic();
        assert_eq!(cstring.fgcolor(), Some(Color::Blue));
        assert_eq!(cstring.bgcolor(), Some(Color::BrightYellow));
        assert_eq!(cstring.style().contains(Styles::Bold), true);
        assert_eq!(cstring.style().contains(Styles::Italic), true);
        assert_eq!(cstring.style().contains(Styles::Dimmed), false);
    }
}
