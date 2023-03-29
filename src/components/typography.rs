use leptos::*;
use styled::style;

use crate::theme::{Theme, HighlightColor};

#[component]
pub fn Code(cx: Scope,
    #[prop(default=false)] block: bool,
    #[prop(optional)] color: Option<HighlightColor>,
    children: Children,
) -> impl IntoView
{
    let colors = Theme::Light.colors();
    let styles = style!(
        code {
            padding: 0.125rem;
            border-radius: 0.25rem;
            display: inline;
            background-color: ${
                if let Some(c) = color {c.to_string()} else {colors.code_background.rgba()}
            };
        }
    );

    styled::view! { cx, styles,
        <code>{children(cx)}</code>
    }
}

#[component]
pub fn Mark(cx: Scope,
    #[prop(optional)] color: Option<HighlightColor>,
    children: Children,
) -> impl IntoView
{
    let styles = style!(
        span {
            background-color: ${
                if let Some(c) = color {c.to_string()} else {"#ffec99".to_string()}
            };
        }
    );

    styled::view! { cx, styles,
        <span>
            {children(cx)}
        </span>
    }
}