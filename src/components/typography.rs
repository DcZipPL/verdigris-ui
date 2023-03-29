use std::fmt;

use color_art::Color;
use leptos::*;

#[component]
pub fn Code(cx: Scope,
    #[prop(default=false)] block: bool,
    #[prop(optional)] color: Option<HighlightColor>,
    children: Children,
) -> impl IntoView
{
    view! { cx,
        <code class="code" style={
            if let Some(c) = color {format!("background-color: {}", c)}
            else {String::new()}
        }>{children(cx)}</code>
    }
}

#[component]
pub fn Mark(cx: Scope,
    #[prop(optional)] color: Option<HighlightColor>,
    children: Children,
) -> impl IntoView
{
    view! { cx,
        <span style={
            if let Some(c) = color {format!("background-color: {}", c)}
            else {"background-color: #ffec99".to_string()}
        }>
            {children(cx)}
        </span>
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HighlightColor {
    Yellow, Green, Blue, Red, Purple, Orange, Gray,
    Color(Color)
}

impl fmt::Display for HighlightColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HighlightColor::Yellow => write!(f, "#ffec99"),
            HighlightColor::Green => write!(f, "#b3ff99"),
            HighlightColor::Blue => write!(f, "#99ffff"),
            HighlightColor::Red => write!(f, "#ff9999"),
            HighlightColor::Purple => write!(f, "#ff99ff"),
            HighlightColor::Orange => write!(f, "#ffcc99"),
            HighlightColor::Gray => write!(f, "#cccccc"),
            HighlightColor::Color(color) => write!(f, "{}", color.hex()),
        }
    }
}