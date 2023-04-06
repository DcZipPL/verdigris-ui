use color_art::Color;
use leptos::*;
use styled::style;

use crate::components::Size;
use crate::components::Radius;
use crate::components::Unit;
use crate::components::Variant;
use crate::theme::Theme;

#[component]
pub fn Button(cx: Scope,
    #[prop(default=Size::Small)] size: Size,
    #[prop(default=Radius::Small)] radius: Radius,
    #[prop(default=Variant::Filled)] variant: Variant,
    #[prop(default=false)] compact: bool,
    #[prop(default=false)] disabled: bool,
    children: Children,
    #[prop(optional, into)] style: String,
) -> impl IntoView
{
    let (foreground, filled, filled_hover, light, hover_light, hover_light_outline) = get_colors().unwrap();
    let hover_color = match variant {
        Variant::Filled => filled_hover,
        Variant::Light => hover_light,
        Variant::Outline => hover_light_outline,
        Variant::Subtle => light,
        _ => filled,
    };
    let border_prop = match variant {
        Variant::Filled => "none",
        Variant::Light => "none",
        Variant::Outline => "1px solid",
        Variant::Subtle => "none",
        _ => "none",
    };
    let fill_color = match variant {
        Variant::Filled => filled.rgba(),
        Variant::Light => light.rgba(),
        Variant::Outline => "transparent".to_string(),
        Variant::Subtle => "transparent".to_string(),
        Variant::Gradient(from, to, angle) => format!("linear-gradient({}deg, {} 0%, {} 100%)", angle, from.hex(), to.hex()),
    };
    let foreground_color = match variant {
        Variant::Filled => foreground,
        Variant::Light => filled,
        Variant::Outline => filled,
        Variant::Subtle => filled,
        _ => foreground,
    };

    let styles = style!(
        button {
            cursor: pointer;
            height: ${
                if compact {
                    "auto".to_string()
                } else {
                    size.ib_height().to_string()
                }
            };
            padding: ${
                if compact {
                    Unit::Rem(0.25).to_string()
                } else {
                    format!("0 {}", size.ib_padding())
                }
            };
            font-size: ${size.ib_font_size()};
            border-radius: ${radius.units()};
            border: ${border_prop};
            color: ${foreground_color.rgba()};
            background: ${fill_color};
        }

        &:active {
            transform: translate(0, 1px);
        }

        &:hover {
            background-color: ${hover_color.rgba()};
            color: ${foreground_color.rgba()};
        }
    );

    styled::view! { cx, styles,
        <button class=if compact { "compact" } else { "full-size" }
                disabled=disabled
                style=style
        >
            {children(cx)}
        </button>
    }
}

fn get_colors() -> anyhow::Result<(Color, Color, Color, Color, Color, Color)> {
    let colors = Theme::Light.colors();
    let filled_hover_color = colors.primary.clone().darken(0.075)?;
    let light_color = colors.primary.clone().fade(0.1)?;
    let hover_light_color = colors.primary.clone().fade(0.15)?;
    let hover_light_outline_color = colors.primary.clone().fade(0.05)?;
    Ok((colors.primary_foreground, colors.primary, filled_hover_color, light_color, hover_light_color, hover_light_outline_color))
}