use leptos::*;
use styled::style;

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
        Justify::FlexStart => "flex-start",
        Justify::Center => "center",
        Justify::FlexEnd => "flex-end",
    };

    let align_style = match align {
        Align::FlexStart => "flex-start",
        Align::Center => "center",
        Align::FlexEnd => "flex-end",
    };

    let direction_style = match direction {
        Direction::Row => "row",
        Direction::Column => "column",
        Direction::RowReverse => "row-reverse",
        Direction::ColumnReverse => "column-reverse",
    };

    let wrap_style = match wrap {
        Wrap::Wrap => "wrap",
        Wrap::Nowrap => "nowrap",
        Wrap::WrapReverse => "wrap-reverse",
    };

    let styles = style!(
        div {
            display: flex;
            justify-content: ${justify_style};
            align-items: ${align_style};
            flex-direction: ${direction_style};
            flex-wrap: ${wrap_style};
        }
    );

    styled::view! { cx, styles,
        <div class="flex" class=gap>
            {children(cx)}
        </div>
    }
}