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

    fn drag_over(&mut self, index: usize) {
        let from = (self.drag_from)().unwrap();
        if from == index {
            return;
        }

        if (self.drop_to)().is_some_and(|to| to == index) {
            return;
        }

        self.drop_to.set(Some(index));

        let mut list = (self.original_list)();
        let element = list.remove(from);
        list.insert(index, element);
        self.temp_list.set(list);
    }

    fn drop(&mut self) {
        self.original_list.set((self.temp_list)());
        self.drag_from.set(None);
        self.drop_to.set(None);
        self.is_dragging.set(false);
    }

    fn remove(&mut self, index: usize) {
        let mut list = (self.original_list)();
        let _ = list.remove(index);
        self.original_list.set(list);
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
/// ## Example
///
/// ```rust
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

    /// Additional attributes to apply to the list element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tabs component.
    pub children: Element,
}

/// # DragAndDropListItem
///
/// ## Example
///
/// ```rust
/// ```
#[component]
pub fn DragAndDropListItem(props: DragAndDropListItemProps) -> Element {
    let mut ctx: DragAndDropContext = use_context();

    let index = props.index;

    rsx! {
        li {
            class: "dnd-list-item",
            draggable: "true",
            "is-grabbing": (ctx.drag_from)().is_some_and(|from| from == index),
            //visibility: if Some(index) == (ctx.drag_from)() { "hidden" },
            ondragstart: move |event: Event<DragData>| {
                ctx.start_drag(index);
                // Note: this is only for Firefox (without it, DnD won't work)
                let _ = event.data_transfer().set_data("text/html", "");
            },
            ondragover: move |event: Event<DragData>| {
                // default is to cancel out the drop
                event.prevent_default();
                ctx.drag_over(index);
            },
            ondrop: move |_| ctx.drop(),
            ondragleave: move |_| ctx.drop_to.set(None),
            ..props.attributes,
            div { class: "item-icon-div", DragIcon {} }
            div { class: "item-body-div", {props.children} }
            if props.is_removable {
                RemoveButton { on_click: move || ctx.remove(index) }
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
