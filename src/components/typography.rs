use leptos::*;
use styled::style;

use crate::theme::{Theme, HighlightColor};

#[component]
pub fn Code(cx: Scope,
    #[prop(optional)] color: Option<HighlightColor>,
    children: Children,
    #[prop(optional, into)] style: String,
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
        <code style=style>{children(cx)}</code>
    }
}

#[component]
pub fn CodeBlock(cx: Scope,
    #[prop(optional)] color: Option<HighlightColor>,
    children: Children,
    #[prop(optional, into)] style: String,
) -> impl IntoView
{
    let colors = Theme::Light.colors();
    let styles = style!(
        pre {
            text-align: left;
            padding: 0.75rem 1rem;
            border-radius: 0.25rem;
            font-size: 0.8125rem;
            line-height: 1.7;
            background-color: ${
                if let Some(c) = color {c.to_string()} else {colors.code_background.rgba()}
            };
        }
    );

    styled::view! { cx, styles,
        <pre style=style>{children(cx)}</pre>
    }
}

#[component]
pub fn CodeBlockTab(cx: Scope,
    #[prop(into)] title: String,
    children: Children,
) -> impl IntoView
{
    view! { cx,
        {children(cx)}
    }
}

#[component]
pub fn Mark(cx: Scope,
    #[prop(optional)] color: Option<HighlightColor>,
    children: Children,
    #[prop(optional, into)] style: String,
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
        <span style=style>
            {children(cx)}
        </span>
    }
}