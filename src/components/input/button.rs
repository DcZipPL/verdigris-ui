use leptos::*;
use leptos::leptos_dom::Transparent;
use styled::style;

use crate::components::Size;
use crate::components::Radius;
use crate::components::Unit;
use crate::components::Variant;
use crate::components::input::get_colors;

#[component]
pub fn Button(cx: Scope,
    #[prop(default=Size::Small)] size: Size,
    #[prop(default=Radius::Small)] radius: Radius,
    #[prop(default=Variant::Filled)] variant: Variant,
    #[prop(default=false)] compact: bool,
    #[prop(default=false)] disabled: bool,
    #[prop(optional, into)] left: Option<Box<dyn Fn(Scope) -> Fragment>>,
    children: Children,
    #[prop(optional, into)] style: String,
) -> impl IntoView
{
    let colors = get_colors(variant).unwrap();

    let left_childeren = if let Some(uleft) = left {
        let extensions = uleft(cx)
        .as_children()
        .iter()
        .filter_map(View::as_transparent)
        .cloned()
        .collect::<Vec<_>>();

        let left_extension = extensions
            .iter()
            .filter_map(Transparent::downcast_ref::<ButtonExtension>)
            .enumerate();

        let result = left_extension.for_each(|(_, extension)| {
            if let ButtonExtension::Left { children } = extension {
                Some(children(cx))
            }
        });
    };

    let styles = style!(
        button {
            cursor: pointer;
            height: ${
                if compact {
                    "auto".to_string()
                } else {
                    size.ib_height().to_string()
                }
            };
            padding: ${
                if compact {
                    Unit::Rem(0.25).to_string()
                } else {
                    format!("0 {}", size.ib_padding())
                }
            };
            font-size: ${size.ib_font_size()};
            border-radius: ${radius.units()};
            border: ${colors.border_prop()};
            color: ${colors.foreground_color().rgba()};
            background: ${colors.fill_color()};
        }

        &:active {
            transform: translate(0, 1px);
        }

        &:hover {
            background-color: ${colors.hover_color().rgba()};
            color: ${colors.foreground_color().rgba()};
        }
    );

    styled::view! { cx, styles,
        <button disabled=disabled
                style=style
        >
            {children(cx)}
        </button>
    }
}

#[component(transparent)]
pub fn ButtonLeft(cx: Scope,
    children: Box<dyn Fn(Scope) -> Fragment>,
) -> impl IntoView
{
    ButtonExtension::Left { children }
}

pub enum ButtonExtension {
    Left { children: Box<dyn Fn(Scope) -> Fragment>, },
    Right { children: Box<dyn Fn(Scope) -> Fragment>,},
}

impl IntoView for ButtonExtension {
    fn into_view(self, _: Scope) -> View {
      View::Transparent(Transparent::new(self))
    }
}