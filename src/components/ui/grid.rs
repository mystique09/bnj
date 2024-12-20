use leptos::{html::Div, logging, prelude::*};
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

#[derive(TwVariant)]
pub enum Gap {
    #[tw(default, class = "")]
    None,
    #[tw(class = "gap-0")]
    Zero,
    #[tw(class = "gap-1")]
    One,
    #[tw(class = "gap-2")]
    Two,
    #[tw(class = "gap-3")]
    Three,
    #[tw(class = "gap-4")]
    Four,
    #[tw(class = "gap-5")]
    Five,
    #[tw(class = "gap-6")]
    Six,
    #[tw(class = "gap-7")]
    Seven,
    #[tw(class = "gap-8")]
    Eight,
    #[tw(class = "gap-9")]
    Nine,
    #[tw(class = "gap-10")]
    Ten,
    #[tw(class = "gap-11")]
    Eleven,
    #[tw(class = "gap-12")]
    Twelve,
    #[tw(class = "gap-14")]
    Fourteen,
    #[tw(class = "gap-16")]
    Sixteen,
}

impl From<u8> for Gap {
    fn from(s: u8) -> Self {
        match s {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Ten,
            11 => Self::Eleven,
            12 => Self::Twelve,
            14 => Self::Fourteen,
            16 => Self::Sixteen,
            _ => {
                logging::debug_warn!("invalid gap: {}", s);
                Self::None
            }
        }
    }
}

#[derive(TwVariant)]
pub enum Cols {
    #[tw(class = "grid-cols-1")]
    One,
    #[tw(class = "grid-cols-2")]
    Two,
    #[tw(class = "grid-cols-3")]
    Three,
    #[tw(class = "grid-cols-4")]
    Four,
    #[tw(class = "grid-cols-5")]
    Five,
    #[tw(class = "grid-cols-6")]
    Six,
    #[tw(class = "grid-cols-7")]
    Seven,
    #[tw(class = "grid-cols-8")]
    Eight,
    #[tw(class = "grid-cols-9")]
    Nine,
    #[tw(class = "grid-cols-10")]
    Ten,
    #[tw(class = "grid-cols-11")]
    Eleven,
    #[tw(class = "grid-cols-12")]
    Twelve,
    #[tw(default, class = "")]
    None,
}

impl From<u8> for Cols {
    fn from(s: u8) -> Self {
        match s {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Ten,
            11 => Self::Eleven,
            12 => Self::Twelve,
            _ => {
                logging::debug_warn!("invalid cols: {}", s);
                Self::None
            }
        }
    }
}

#[derive(TwVariant)]
pub enum Rows {
    #[tw(class = "grid-rows-1")]
    One,
    #[tw(class = "grid-rows-2")]
    Two,
    #[tw(class = "grid-rows-3")]
    Three,
    #[tw(class = "grid-rows-4")]
    Four,
    #[tw(class = "grid-rows-5")]
    Five,
    #[tw(class = "grid-rows-6")]
    Six,
    #[tw(class = "grid-rows-7")]
    Seven,
    #[tw(class = "grid-rows-8")]
    Eight,
    #[tw(class = "grid-rows-9")]
    Nine,
    #[tw(class = "grid-rows-10")]
    Ten,
    #[tw(class = "grid-rows-11")]
    Eleven,
    #[tw(class = "grid-rows-12")]
    Twelve,
    #[tw(default, class = "")]
    None,
}

impl From<u8> for Rows {
    fn from(s: u8) -> Self {
        match s {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Ten,
            11 => Self::Eleven,
            12 => Self::Twelve,
            _ => {
                logging::debug_warn!("invalid rows: {}", s);
                Self::None
            }
        }
    }
}

#[component]
pub fn Grid<'a>(
    #[prop(optional)] _ref: Ref,
    #[prop(optional)] class: &'a str,
    #[prop(optional)] cols: Cols,
    #[prop(optional)] rows: Rows,
    #[prop(optional)] gap: Gap,
    #[prop(optional)] place_content: PlaceContent,
    #[prop(optional)] place_items: PlaceItems,
    #[prop(optional)] place_self: PlaceSelf,
    children: Children,
) -> impl IntoView {
    let _node_ref: Ref = NodeRef::new();
    let prop_ref = move || _ref.get_untracked();
    let __ref = if prop_ref().is_none() {
        _node_ref
    } else {
        _ref
    };
    let _class = tw_merge!(
        "grid",
        cols,
        rows,
        gap,
        place_self,
        place_items,
        place_content,
        class
    );

    view! {
        <div node_ref=__ref class=_class>
            {children()}
        </div>
    }
}
