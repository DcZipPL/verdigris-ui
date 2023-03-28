use leptos::*;

use crate::ui::{Radius, Padding};

#[component]
pub fn Card(cx: Scope,
    #[prop(default=Radius::Small)] radius: Radius,
    #[prop(default=Padding::Small)] padding: Padding,
    #[prop(default=false)] border : bool,
    #[prop(default=false)] hover_tilt : bool,
    children: Children,
) -> impl IntoView
{
    view! { cx,
        <div class="card"
            class=radius.to_string()
            class=padding.to_string()
            class=if hover_tilt { "hover-tilt" } else { "no-hover-tilt" }
            class=if border { "border" } else { "no-border" }
        >
            {children(cx)}
        </div>
    }
}