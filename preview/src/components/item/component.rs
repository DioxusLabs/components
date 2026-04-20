use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

impl ItemVariant {
    pub fn class(&self) -> &'static str {
        match self {
            ItemVariant::Default => "default",
            ItemVariant::Outline => "outline",
            ItemVariant::Muted => "muted",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ItemSize {
    #[default]
    Default,
    Sm,
    Xs,
}

impl ItemSize {
    pub fn class(&self) -> &'static str {
        match self {
            ItemSize::Default => "default",
            ItemSize::Sm => "sm",
            ItemSize::Xs => "xs",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ItemMediaVariant {
    #[default]
    Default,
    Icon,
    Image,
}

impl ItemMediaVariant {
    pub fn class(&self) -> &'static str {
        match self {
            ItemMediaVariant::Default => "default",
            ItemMediaVariant::Icon => "icon",
            ItemMediaVariant::Image => "image",
        }
    }
}

#[component]
pub fn ItemGroup(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "item-group",
        role: "list",
        "data-slot": "item-group",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { ..merged,{children} }
    }
}

#[component]
pub fn ItemSeparator(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
) -> Element {
    let base = attributes!(div {
        class: "item-separator",
        role: "separator",
        "data-slot": "item-separator",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { ..merged }
    }
}

#[component]
pub fn Item(
    #[props(default)] variant: ItemVariant,
    #[props(default)] size: ItemSize,
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "item",
        "data-slot": "item",
        "data-style": variant.class(),
        "data-size": size.class(),
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            onclick: move |event| {
                if let Some(f) = &onclick {
                    f.call(event);
                }
            },
            ..merged,
            {children}
        }
    }
}

#[component]
pub fn ItemMedia(
    #[props(default)] variant: ItemMediaVariant,
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "item-media",
        "data-slot": "item-media",
        "data-style": variant.class(),
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged,{children} }
    }
}

#[component]
pub fn ItemContent(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "item-content",
        "data-slot": "item-content",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged,{children} }
    }
}

#[component]
pub fn ItemTitle(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "item-title",
        "data-slot": "item-title",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged,{children} }
    }
}

#[component]
pub fn ItemDescription(
    #[props(extends=GlobalAttributes)]
    #[props(extends=p)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(p {
        class: "item-description",
        "data-slot": "item-description",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        p { ..merged,{children} }
    }
}

#[component]
pub fn ItemActions(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "item-actions",
        "data-slot": "item-actions",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged,{children} }
    }
}

#[component]
pub fn ItemHeader(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "item-header",
        "data-slot": "item-header",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged,{children} }
    }
}

#[component]
pub fn ItemFooter(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "item-footer",
        "data-slot": "item-footer",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged,{children} }
    }
}
