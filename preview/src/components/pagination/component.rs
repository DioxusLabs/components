use dioxus::prelude::*;

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum PaginationLinkSize {
    #[default]
    Icon,
    Default,
}

impl PaginationLinkSize {
    pub fn class(&self) -> &'static str {
        match self {
            PaginationLinkSize::Icon => "icon",
            PaginationLinkSize::Default => "default",
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum PaginationLinkKind {
    Previous,
    Next,
}

impl PaginationLinkKind {
    pub fn attr(&self) -> &'static str {
        match self {
            PaginationLinkKind::Previous => "previous",
            PaginationLinkKind::Next => "next",
        }
    }
}

#[component]
pub fn Pagination(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        nav {
            class: "pagination",
            "data-slot": "pagination",
            role: "navigation",
            aria_label: "pagination",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn PaginationContent(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        ul {
            class: "pagination-content",
            "data-slot": "pagination-content",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn PaginationItem(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        li {
            class: "pagination-item",
            "data-slot": "pagination-item",
            ..attributes,
            {children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PaginationLinkProps {
    #[props(default)]
    pub is_active: bool,
    #[props(default)]
    pub size: PaginationLinkSize,
    #[props(default)]
    pub data_kind: Option<PaginationLinkKind>,
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = a)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

#[component]
pub fn PaginationLink(props: PaginationLinkProps) -> Element {
    let aria_current = if props.is_active { Some("page") } else { None };
    let data_kind = props.data_kind.map(|kind| kind.attr());
    rsx! {
        a {
            class: "pagination-link",
            "data-slot": "pagination-link",
            "data-active": props.is_active,
            "data-size": props.size.class(),
            "data-kind": data_kind,
            aria_current: aria_current,
            onclick: move |event| {
                if let Some(f) = &props.onclick {
                    f.call(event);
                }
            },
            onmousedown: move |event| {
                if let Some(f) = &props.onmousedown {
                    f.call(event);
                }
            },
            onmouseup: move |event| {
                if let Some(f) = &props.onmouseup {
                    f.call(event);
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn PaginationPrevious(
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = a)]
    attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        PaginationLink {
            size: PaginationLinkSize::Default,
            aria_label: "Go to previous page",
            data_kind: Some(PaginationLinkKind::Previous),
            onclick,
            onmousedown,
            onmouseup,
            attributes,
            // ChevronLeft icon from lucide https://lucide.dev/icons/chevron-left
            svg {
                class: "pagination-icon",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                polyline { points: "15 6 9 12 15 18" }
            }
            span { class: "pagination-label", "Previous" }
        }
    }
}

#[component]
pub fn PaginationNext(
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = a)]
    attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        PaginationLink {
            size: PaginationLinkSize::Default,
            aria_label: "Go to next page",
            data_kind: Some(PaginationLinkKind::Next),
            onclick,
            onmousedown,
            onmouseup,
            attributes,
            span { class: "pagination-label", "Next" }
            // ChevronRight icon from lucide https://lucide.dev/icons/chevron-right
            svg {
                class: "pagination-icon",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                polyline { points: "9 6 15 12 9 18" }
            }
        }
    }
}

#[component]
pub fn PaginationEllipsis(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        span {
            class: "pagination-ellipsis",
            "data-slot": "pagination-ellipsis",
            aria_hidden: "true",
            ..attributes,
            // MoreHorizontal icon from lucide https://lucide.dev/icons/more-horizontal
            svg {
                class: "pagination-icon",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                circle { cx: "5", cy: "12", r: "1.5" }
                circle { cx: "12", cy: "12", r: "1.5" }
                circle { cx: "19", cy: "12", r: "1.5" }
            }
            span { class: "sr-only", "More pages" }
        }
    }
}
