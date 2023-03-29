use leptos::*;
use styled::style;

use crate::components::Size;

#[component]
pub fn Space(cx: Scope,
    #[prop(default=Size::Micro)] width: Size,
    #[prop(default=Size::Micro)] height: Size,
) -> impl IntoView
{
    let styles = style!(
        div {
            display: inline-block;
            width: ${width.space()};
            height: ${height.space()};
        }
    );

    styled::view! { cx, styles,
        <div/>
    }
}