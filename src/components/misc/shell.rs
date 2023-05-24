use leptos::*;
use styled::style;

use crate::{components::{Size, layout::{Justify, flex::*}}, theme::Theme};

#[component]
pub fn HeaderBar(cx: Scope,
    /// Height of the headerbar. Defaults to Size::Large.
    #[prop(default=Size::Large)] height: Size,
    /// Edge padding. Padding is applied to the left and right edges of the headerbar. Defaults to Size::Medium.
    #[prop(default=Size::Medium)] ep: Size,
    #[prop(default=true)] with_border: bool,
    children: Children,
    #[prop(optional, into)] style: String,
) -> impl IntoView
{
    let colors = Theme::default().colors();

    let mut border = format!("{}  0.0625rem solid", colors.shell_border.rgba());
    if !with_border {
        border = "none".to_string();
    }

    let styles = style!(
        div {
            height: ${height.headerbar_height()};
            border-bottom: ${border};
        }

        & > div {
            height: 100%;
            padding-left: 1rem;
            padding-right: 1rem;
        }
    );

    styled::view! { cx, styles,
        <div style=style>
            <Flex justify=Justify::SpaceBetween>
                {children(cx)}
            </Flex>
        </div>
    }
}