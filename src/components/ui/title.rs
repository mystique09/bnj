use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Heading<'a>(#[prop(optional)] class: &'a str, children: Children) -> impl IntoView {
    let _class = tw_merge!("relative w-fit overflow-hidden text-sm text-left", class);

    view! {
        <h1  class=_class>{children()}</h1>
    }
}
