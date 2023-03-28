use leptos::*;

use crate::ui::Size;
use crate::ui::Radius;
use crate::ui::Variant;

#[component]
pub fn Button(cx: Scope,
    #[prop(default=Size::Small)] size: Size,
    #[prop(default=Radius::Small)] radius: Radius,
    #[prop(default=Variant::Filled)] variant: Variant,
    #[prop(default=false)] compact: bool,
    children: Children,
) -> impl IntoView
{
    view! { cx,
        <button class=size.to_string()
                class=radius.to_string()
                class=variant.to_string()
                class=if compact { "compact" } else { "full-size" }
        >
            {children(cx)}
        </button>
    }
}