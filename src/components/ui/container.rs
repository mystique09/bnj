use leptos::{html::Div, prelude::*};
use tailwind_fuse::tw_merge;

type Ref = NodeRef<Div>;

#[component]
pub fn Container<'a>(
    #[prop(optional)] class: &'a str,
    #[prop(optional)] _ref: Ref,
    #[prop(optional)] max_width: &'a str,
    children: Children,
) -> impl IntoView {
    let default_ref: Ref = NodeRef::new();
    let prop_ref = move || _ref.get_untracked();
    let _node_ref = if prop_ref().is_some() {
        _ref
    } else {
        default_ref
    };

    let _class = tw_merge!(
        "relative w-full h-full mx-auto max-w-sm",
        format!("max-w-{}", max_width),
        class,
    );

    view! {
        <div node_ref=_node_ref class=_class>
            {children()}
        </div>
    }
}
