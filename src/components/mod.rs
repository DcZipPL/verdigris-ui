use std::fmt;

use color_art::Color;

pub mod display;
pub mod input;
pub mod layout;
pub mod typography;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Radius {
    None,
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
    Circle,
    Custom(Unit),
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Variant {
    Filled,
    Outline,
    Light,
    Subtle,
    Gradient(Color, Color, u8), // From, To, Angle
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

impl Radius {
    pub(crate) fn units(&self) -> Unit {
        match self {
            Radius::None => Unit::Px(0),
            Radius::ExtraSmall => Unit::Rem(0.125),
            Radius::Small => Unit::Rem(0.25),
            Radius::Medium => Unit::Rem(0.5),
            Radius::Large => Unit::Rem(1.0),
            Radius::ExtraLarge => Unit::Rem(2.0),
            Radius::Circle => Unit::Percent(50),
            Radius::Custom(unit) => unit.clone(),
        }
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("size-{:?}", self).to_lowercase())
    }
}

impl Size {
    pub(crate) fn ib_height(&self) -> Unit {
        match self {
            Size::None => Unit::Px(0),
            Size::Micro => Unit::Rem(1.375),
            Size::ExtraSmall => Unit::Rem(1.875),
            Size::Small => Unit::Rem(2.25),
            Size::Medium => Unit::Rem(2.625),
            Size::Large => Unit::Rem(3.125),
            Size::ExtraLarge => Unit::Rem(3.75),
            Size::Custom(unit) => unit.clone(),
        }
    }

    pub(crate) fn ib_padding(&self) -> Unit {
        match self {
            Size::None => Unit::Px(0),
            Size::Micro => Unit::Rem(0.625),
            Size::ExtraSmall => Unit::Rem(0.875),
            Size::Small => Unit::Rem(1.125),
            Size::Medium => Unit::Rem(1.375),
            Size::Large => Unit::Rem(1.625),
            Size::ExtraLarge => Unit::Rem(2.0),
            Size::Custom(unit) => unit.clone(),
        }
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