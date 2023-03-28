use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::ui::Padding;
use crate::ui::Size;
use crate::ui::Variant;
use crate::ui::display::alert::*;
use crate::ui::display::badge::*;
use crate::ui::display::card::*;
use crate::ui::input::button::*;
use crate::ui::layout::flex::*;
use crate::ui::layout::grid::*;
use crate::ui::layout::space::*;
use crate::ui::typography::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"Verdigris UI"</h1>
        <h2>"Grid"</h2>
        <Grid>
            <div class="preview-background">
                <div class=Padding::Large.to_string()>
                    <h2>"Card"</h2>
                    <Card padding=Padding::Medium>
                        <p>"This is a card"</p>
                    </Card>
                    <Space height=Size::Medium/>
                    <Card padding=Padding::Medium hover_tilt=true>
                        <p>"This is a card has "<Code>"hover-tilt"</Code>" attribute"</p>
                    </Card>
                    <Space height=Size::Medium/>
                    <Card padding=Padding::Medium border=true>
                        <p>"This is a card with border"</p>
                    </Card>
                </div>
            </div>
            <div>
                <h2>"Alert"</h2>
                <Alert variant=Variant::Light title="Bummer!">
                    <p>"This is an alert!"</p>
                </Alert>
            </div>
            <div>
                <h2>"Button"</h2>
                <Button variant=Variant::Filled>"Filled Button"</Button><Space width=Size::Medium/>
                <Button variant=Variant::Outline>"Outline Button"</Button><Space width=Size::Medium/>
                <Button variant=Variant::Light>"Light Button"</Button><Space width=Size::Medium/>
                <Button variant=Variant::Subtle>"Subtle Button"</Button><Space width=Size::Medium/>
            </div>
            <div>
                <h2>"Badge"</h2>
                <Badge variant=BadgeVariant::Filled>"Filled"</Badge><Space width=Size::Medium/>
                <Badge variant=BadgeVariant::Outline>"Outline"</Badge><Space width=Size::Medium/>
                <Badge variant=BadgeVariant::Light>"Light"</Badge><Space width=Size::Medium/>
                <Badge variant=BadgeVariant::Dot>"Dot"</Badge><Space width=Size::Medium/>
            </div>
            <Flex>
                <h2>"Flex"</h2>
                <Space width=Size::Medium/>
                "A container"
            </Flex>
            <div>
                <h2>"Typography"</h2>
                <h1>"Heading 1"</h1>
                <h2>"Heading 2"</h2>
                <h3>"Heading 3"</h3>
                <h4>"Heading 4"</h4>
                <h5>"Heading 5"</h5>
                <h6>"Heading 6"</h6>
                <p>"Paragraph"</p>
                <Mark>"Mark"</Mark>
                <Space height=Size::Medium/>
                <Code>"Code"</Code>
                <Space height=Size::Medium/>
                "This "<Mark color=HighlightColor::Purple>"is example"</Mark>" "<Mark>"text"</Mark>" with "<Mark color=HighlightColor::Blue>"markings"</Mark>" and custom "<Code color=HighlightColor::Orange>"inline code block"</Code>""
            </div>
        </Grid>
        <Button on:click=on_click variant=Variant::Filled>"Click Me: " {count}</Button>
    }
}