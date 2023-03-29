use leptos::*;

use crate::components::{layout::{Justify, Align, flex::*}, Size};

#[component]
pub fn Stack(cx: Scope,
    #[prop(default=Justify::Center)] justify: Justify,
    #[prop(default=Align::Center)] align: Align,
    #[prop(default=Size::Medium)] gap: Size,
    children: Children,
) -> impl IntoView
{
    view! { cx,
        <Flex align=align justify=justify gap=gap direction=crate::components::layout::Direction::Column>
            {children(cx)}
        </Flex>
    }
}