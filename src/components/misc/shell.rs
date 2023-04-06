use leptos::*;
use styled::style;

use crate::{components::{Size, layout::{Justify, flex::*}}, theme::Theme};

#[component]
pub fn HeaderBar(cx: Scope,
    #[prop(default=Size::Large)] height: Size,
    children: Children,
    #[prop(optional, into)] style: String,
) -> impl IntoView
{
    let colors = Theme::Light.colors();

    let styles = style!(
        div {
            height: ${height.headerbar_height()};
            border-bottom: ${colors.shell_border.rgba()} 0.0625rem solid;
        }
    );

    styled::view! { cx, styles,
        <div style=style>
            <Flex justify=Justify::SpaceBetween style="height: 100%;">
                {children(cx)}
            </Flex>
        </div>
    }
}