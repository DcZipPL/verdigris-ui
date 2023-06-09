use leptos::*;

use crate::components::{layout::{Justify, Align, flex::*, Direction}, Size};

#[component]
pub fn Stack(cx: Scope,
    #[prop(default=Justify::Center)] justify: Justify,
    #[prop(default=Align::Center)] align: Align,
    #[prop(default=Size::Medium)] gap: Size,
    children: Children,
    #[prop(optional, into)] style: String,
) -> impl IntoView
{
    view! { cx,
        <Flex align=align justify=justify gap=gap direction=Direction::Column style=style>
            {children(cx)}
        </Flex>
    }
}