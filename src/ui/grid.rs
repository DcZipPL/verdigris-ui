use leptos::*;

#[component]
pub fn Grid(cx: Scope,
    children: Children,
) -> impl IntoView
{
    view! { cx,
        <div class="grid">
            {children(cx)}
        </div>
    }
}