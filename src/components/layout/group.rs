use leptos::*;

use crate::components::{layout::{Justify, Align, flex::*}, Size};

#[component]
pub fn Group(cx: Scope,
    #[prop(default=Position::Center)] position: Position,
    #[prop(default=Size::Medium)] gap: Size,
    #[prop(default=false)] grow: bool,
    children: Children,
) -> impl IntoView
{
    let styles = style!(
        div {
            display: flex;
            justify-content: ${
                match position {
                    Position::Left => "flex-start",
                    Position::Center => "center",
                    Position::Right => "flex-end",
                    Position::Apart => "space-between",
                }
            };
            gap: ${gap.space()};
        }
    );

    styled::view! { cx, styles,
        <Flex align=align justify=justify gap=gap direction=crate::components::layout::Direction::Row>
            {children(cx)}
        </Flex>
    }
}