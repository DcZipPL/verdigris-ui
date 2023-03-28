use leptos::*;

use crate::components::{Variant, Radius};

#[component]
pub fn Alert(cx: Scope,
    #[prop(optional, into)] title: Option<String>,
    #[prop(default=Radius::Small)] radius: Radius,
    #[prop(default=Variant::Light)] variant: Variant,
    children: Children,
) -> impl IntoView
{
    view! { cx,
        <div class="alert" class=radius.to_string() class=variant.to_string()>
            <div class="title">{title}</div>
            <div>
                {children(cx)}
            </div>
        </div>
    }
}