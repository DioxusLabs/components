//! Defines the [`ScrollArea`] component for creating scrollable areas with customizable scrollbars.

use dioxus::prelude::*;

/// The props for the [`ScrollArea`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ScrollAreaProps {
    /// The scroll direction.
    #[props(default)]
    pub direction: ReadOnlySignal<ScrollDirection>,

    /// Whether the scrollbars should be always visible.
    #[props(default)]
    pub always_show_scrollbars: ReadOnlySignal<bool>,

    /// The scroll type.
    #[props(default)]
    pub scroll_type: ReadOnlySignal<ScrollType>,

    /// Additional attributes to apply to the scroll area element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// The children of the scroll area component.
    children: Element,
}

/// The direction in which scrolling is allowed.
#[derive(Clone, Copy, PartialEq)]
pub enum ScrollDirection {
    /// Allow vertical scrolling only.
    Vertical,
    /// Allow horizontal scrolling only.
    Horizontal,
    /// Allow scrolling in both directions.
    Both,
}

impl Default for ScrollDirection {
    fn default() -> Self {
        Self::Both
    }
}

/// The type of scrolling behavior.
#[derive(Clone, Copy, PartialEq)]
pub enum ScrollType {
    /// Browser default scrolling.
    Auto,
    /// Always show scrollbars.
    Always,
    /// Hide scrollbars but enable scrolling.
    Hidden,
}

impl Default for ScrollType {
    fn default() -> Self {
        Self::Auto
    }
}

/// # ScrollArea
///
/// The `ScrollArea` component creates a scrollable area. If you don't
/// have any focusable content within the scroll area, you should make the
/// scroll area focusable by adding a `tabindex` attribute.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::scroll_area::{ScrollArea, ScrollDirection};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         ScrollArea {
///             width: "10em",
///             height: "10em",
///             direction: ScrollDirection::Vertical,
///             tabindex: "0",
///             div { class: "scroll-content",
///                 for i in 1..=20 {
///                     p {
///                         "Scrollable content item {i}"
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`ScrollArea`] component defines the following data attributes you can use to control styling:
/// - `data-scroll-direction`: Indicates the scroll direction. Values are `vertical`, `horizontal`, or `both`.
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
