use color_art::Color;
use std::str::FromStr;

pub struct Colors {
    pub primary: Color,
    pub primary_foreground: Color,
    pub foreground: Color,
    pub background: Color,
    pub background_alt: Color,
}

pub enum Theme {
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
            },
        }
    }
}