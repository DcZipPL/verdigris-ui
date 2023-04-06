use leptos::*;
use styled::style;

use crate::components::Size;
use crate::components::Radius;
use crate::components::Variant;
use crate::components::input::get_colors;

#[component]
pub fn IconButton(cx: Scope,
    #[prop(default=Size::Small)] size: Size,
    #[prop(default=Radius::Small)] radius: Radius,
    #[prop(default=Variant::Filled)] variant: Variant,
    #[prop(default=false)] disabled: bool,
    children: Children,
    #[prop(optional, into)] style: String,
) -> impl IntoView
{
    let colors = get_colors(variant).unwrap();

    let styles = style!(
        button {
            cursor: pointer;
            height: ${size.ib_height().to_string()};
			width: ${size.ib_height().to_string()};
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
            {children(cx)}
        </button>
    }
}