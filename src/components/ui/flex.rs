use html::Div;
use leptos::*;
use tailwind_fuse::{tw_merge, AsTailwindClass, TwVariant};

type Ref = NodeRef<Div>;

#[derive(TwVariant)]
pub enum FlexDirection {
    #[tw(default, class = "flex-row")]
    Row,
    #[tw(class = "flex-col")]
    Col,
    #[tw(class = "flex-row-reverse")]
    ReverseRow,
    #[tw(class = "flex-col-reverse")]
    ReverseCol,
    #[tw(class = "")]
    None,
}

impl<'a> From<&'a str> for FlexDirection {
    fn from(s: &str) -> Self {
        match s {
            "row" => Self::Row,
            "col" => Self::Col,
            "row-reverse" => Self::ReverseRow,
            "col-reverse" => Self::ReverseCol,
            _ => {
                logging::debug_warn!("invalid flex direction: {}", s);
                Self::None
            }
        }
    }
}

#[derive(TwVariant)]
pub enum Align {
    #[tw(class = "items-start")]
    Start,
    #[tw(class = "items-center")]
    Center,
    #[tw(class = "items-end")]
    End,
    #[tw(class = "self-start")]
    SelfStart,
    #[tw(class = "self-center")]
    SelfCenter,
    #[tw(class = "self-end")]
    SelfEnd,
    #[tw(class = "items-baseline")]
    Baseline,
    #[tw(default, class = "")]
    None,
}

impl<'a> From<&'a str> for Align {
    fn from(s: &str) -> Self {
        match s {
            "start" => Self::Start,
            "center" => Self::Center,
            "end" => Self::End,
            "self-start" => Self::SelfStart,
            "self-center" => Self::SelfCenter,
            "self-end" => Self::SelfEnd,
            "" => Self::None,
            _ => {
                logging::debug_warn!("invalid flex alignment: {}", s);
                Self::None
            }
        }
    }
}

#[derive(TwVariant)]
pub enum Justify {
    #[tw(class = "justify-start")]
    Start,
    #[tw(class = "justify-center")]
    Center,
    #[tw(class = "justify-end")]
    End,
    #[tw(class = "justify-self-start")]
    SelfStart,
    #[tw(class = "justify-self-center")]
    SelfCenter,
    #[tw(class = "justify-self-end")]
    SelfEnd,
    #[tw(default, class = "")]
    None,
}

impl<'a> From<&'a str> for Justify {
    fn from(s: &str) -> Self {
        match s {
            "start" => Self::Start,
            "center" => Self::Center,
            "end" => Self::End,
            "self-start" => Self::SelfStart,
            "self-center" => Self::SelfCenter,
            "self-end" => Self::SelfEnd,
            "" => Self::None,
            _ => {
                logging::debug_warn!("invalid flex alignment: {}", s);
                Self::None
            }
        }
    }
}

#[component]
pub fn Flex<'a>(
    #[prop(optional)] _ref: Ref,
    #[prop(optional)] class: &'a str,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    #[prop(optional)] direction: FlexDirection,
    #[prop(optional)] align: Align,
    #[prop(optional)] justify: Justify,
    #[prop(optional)] gap: u8,
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
    let _class = tw_merge!("flex", direction, _gap, align, justify, class);

    return view! {
        <div {..attributes} node_ref=__ref class=_class>
            {children()}
        </div>
    };
}
