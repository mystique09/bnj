use crate::{
    components::ui::{
        container::Container, flex::Flex, grid::Grid, header::Header, title::Heading,
    },
    error_template::{AppError, ErrorTemplate},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/bnj.css"/>
        <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-wght-normal.woff2"/>
        <Stylesheet href="/fonts/space-mono/space-mono-latin-400-normal.woff2"/>
        <Stylesheet href="/fonts/space-mono/space-mono-latin-400-normal.woff"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <Header />
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/test" view=TestPage/>
                </Routes>
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
            <Grid cols=12 rows=3 gap=6 class="w-full h-full">
                <Container max_width="[432px]" class="h-[432px] p-6 w-full bg-white rounded-2xl mx-0 shadow-sm col-start-1 col-span-7 row-start-1 row-span-8 shadow-none hover:shadow-lg duration-300 ease-in-out transition-all">
                    <Flex direction="col".into() gap=4 class="relative w-full h-full">
                        <Heading class="text-2xl font-bold uppercase text-accent">About Me.</Heading>
                        <p>
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
                        </p>
                    </Flex>
                </Container>
                <Container max_width="[240px]" class="w-full bg-white h-[240px] mx-0 p-6 rounded-2xl shadow-sm col-start-8 col-span-4 row-start-1 shadow-none hover:shadow-lg duration-300 ease-in-out transition-all">
                    <Flex direction="col".into() gap=4 class="relative w-full h-full">
                        <Heading class="text-2xl font-bold uppercase text-accent">Blogs.</Heading>
                        <p>
                            Test
                        </p>
                    </Flex>
                </Container>
                <Container max_width="[432px]" class="w-full bg-white h-[240px] p-6 rounded-2xl shadow-sm row-start-3 col-start-1 col-span-7 shadow-none hover:shadow-lg duration-300 ease-in-out transition-all">
                    <Flex direction="col".into() gap=4 class="relative w-full h-full">
                        <Heading class="text-2xl font-bold uppercase text-accent">"Let's connect."</Heading>
                        <p>
                            Test
                        </p>
                    </Flex>
                </Container>
                <Container max_width="[288px]" class="w-full bg-white h-[240px] p-6 rounded-2xl shadow-sm row-start-2 col-start-8 col-span-5 shadow-none hover:shadow-lg duration-300 ease-in-out transition-all">
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
    view! {
        <h1>"Test Page"</h1>
    }
}
