use leptos::*;
use styled::style;

use crate::components::Size;
use crate::components::Radius;
use crate::components::Unit;
use crate::components::Variant;
use crate::components::input::get_colors;
use crate::components::layout::flex::*;

#[component]
pub fn Button(cx: Scope,
    #[prop(default=Size::Small)] size: Size,
    #[prop(default=Radius::Small)] radius: Radius,
    #[prop(default=Variant::Filled)] variant: Variant,
    #[prop(default=false)] compact: bool,
    #[prop(default=false)] disabled: bool,
    children: Box<dyn Fn(Scope) -> Fragment>,
    #[prop(optional, into)] style: String,
) -> impl IntoView
{
    let colors = get_colors(variant).unwrap();

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
            border: ${colors.border_prop()};
            color: ${colors.foreground_color().rgba()};
            background: ${colors.fill_color()};
        }

        &:active {
            transform: translate(0, 1px);
        }

        &:hover {
            background-color: ${colors.hover_color().rgba()};
            color: ${colors.foreground_color().rgba()};
        }
    );

    styled::view! { cx, styles,
        <button disabled=disabled
                style=style
        >
            <Flex gap=Size::Custom(Unit::Rem(0.25))>
                {children(cx)}
            </Flex>
        </button>
    }
}