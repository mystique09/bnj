use leptos::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Heading<'a>(
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    #[prop(optional)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let _class = tw_merge!("relative w-fit overflow-hidden text-sm text-left", class);

    view! {
        <h1 {..attributes} class=_class>{children()}</h1>
    }
}
