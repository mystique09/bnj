use crate::{
    components::ui::header::Header,
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
    view! {
        <section class="max-w-3xl mx-auto">
            <h1 class="text-primary font-bold font-mono text-4xl">Hello, world!</h1>
        </section>
    }
}

#[component]
fn TestPage() -> impl IntoView {
    view! {
        <h1>"Test Page"</h1>
    }
}
