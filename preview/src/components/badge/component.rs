use dioxus::prelude::*;

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum BadgeVariant {
    #[default]
    Primary,
    Secondary,
    Destructive,
    Outline,
}

impl BadgeVariant {
    pub fn class(&self) -> &'static str {
        match self {
            BadgeVariant::Primary => "primary",
            BadgeVariant::Secondary => "secondary",
            BadgeVariant::Destructive => "destructive",
            BadgeVariant::Outline => "outline",
        }
    }
}

/// The props for the [`Badge`] component.
#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    #[props(default)]
    pub variant: BadgeVariant,

    /// Additional attributes to extend the badge element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the badge element
    pub children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        BadgeElement {
            "padding": true,
            variant: props.variant,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`NotifyBadge`] component.
#[derive(Props, Clone, PartialEq)]
pub struct BadgeNotifyProps {
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

    #[props(default = BadgeVariant::Destructive)]
    pub variant: BadgeVariant,

    /// Additional attributes to extend the badge element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the badge element
    pub children: Element,
}

#[component]
pub fn NotifyBadge(props: BadgeNotifyProps) -> Element {
    let text = if props.dot {
        String::default()
    } else if props.overflow_count < props.count {
        format!("{}+", props.overflow_count)
    } else {
        format!("{}", props.count)
    };

    let add_padding = text.chars().count() > 1;

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        span {
            {props.children}

            if props.count > 0 || props.show_zero {
                BadgeElement {
                    class: "badge",
                    "padding": if add_padding { true },
                    "dot": if props.dot { true },
                    "notify": true,
                    variant: props.variant,
                    attributes: props.attributes,
                    {text}
                }
            }
        }
    }
}

#[component]
fn BadgeElement(props: BadgeProps) -> Element {
    rsx! {
        span {
            class: "badge",
            "data-style": props.variant.class(),
            ..props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn CardIcon() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            fill: "none",
            stroke: "var(--secondary-color-4)",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: 2,
            circle { cx: 8, cy: 21, r: 1 }
            circle { cx: 19, cy: 21, r: 1 }
            path { d: "M2.05 2.05h2l2.66 12.42a2 2 0 0 0 2 1.58h9.78a2 2 0 0 0 1.95-1.57l1.65-7.43H5.12" }
        }
    }
}

#[component]
pub fn VerifiedIcon() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            width: "12",
            height: "12",
            fill: "none",
            stroke: "var(--secondary-color-4)",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: 2,
            path { d: "M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" }
            path { d: "m9 12 2 2 4-4" }
        }
    }
}
