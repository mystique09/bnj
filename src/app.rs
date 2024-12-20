use crate::{
    components::ui::{
        container::Container, flex::Flex, grid::Grid, header::Header, title::Heading,
    },
    error_template::{AppError, ErrorTemplate},
};
use leptos::logging;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <head>
            <Stylesheet id="leptos" href="/pkg/bnj.css"/>
            <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-300-normal.woff2"/>
            <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-400-normal.woff2"/>
            <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-500-normal.woff2"/>
            <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-600-normal.woff2"/>
            <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-700-normal.woff2"/>

            <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-300-normal.woff"/>
            <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-400-normal.woff"/>
            <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-500-normal.woff"/>
            <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-600-normal.woff"/>
            <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-700-normal.woff"/>

            <Stylesheet href="/fonts/space-mono/space-mono-latin-400-normal.woff2"/>
            // <Stylesheet href="/fonts/space-mono/space-mono-latin-400-normal.woff"/>
            <Stylesheet href="/fonts/space-mono/space-mono-latin-400-italic.woff2"/>
            // <Stylesheet href="/fonts/space-mono/space-mono-latin-400-italic.woff"/>
            <Stylesheet href="/fonts/space-mono/space-mono-latin-700-normal.woff2"/>
            // <Stylesheet href="/fonts/space-mono/space-mono-latin-700-normal.woff"/>
            <Stylesheet href="/fonts/space-mono/space-mono-latin-700-italic.woff2"/>
            // <Stylesheet href="/fonts/space-mono/space-mono-latin-700-italic.woff"/>

            // sets the document title
            <Title text="Welcome to Leptos"/>
        </head>

        // content for this welcome page
        <Router>
            <Header />
            <main>
                <FlatRoutes fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::NotFound);
                    view! {
                        <ErrorTemplate outside_errors/>
                    }
                    .into_view()
                }>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("/test") view=TestPage/>
                </FlatRoutes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    logging::log!("Hello from HomePage");

    view! {
        <section class="relative w-full h-full max-w-3xl mx-auto">
            <Grid cols=12.into() rows=3.into() gap=6.into() class="w-full h-full">
                <Container max_width="[432px]" class="h-[432px] p-6 w-full bg-white rounded-2xl mx-0 shadow-sm col-start-1 col-span-8 row-start-1 row-span-8 shadow-none hover:shadow-lg duration-300 ease-in-out transition-all">
                    <Flex direction="col".into() gap=4 class="relative w-full h-full">
                        <Heading class="text-2xl font-bold uppercase text-accent">About Me.</Heading>
                        <p>
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
                        </p>
                    </Flex>
                </Container>
                <Container max_width="[240px]" class="w-full bg-white h-[240px] mx-0 p-6 rounded-2xl shadow-sm col-start-9 col-span-4 row-start-1 shadow-none hover:shadow-lg duration-300 ease-in-out transition-all">
                    <Flex direction="col".into() gap=4 class="relative w-full h-full">
                        <Heading class="text-2xl font-bold uppercase text-accent">Blogs.</Heading>
                        <p>
                            Test
                        </p>
                    </Flex>
                </Container>
                <Container max_width="[432px]" class="w-full bg-white h-[240px] p-6 rounded-2xl shadow-sm row-start-3 col-start-1 col-span-8 shadow-none hover:shadow-lg duration-300 ease-in-out transition-all">
                    <Flex direction="col".into() gap=4 class="relative w-full h-full">
                        <Heading class="text-2xl font-bold uppercase text-accent">"Let's connect."</Heading>
                        <p>
                            Test
                        </p>
                    </Flex>
                </Container>
                <Container max_width="[288px]" class="w-full bg-white h-[240px] p-6 rounded-2xl shadow-sm row-start-2 col-start-9 col-span-5 shadow-none hover:shadow-lg duration-300 ease-in-out transition-all">
                    <Flex direction="col".into() gap=4 class="relative w-full h-full">
                        <Heading class="text-2xl font-bold uppercase text-accent">Projects</Heading>
                        <p>
                            Test
                        </p>
                    </Flex>
                </Container>
            </Grid>
        </section>
    }
}

#[component]
fn TestPage() -> impl IntoView {
    logging::log!("Hello from TestPage");
    let (image_file, set_image_file) = signal::<Option<String>>(None);
    let (name, set_name) = signal("test".to_string());

    view! {
        <h1>"Test Page"</h1>
            <input
                type="file"
                accept="image/*"
                on:input:target=move |ev| {
                    ev.prevent_default();
                    logging::log!("event {:?}", ev.value_of());
                    set_name("test2".to_string());
                }
            />
            <input type="text"
            // fire an event whenever the input changes
            // adding :target after the event gives us access to
            // a correctly-typed element at ev.target()
            on:input:target=move |ev| {
                set_name.set(ev.target().value());
            }

            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            //
            // IMPORTANT: the `value` *attribute* only sets the
            // initial value, until you have made a change.
            // The `value` *property* sets the current value.
            // This is a quirk of the DOM; I didn't invent it.
            // Other frameworks gloss this over; I think it's
            // more important to give you access to the browser
            // as it really works.
            //
            // tl;dr: use prop:value for form inputs
            prop:value=name
        />
            <input type="submit" value="Upload"/>
        <img
            id="image-preview"
            src=image_file
            alt="Preview Image"
        />
        <p>{name}</p>
    }
}
