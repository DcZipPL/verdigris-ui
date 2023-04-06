use leptos::*;
use styled::style;

use crate::components::{layout::{Position, Direction, Justify, flex::*}, Size};

#[component]
pub fn Group(cx: Scope,
    /// Position of the group
    #[prop(default=Position::Center)] position: Position,
    /// Spacing between the children
    #[prop(default=Size::Medium)] gap: Size,
    #[prop(default=false)] grow: bool,
    children: Children,
    #[prop(optional, into)] style: String,
) -> impl IntoView
{
    let styles = style!(
        div {
            display: flex;
        }
    );

    let mut justify = match position {
        Position::Left => Justify::FlexStart,
        Position::Center => Justify::Center,
        Position::Right => Justify::FlexEnd,
        Position::Apart => Justify::SpaceBetween,
    };

    if grow {
        justify = Justify::Stretch;
    }

    styled::view! { cx, styles,
        <Flex justify=justify gap=gap direction=Direction::Row style=style>
            {children(cx)}
        </Flex>
    }
}