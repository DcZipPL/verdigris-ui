use leptos::*;
use styled::style;

use crate::{components::{Variant, Radius}, theme::Theme};

#[component]
pub fn Alert(cx: Scope,
    #[prop(optional, into)] title: Option<String>,
    #[prop(default=Radius::Small)] radius: Radius,
    #[prop(default=Variant::Light)] variant: Variant,
    children: Children,
    #[prop(optional, into)] style: String,
) -> impl IntoView
{
    // TODO: Implement variant

    let colors = Theme::Light.colors();

    let styles = style!(
        .alert {
            border-radius: ${radius.units()};
            padding: 0.75rem 0.75rem 0.75rem 1rem;
            background-color: ${colors.primary.clone().fade(0.1).rgba()};
            text-align: left;
        }

        & > .title {
            color: ${colors.primary.rgba()};
            font-weight: bold;
            margin-bottom: 0.25rem;
        }
    );

    styled::view! { cx, styles,
        <div class="alert" style=style>
            <div class="title">{title}</div>
            <div>
                {children(cx)}
            </div>
        </div>
    }
}