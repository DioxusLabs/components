//! Defines the [`Badge`] component

use dioxus::prelude::*;

const DEF_COLOR: &str = "EB5160";

/// The props for the [`Badge`] component.
#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    /// Number to show in badge
    pub count: u32,

    /// Max count to show
    #[props(default = u32::MAX)]
    pub overflow_count: u32,

    /// Whether to display a dot instead of count
    #[props(default = false)]
    pub dot: bool,

    /// Whether to show badge when count is zero
    #[props(default = false)]
    pub show_zero: bool,

    /// Customize Badge color (as HEX)
    #[props(default = String::from(DEF_COLOR))]
    pub color: String,

    /// Additional attributes to extend the badge element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the badge element
    pub children: Element,
}

/// # Badge
///
/// The [`Badge`] component displays a small badge to the top-right of its child(ren).
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::badge::Badge;
/// use dioxus_primitives::avatar::*;
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Badge {
///             count: 100,
///             overflow_count: 99,
///             Avatar {
///                 aria_label: "Space item",
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let text = if props.dot {
        String::default()
    } else if props.overflow_count < props.count {
        format!("{}+", props.overflow_count)
    } else {
        format!("{}", props.count)
    };

    let add_padding = text.chars().count() > 1;
    let color = if u32::from_str_radix(&props.color, 16).is_ok() {
        props.color
    } else {
        DEF_COLOR.to_string()
    };

    rsx! {
        span {
            {props.children}

            if props.count > 0 || props.show_zero {
                span {
                    class: "badge",
                    style: "--badge-color: #{color}",
                    "padding": if add_padding { true },
                    "dot": if props.dot { true },
                    ..props.attributes,
                    {text}
                }
            }
        }
    }
}
