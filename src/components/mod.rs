use std::fmt;

use rgb::RGBA16;

pub mod display;
pub mod input;
pub mod layout;
pub mod typography;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Radius {
    None,
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
    Circle,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    None,
    Micro,
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
    Custom(Unit),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Padding {
    None,
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
    Custom(Unit),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Margin {
    None,
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
    Custom(Unit),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Variant {
    Filled,
    Outline,
    Light,
    Subtle,
    Gradient(RGBA16, RGBA16, u8), // From, To, Angle
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    Px(u32),
    Em(f32),
    Rem(f32),
    Percent(u8),
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Unit::Px(px) => write!(f, "{}px", px),
            Unit::Em(em) => write!(f, "{}em", em),
            Unit::Rem(rem) => write!(f, "{}rem", rem),
            Unit::Percent(percent) => write!(f, "{}%", percent),
        }
    }
}

impl fmt::Display for Radius {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("radius-{:?}", self).to_lowercase())
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("size-{:?}", self).to_lowercase())
    }
}

impl fmt::Display for Padding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("padding-{:?}", self).to_lowercase())
    }
}

impl fmt::Display for Margin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("margin-{:?}", self).to_lowercase())
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("variant-{:?}", self).to_lowercase())
    }
}