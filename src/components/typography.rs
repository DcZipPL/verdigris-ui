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
    #[prop(into)] data: MaybeSignal<Vec<CodeTab>>,
) -> impl IntoView
{
    let (current, set_current) = create_signal(cx, 0i16);

    let colors = Theme::Light.colors();
    let styles = style!(
        div {
            text-align: left;
        }

        & > button {
            cursor: pointer;
            height: 2rem;
            font-size: 0.8125rem;
            border: ${colors.border.rgba()} 1px solid;
            border-bottom: none;
            border-radius: 0.25rem 0.25rem 0 0;
            padding: 0 0.75rem;
        }

        div > div > pre {
            margin-top: 0;
            text-align: left;
            padding: 0.75rem 1rem;
            border-radius: 0 0 0.25rem 0.25rem;
            border: ${colors.border.rgba()} 1px solid;
            font-size: 0.8125rem;
            line-height: 1.7;
            background-color: ${
                 {colors.code_background.rgba()}
            };
        }
    );

    styled::view! { cx, styles,
        <div>
            <For
            each=data.clone()
            key=|item| item.id
            view=move |cx, item: CodeTab| {
                view! { cx,
                    <button on:click=move |_| set_current.set(item.id) style:background=move || {
                        if item.id == current() {
                            colors.code_background.rgba()
                        } else {
                            colors.background.rgba()
                        }
                    }>{item.title.clone()}</button>
                }
            }
            />
            <For
                each=data
                key=|item| item.id
                view=move |cx, item: CodeTab| {
                    view! { cx,
                        <div style=move || format!("display: {}",
                            if item.id == current() {"block"} else {"none"}
                        )>
                            <pre>{item.code}</pre>
                        </div>
                    }
                }
            />
        </div>
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeTab {
    pub id: i16,
    pub title: String,
    pub language: String,
    pub code: String,
}

impl CodeTab {
    pub fn new(id: i16, title: String, language: String, code: String) -> Self { Self { id, title, language, code } }
    pub fn new_from_str(id: i16, title: &str, language: &str, code: &str) -> Self { Self { id, title: title.to_string(), language: language.to_string(), code: code.to_string() } }
}