use color_art::Color;
use leptos::*;

use crate::components::{Radius, Size};

#[component]
pub fn Badge(cx: Scope,
    #[prop(default=BadgeVariant::Filled)] variant: BadgeVariant,
    #[prop(default=Size::Small)] size: Size,
    #[prop(default=Radius::Circle)] radius: Radius,
    children: Children,
    #[prop(optional, into)] style: String,
) -> impl IntoView
{
    view! { cx,
        <span class="badge" class=radius.to_string() size=size.to_string() style=style>
            {children(cx)}
        </span>
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BadgeVariant {
    Filled,
    Outline,
    Light,
    Dot,
    Gradient(Color, Color, u8), // From, To, Angle
}