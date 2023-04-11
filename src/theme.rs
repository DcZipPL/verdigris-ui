use color_art::Color;
use std::{str::FromStr, fmt};

pub struct Colors {
    pub primary: Color,
    pub primary_foreground: Color,
    pub foreground: Color,
    pub background: Color,
    pub background_alt: Color,
    pub code_background: Color,
    pub border: Color,
    pub shell_border: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Theme {
    #[default]
    Light,
}

impl Theme {
    pub fn colors(&self) -> Colors {
        match self {
            Theme::Light => Colors {
                primary: Color::from_str("#46c7c0").unwrap(),
                primary_foreground: Color::from_str("#fff").unwrap(),
                foreground: Color::from_str("#000").unwrap(),
                background: Color::from_str("#fff").unwrap(),
                background_alt: Color::from_str("#f8f9fa").unwrap(),
                code_background: Color::from_str("#f8f9fa").unwrap(),
                border: Color::from_str("#e0e0e0").unwrap(),
                shell_border: Color::from_str("#e9ecef").unwrap(),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HighlightColor {
    Yellow, Green, Blue, Red, Purple, Orange, Gray,
    Color(Color)
}

impl fmt::Display for HighlightColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HighlightColor::Yellow => write!(f, "#ffec99"),
            HighlightColor::Green => write!(f, "#b3ff99"),
            HighlightColor::Blue => write!(f, "#99ffff"),
            HighlightColor::Red => write!(f, "#ff9999"),
            HighlightColor::Purple => write!(f, "#ff99ff"),
            HighlightColor::Orange => write!(f, "#ffcc99"),
            HighlightColor::Gray => write!(f, "#cccccc"),
            HighlightColor::Color(color) => write!(f, "{}", color.hex()),
        }
    }
}
