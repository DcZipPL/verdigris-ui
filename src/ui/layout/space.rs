use leptos::*;

use crate::ui::Size;

#[component]
pub fn Space(cx: Scope,
    #[prop(default=Size::None)] width: Size,
    #[prop(default=Size::None)] height: Size,
) -> impl IntoView
{
    view! { cx,
        <div class=width.to_string().replace("size", "width") class=height.to_string().replace("size", "height")/>
    }
}