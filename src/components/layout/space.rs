use leptos::*;

use crate::components::Size;

#[component]
pub fn Space(cx: Scope,
    #[prop(default=Size::Micro)] width: Size,
    #[prop(default=Size::Micro)] height: Size,
) -> impl IntoView
{
    view! { cx,
        <div class="space" class=width.to_string().replace("size", "width") class=height.to_string().replace("size", "height")/>
    }
}