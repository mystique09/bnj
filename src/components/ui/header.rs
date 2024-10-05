use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="w-full h-fit max-w-3xl mx-auto py-20 px-4 xl:px-0">
            <div class="flex items-center justify-between h-12 bg-white px-12 rounded-full shadow-lg">
                <Logo />
                <NavLinks />
            </div>
        </header>
    }
}

#[component]
fn Logo() -> impl IntoView {
    view! {
        <div class="logo relative w-fit">
            <h1
                class="relative w-fit text-primary font-bold font-mono after:content-[''] after:absolute after:-right-40 after:top-1/2 after:w-40 after:h-px after:bg-primary"
            >
                <a href="/"> Benj </a>
            </h1>
        </div>
    }
}

#[component]
fn NavLinks() -> impl IntoView {
    view! {
        <nav>
            <ul class="flex items-center justify-center gap-4 font-mono text-accent font-bold">
                <li>
                    <a href="/#projects">Projects</a>
                </li>
                <li>
                    <a href="/#contact">Contact</a>
                </li>
            </ul>
        </nav>
    }
}
