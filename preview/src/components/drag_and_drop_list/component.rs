use dioxus::prelude::*;
use dioxus_primitives::drag_and_drop_list::{
    self, DragAndDropContext, DragAndDropItemContext, DragAndDropListItemProps,
};
use dioxus_primitives::icon::Icon;

#[derive(Props, Clone, PartialEq)]
pub struct DragAndDropListProps {
    /// Items (labels) to be rendered.
    pub items: Vec<Element>,

    /// Set if the list items should be removable
    #[props(default)]
    pub is_removable: bool,

    /// Accessible label for the list
    #[props(default)]
    pub aria_label: Option<String>,

    /// Additional attributes to apply to the list element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the list component.
    pub children: Element,
}

#[component]
pub fn DragAndDropList(props: DragAndDropListProps) -> Element {
    let is_removable = props.is_removable;
    // Wrap each item in a `display: contents` div carrying the caller's key.
    // rsx! fragments can't hold a key, and without a stable key per item
    // Dioxus falls back to an index key — which makes reorders swap
    // *content* between <li>s instead of moving the elements themselves.
    // Stable keys keep element state (focus, transitions mid-flight)
    // attached to the content as it moves.
    let items: Vec<Element> = props
        .items
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            let key = item
                .as_ref()
                .ok()
                .and_then(|v| v.key.clone())
                .unwrap_or_else(|| idx.to_string());
            rsx! {
                div {
                    key: "{key}",
                    style: "display: contents",
                    DragIcon {}
                    div { class: "dx-item-body-div", {item} }
                    if is_removable {
                        RemoveButton {}
                    }
                }
            }
        })
        .collect();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        drag_and_drop_list::DragAndDropList {
            items,
            aria_label: props.aria_label,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn DragAndDropListItem(props: DragAndDropListItemProps) -> Element {
    rsx! {
        drag_and_drop_list::DragAndDropListItem {
            index: props.index,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
fn DragIcon() -> Element {
    rsx! {
        Icon {
            // grip-vertical from lucide https://lucide.dev/icons/grip-vertical
            class: "dx-item-icon",
            aria_hidden: "true",
            width: "16px",
            height: "16px",
            fill: "currentColor",
            stroke: "none",
            circle { cx: "9", cy: "5", r: "1.25" }
            circle { cx: "9", cy: "12", r: "1.25" }
            circle { cx: "9", cy: "19", r: "1.25" }
            circle { cx: "15", cy: "5", r: "1.25" }
            circle { cx: "15", cy: "12", r: "1.25" }
            circle { cx: "15", cy: "19", r: "1.25" }
        }
    }
}

#[component]
pub fn RemoveButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut ctx: DragAndDropContext = use_context();
    let item_ctx: DragAndDropItemContext = use_context();
    let index = item_ctx.index();
    let label = format!("Remove item {}", index + 1);
    rsx! {
        button {
            class: "dx-remove-button",
            aria_label: "{label}",
            onclick: move |_| ctx.remove(index),
            ..attributes,
            {children}
            Icon {
                // X icon from lucide https://lucide.dev/icons/x
                width: "14px",
                height: "14px",
                path { d: "M18 6 6 18" }
                path { d: "m6 6 12 12" }
            }
        }
    }
}
