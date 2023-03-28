use leptos::*;

use crate::components::Size;
use crate::components::Radius;
use crate::components::Variant;

#[component]
pub fn Button(cx: Scope,
    #[prop(default=Size::Small)] size: Size,
    #[prop(default=Radius::Small)] radius: Radius,
    #[prop(default=Variant::Filled)] variant: Variant,
    #[prop(default=false)] compact: bool,
    #[prop(default=false)] disabled: bool,
    children: Children,
) -> impl IntoView
{
    view! { cx,
        <button class=size.to_string()
                class=radius.to_string()
                class=variant.to_string()
                class=if compact { "compact" } else { "full-size" }
                disabled=disabled
        >
            {children(cx)}
        </button>
    }
}