use dioxus_lib::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ScrollAreaProps {
    /// The scroll direction.
    #[props(default)]
    direction: ReadOnlySignal<ScrollDirection>,

    /// Whether the scrollbars should be always visible.
    #[props(default)]
    always_show_scrollbars: ReadOnlySignal<bool>,

    /// The scroll type.
    #[props(default)]
    scroll_type: ReadOnlySignal<ScrollType>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ScrollDirection {
    Vertical,
    Horizontal,
    Both,
}

impl Default for ScrollDirection {
    fn default() -> Self {
        Self::Both
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ScrollType {
    /// Browser default scrolling
    Auto,
    /// Always show scrollbars
    Always,
    /// Hide scrollbars but enable scrolling
    Hidden,
}

impl Default for ScrollType {
    fn default() -> Self {
        Self::Auto
    }
}

#[component]
pub fn ScrollArea(props: ScrollAreaProps) -> Element {
    let direction = props.direction;
    let scroll_type = props.scroll_type;
    let always_show = props.always_show_scrollbars;

    let overflow_style = use_memo(move || match scroll_type() {
        ScrollType::Auto => match direction() {
            ScrollDirection::Vertical => "overflow-y: auto; overflow-x: hidden;",
            ScrollDirection::Horizontal => "overflow-x: auto; overflow-y: hidden;",
            ScrollDirection::Both => "overflow: auto;",
        },
        ScrollType::Always => match direction() {
            ScrollDirection::Vertical => "overflow-y: scroll; overflow-x: hidden;",
            ScrollDirection::Horizontal => "overflow-x: scroll; overflow-y: hidden;",
            ScrollDirection::Both => "overflow: scroll;",
        },
        ScrollType::Hidden => match direction() {
            ScrollDirection::Vertical => {
                "overflow-y: scroll; overflow-x: hidden; scrollbar-width: none;"
            }
            ScrollDirection::Horizontal => {
                "overflow-x: scroll; overflow-y: hidden; scrollbar-width: none;"
            }
            ScrollDirection::Both => "overflow: scroll; scrollbar-width: none;",
        },
    });

    let visibility_class = use_memo(move || {
        if always_show() {
            "scroll-area-always-show"
        } else {
            "scroll-area-auto-hide"
        }
    });

    rsx! {
        div {
            class: "{visibility_class}",
            style: "{overflow_style}",
            "data-scroll-direction": match direction() {
                ScrollDirection::Vertical => "vertical",
                ScrollDirection::Horizontal => "horizontal",
                ScrollDirection::Both => "both",
            },
            ..props.attributes,

            {props.children}
        }
    }
}
