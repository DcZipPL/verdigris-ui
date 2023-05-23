use color_art::Color;

use crate::theme::Theme;
use crate::components::Variant;

pub mod button;
pub mod icon_button;

fn get_colors(variant: Variant) -> anyhow::Result<ButtonColor> {
    let colors = Theme::Light.colors();
    Ok(ButtonColor {
        variant,
        filled: colors.primary,
        filled_hover: colors.primary.clone().darken(0.075),
        foreground: colors.primary_foreground,
        light: colors.primary.clone().fade(0.1),
        hover_light: colors.primary.clone().fade(0.15),
        hover_light_outline: colors.primary.clone().fade(0.05),
    })
}

struct ButtonColor {
	variant: Variant,
	filled: Color,
	filled_hover: Color,
	foreground: Color,
	light: Color,
	hover_light: Color,
	hover_light_outline: Color,
}

impl ButtonColor {

	fn hover_color(&self) -> Color {
		match self.variant {
			Variant::Filled => self.filled_hover,
			Variant::Light => self.hover_light,
			Variant::Outline => self.hover_light_outline,
			Variant::Subtle => self.light,
			Variant::Transparent => Color::new(0., 0., 0., 0.),
			_ => self.filled,
		}
	}

	fn border_prop(&self) -> &str {
		match self.variant {
			Variant::Outline => "1px solid",
			_ => "none",
		}
	}

	fn fill_color(&self) -> String {
		match self.variant {
			Variant::Filled => self.filled.rgba(),
			Variant::Light => self.light.rgba(),
			Variant::Outline => "transparent".to_string(),
			Variant::Subtle => "transparent".to_string(),
			Variant::Transparent => "transparent".to_string(),
			Variant::Gradient(from, to, angle) => format!("linear-gradient({}deg, {} 0%, {} 100%)", angle, from.hex(), to.hex()),
		}
	}

	fn foreground_color(&self) -> Color {
		match self.variant {
			Variant::Filled => self.foreground,
			Variant::Light => self.filled,
			Variant::Outline => self.filled,
			Variant::Subtle => self.filled,
			_ => self.foreground,
		}
	}
}