use leptos::*;
use styled::style;

use crate::{theme::{Theme, HighlightColor}, components::input::button::Button};

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
    println!("CodeBlockTab");
    let (current, set_current) = create_signal(cx, 0i16);

    view! { cx,
        <Button on:click=move |_| set_current.update(|v| *v += 1)>"h"</Button>
        <For
            each=data.clone()
            key=|item| item.id
            view=move |cx, item: CodeTab| {
                view! { cx,
                    <button on:click=move |_| set_current.update(|v| *v += 1) id=format!("btn{}", item.id)>{item.title}"h"</button>
                }
            }
        />
        <For
            each=data
            key=|item| item.id
            view=move |cx, item: CodeTab| {
                view! { cx,
                    <CodeBlock style=format!("display: {};", if current.get() == item.id { "block" } else { "none" })>
                        {item.code}
                    </CodeBlock>
                }
            }
        />
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