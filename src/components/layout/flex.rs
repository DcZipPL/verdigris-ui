use leptos::*;

use crate::components::{layout::{Justify, Align, Direction, Wrap}, Size};

#[component]
pub fn Flex(cx: Scope,
    #[prop(default=Size::Medium)] gap: Size,
    #[prop(default=Justify::Center)] justify: Justify,
    #[prop(default=Align::Center)] align: Align,
    #[prop(default=Direction::Row)] direction: Direction,
    #[prop(default=Wrap::Wrap)] wrap: Wrap,
    children: Children,
) -> impl IntoView
{
    let gap = gap.to_string().replace("size", "gap");
    let justify_style = match justify {
        Justify::FlexStart => "justify-content: flex-start;",
        Justify::Center => "justify-content: center;",
        Justify::FlexEnd => "justify-content: flex-end;",
    };

    let align_style = match align {
        Align::FlexStart => "align-items: flex-start;",
        Align::Center => "align-items: center;",
        Align::FlexEnd => "align-items: flex-end;",
    };

    let direction_style = match direction {
        Direction::Row => "flex-direction: row;",
        Direction::Column => "flex-direction: column;",
        Direction::RowReverse => "flex-direction: row-reverse;",
        Direction::ColumnReverse => "flex-direction: column-reverse;",
    };

    let wrap_style = match wrap {
        Wrap::Wrap => "flex-wrap: wrap;",
        Wrap::Nowrap => "flex-wrap: nowrap;",
        Wrap::WrapReverse => "flex-wrap: wrap-reverse;",
    };

    let style = format!("display: flex; {}{}{}{}", justify_style, align_style, direction_style, wrap_style);

    view! { cx,
        <div class="flex" class=gap style=style>
            {children(cx)}
        </div>
    }
}