use leptos::*;
use rgb::RGBA16;

use crate::components::{Radius, Size};

#[component]
pub fn Badge(cx: Scope,
    #[prop(default=BadgeVariant::Filled)] variant: BadgeVariant,
    #[prop(default=Size::Small)] size: Size,
    #[prop(default=Radius::Circle)] radius: Radius,
    children: Children,
) -> impl IntoView
{
    view! { cx,
        <span class="badge" class=radius.to_string() size=size.to_string()>
            {children(cx)}
        </span>
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BadgeVariant {
    Filled,
    Outline,
    Light,
    Dot,
    Gradient(RGBA16, RGBA16, u8), // From, To, Angle
}