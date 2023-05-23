use leptos::*;
use styled::style;

use crate::{components::{Radius, Size}, theme::Theme};

#[component]
pub fn Card(cx: Scope,
    #[prop(default=Radius::Small)] radius: Radius,
    #[prop(default=Size::Small)] padding: Size,
    #[prop(default=false)] border : bool,
    #[prop(default=false)] hover_tilt : bool,
    children: Children,
    #[prop(optional, into)] style: String,
) -> impl IntoView
{
    let colors = Theme::Light.colors();
    let border_prop = if border { format!("1px solid {}", colors.border.rgba()) } else { "none".to_string() };

    let styles = style!(
        .card {
            box-shadow: ${"rgba(0, 0, 0, 0.05) 0px 0.0625rem 0.1875rem, rgba(0, 0, 0, 0.05) 0px 0.625rem 0.9375rem -0.3125rem, rgba(0, 0, 0, 0.04) 0px 0.4375rem 0.4375rem -0.3125rem"};
            background-color: ${colors.background.rgba()};
            border: ${border_prop};
            border-radius: ${radius.units()};
            padding: ${padding.padding()};
        }

        &.hover-tilt:hover {
            transition: transform 0.02s ease-in-out;
            transform: translateY(-0.25rem);
        }
    );

    styled::view! { cx, styles,
        <div class="card"
            class:hover-tilt=hover_tilt
            style=style
        >
            {children(cx)}
        </div>
    }
}