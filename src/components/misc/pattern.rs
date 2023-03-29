use leptos::*;
use styled::style;

#[component]
pub fn Pattern(cx: Scope,
    #[prop(default=PatternType::Grid)] pattern_type: PatternType,
    children: Children,
) -> impl IntoView
{
    let styles = style!(
        div {
            background: ${pattern_type.to_string()};
            background-size: 28px 28px;
        }
    );

    styled::view! { cx, styles,
        <div>
            {children(cx)}
        </div>
    }
}

pub enum PatternType{
    Dot,
    Grid,
}

impl PatternType{
    pub fn to_string(&self) -> String{
        match self {
            PatternType::Dot => format!("url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 28 28' fill='%23{}'%3E%3Ccircle cx='14' cy='14' r='2'/%3E%3C/svg%3E\")", "E2E0E7"),
            PatternType::Grid => format!("url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='28' height='28' viewBox='0 0 28 28'%3E%3Cg fill='none' fill-rule='evenodd'%3E%3Cpath fill='%23fffffff' d='M0 0h28v28H0z'/%3E%3Cpath stroke='%23{}' stroke-width='2' d='M14 0v28M0 14h28'/%3E%3C/g%3E%3C/svg%3E\")","E2E0E7"),
        }
    }
}