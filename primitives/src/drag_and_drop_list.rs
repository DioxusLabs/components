//! Defines the [`DragAndDropList`] component and its sub-components.
use dioxus::prelude::*;

#[derive(Clone, Copy)]
struct DragAndDropContext {
    drag_from: Signal<Option<usize>>,
    drop_to: Signal<Option<usize>>,
    is_dragging: Signal<bool>,
    original_list: Signal<Vec<Element>>,
    temp_list: Signal<Vec<Element>>,
}

impl DragAndDropContext {
    fn start_drag(&mut self, index: usize) {
        self.drag_from.set(Some(index));
        self.drop_to.set(None);
        self.is_dragging.set(true);
        self.temp_list.set((self.original_list)());
    }

    fn end_drag(&mut self) {
        self.drag_from.set(None);
        self.drop_to.set(None);
        self.is_dragging.set(false);
    }

    fn drag_over(&mut self, index: usize) {
        if (self.drop_to)().is_some_and(|to| to == index) {
            return;
        }

        self.drop_to.set(Some(index));

        let mut list = (self.original_list)();
        let from = (self.drag_from)().unwrap();
        let element = list.remove(from);
        list.insert(index, element);
        self.temp_list.set(list);
    }

    fn drop(&mut self) {
        self.original_list.set((self.temp_list)());
    }

    fn remove(&mut self, index: usize) {
        let mut list = (self.original_list)();
        if list.remove(index).is_ok() {
            self.original_list.set(list);
        }
    }
}

/// The props for the [`DragAndDropListItem`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DragAndDropListProps {
    /// Items (labels) to be rendered.
    pub items: Vec<Element>,

    /// Set if the list items should be removable
    #[props(default)]
    pub is_removable: bool,

    /// Additional attributes to apply to the list element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the list component.
    pub children: Element,
}

/// # DragAndDropList
///
/// A list can be used to display content related to a single subject.
/// The content can consist of multiple elements of varying type and size.
/// Used when a user wants to change a collection order.
///
/// ## Example
///
/// ```rust
///use dioxus::prelude::*;
///use dioxus_primitives::drag_and_drop_list::{DragAndDropList, DragAndDropListItem};
///#[component]
///pub fn Demo() -> Element {
///    let items = ["Item1", "Item2", "Item3"]
///        .map(|t| {
///            rsx! { {t} }
///        })
///        .to_vec();
///    rsx! {
///        DragAndDropList { items }
///    }
///}
/// ```
#[component]
pub fn DragAndDropList(props: DragAndDropListProps) -> Element {
    let drag_from = use_signal(|| None);
    let drop_to = use_signal(|| None);
    let is_dragging = use_signal(|| false);
    let list_items = use_signal(|| props.items.clone());
    let temp_list = use_signal(|| props.items.clone());

    use_context_provider(|| DragAndDropContext {
        drag_from,
        drop_to,
        is_dragging,
        original_list: list_items,
        temp_list,
    });

    let display_list = move |elements: Vec<Element>| {
        elements
            .iter()
            .enumerate()
            .map(|(index, children)| {
                rsx! {
                    DragAndDropListItem {
                        index,
                        is_removable: props.is_removable,
                        {children}
                    }
                }
            })
            .collect::<Vec<Element>>()
    };

    rsx! {
        div {
            class: "dnd-list",
            ..props.attributes,
            ul {
                class: "dnd-list-ul",
                {
                    display_list(if is_dragging() {
                        temp_list()
                    } else {
                        list_items()
                    }).iter()
                }
            }
            {props.children}
        }
    }
}

/// The props for the [`DragAndDropListItemProps`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DragAndDropListItemProps {
    /// The index of the index trigger
    pub index: usize,

    /// Set if the list item should be removable
    pub is_removable: bool,

    /// Additional attributes to apply to the list item element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the list item component.
    pub children: Element,
}

/// # DragAndDropListItem
///
/// This component represents an individual draggable item in the dnd list.
/// This must be used inside a [`DragAndDropList`] component.
///
/// ## Example
///
/// ```rust
///use dioxus::prelude::*;
///use dioxus_primitives::drag_and_drop_list::{DragAndDropList, DragAndDropListItem};
///#[component]
///pub fn Demo() -> Element {
///    let items = ["Item1", "Item2", "Item3"]
///        .map(|t| {
///            rsx! { {t} }
///        })
///        .to_vec();
///    rsx! {
///        DragAndDropList { items }
///    }
///}
/// ```
#[component]
pub fn DragAndDropListItem(props: DragAndDropListItemProps) -> Element {
    let mut ctx: DragAndDropContext = use_context();

    let index = props.index;

    let render_body = move |to: Option<usize>| match to {
        None => true,
        Some(v) => v != index,
    };

    rsx! {
        li {
            class: "dnd-list-item",
            draggable: "true",
            "is-grabbing": if (ctx.drag_from)().is_some_and(|from| from == index) { "true" },
            ondragstart: move |event: Event<DragData>| {
                ctx.start_drag(index);
                // Note: this is only for Firefox (without it, DnD won't work)
                let _ = event.data_transfer().set_data("text/html", "");
            },
            ondragend: move |_| ctx.end_drag(),
            ondragover: move |event: Event<DragData>| {
                // default is to cancel out the drop
                event.prevent_default();
                ctx.drag_over(index);
            },
            ondrop: move |_| ctx.drop(),
            //ondragleave: move |_| ctx.drop_to.set(None),
            ..props.attributes,
            if render_body((ctx.drop_to)()) {
                div { class: "item-icon-div", DragIcon {} }
                div { class: "item-body-div", {props.children} }
                if props.is_removable {
                    RemoveButton { on_click: move || ctx.remove(index) }
                }
            } else {
                div { class: "space-item" }
            }
        }
    }
}

#[component]
fn RemoveButton(on_click: Callback<()>) -> Element {
    rsx! {
        button {
            class: "remove-button",
            onclick: move |_| on_click.call(()),
            BaseIcon {
                path { d: "M18 6 6 18" }
                path { d: "m6 6 12 12" }
            }
        }
    }
}

#[component]
fn DragIcon() -> Element {
    rsx! {
        BaseIcon {
            line { x1: "5", x2: "19", y1: "9", y2: "9", }
            line { x1: "5", x2: "19", y1: "15", y2: "15", }
        }
    }
}

#[component]
fn BaseIcon(children: Element) -> Element {
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
            {children}
        }
    }
}
