use leptos::*;
use styled::style;

#[component]
pub fn Grid(cx: Scope,
    children: Children,
    #[prop(optional, into)] style: String,
) -> impl IntoView
{
    let styles = style!(
        div {
            display: grid;
            grid-gap: 1rem;
            grid-template-columns: auto auto auto;
            padding: 1rem;
        }
    );

    styled::view! { cx, styles,
        <div style=style>
            {children(cx)}
        </div>
    }
}