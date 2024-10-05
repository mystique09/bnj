use html::Div;
use leptos::*;
use tailwind_fuse::{tw_merge, AsTailwindClass, TwVariant};

type Ref = NodeRef<Div>;

#[derive(TwVariant)]
pub enum PlaceContent {
    #[tw(class = "place-content-start")]
    Start,
    #[tw(class = "place-content-center")]
    Center,
    #[tw(class = "place-content-end")]
    End,
    #[tw(class = "place-content-between")]
    Between,
    #[tw(class = "place-content-around")]
    Around,
    #[tw(class = "place-content-evenly")]
    Evenly,
    #[tw(class = "place-content-stretch")]
    Stretch,
    #[tw(class = "place-content-baseline")]
    Baseline,
    #[tw(default, class = "")]
    None,
}

impl<'a> From<&'a str> for PlaceContent {
    fn from(s: &str) -> Self {
        match s {
            "start" => Self::Start,
            "center" => Self::Center,
            "end" => Self::End,
            "between" => Self::Between,
            "around" => Self::Around,
            "evenly" => Self::Evenly,
            "stretch" => Self::Stretch,
            _ => {
                logging::debug_warn!("invalid place content: {}", s);
                Self::None
            }
        }
    }
}

#[derive(TwVariant)]
pub enum PlaceItems {
    #[tw(class = "place-items-start")]
    Start,
    #[tw(class = "place-items-center")]
    Center,
    #[tw(class = "place-items-end")]
    End,
    #[tw(class = "place-items-stretch")]
    Stretch,
    #[tw(class = "place-items-baseline")]
    Baseline,
    #[tw(default, class = "")]
    None,
}

impl From<&str> for PlaceItems {
    fn from(s: &str) -> Self {
        match s {
            "start" => Self::Start,
            "center" => Self::Center,
            "end" => Self::End,
            "stretch" => Self::Stretch,
            _ => {
                logging::debug_warn!("invalid place items: {}", s);
                Self::None
            }
        }
    }
}

#[derive(TwVariant)]
pub enum PlaceSelf {
    #[tw(class = "place-self-start")]
    Start,
    #[tw(class = "place-self-center")]
    Center,
    #[tw(class = "place-self-end")]
    End,
    #[tw(class = "place-self-stretch")]
    Stretch,
    #[tw(class = "place-self-auto")]
    Auto,
    #[tw(class = "place-self-baseline")]
    Baseline,
    #[tw(default, class = "")]
    None,
}

impl From<&str> for PlaceSelf {
    fn from(s: &str) -> Self {
        match s {
            "start" => Self::Start,
            "center" => Self::Center,
            "end" => Self::End,
            "stretch" => Self::Stretch,
            "auto" => Self::Auto,
            _ => {
                logging::debug_warn!("invalid place self: {}", s);
                Self::None
            }
        }
    }
}

#[component]
pub fn Grid<'a>(
    #[prop(optional)] _ref: Ref,
    #[prop(optional)] class: &'a str,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    #[prop(optional)] cols: u8,
    #[prop(optional)] rows: u8,
    #[prop(optional)] gap: u8,
    #[prop(optional)] place_content: PlaceContent,
    #[prop(optional)] place_items: PlaceItems,
    #[prop(optional)] place_self: PlaceSelf,
    children: Children,
) -> impl IntoView {
    let _node_ref: Ref = create_node_ref();
    let prop_ref = move || _ref.get_untracked();
    let __ref = if prop_ref().is_none() {
        _node_ref
    } else {
        _ref
    };
    let _gap = format!("gap-{}", gap);
    let _cols = format!("grid-cols-{}", cols);
    let _rows = format!("grid-rows-{}", rows);
    let _class = tw_merge!(
        "grid",
        _gap,
        _cols,
        _rows,
        place_self,
        place_items,
        place_content,
        class
    );

    view! {
        <div {..attributes} node_ref=__ref class=_class>
            {children()}
        </div>
    }
}
