//! Defines the [`DragAndDropList`] component and its sub-components.
use crate::icon::Icon;
use dioxus::prelude::*;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Debug)]
enum DropPosition {
    Before,
    After,
    Undefined,
}

#[derive(Clone, Copy)]
struct DragAndDropContext {
    drag_from: Signal<Option<usize>>,
    drop_to: Signal<Option<usize>>,
    drop_position: Signal<DropPosition>,
    is_dragging: Signal<bool>,
    list_items: Signal<Vec<Element>>,
    focused_index: Signal<Option<usize>>,
}

impl DragAndDropContext {
    fn start_drag(&mut self, index: usize) {
        self.drag_from.set(Some(index));
        self.drop_to.set(None);
        self.drop_position.set(DropPosition::After);
        self.is_dragging.set(true);
    }

    fn end_drag(&mut self) {
        self.set_focus((self.drop_to)());
        self.drag_from.set(None);
        self.drop_to.set(None);
        self.drop_position.set(DropPosition::Undefined);
        self.is_dragging.set(false);
    }

    fn drag_over(&mut self, index: usize) {
        let Some(to) = (self.drop_to)() else {
            self.drop_to.set(Some(index));
            return;
        };

        if to == index {
            return;
        }

        self.drop_position.set(if to < index {
            DropPosition::After
        } else {
            DropPosition::Before
        });

        self.drop_to.set(Some(index));
    }

    fn drop(&mut self) {
        let Some(index) = (self.drop_to)() else {
            return;
        };

        let mut list = (self.list_items)();
        let from = (self.drag_from)().unwrap();
        let element = list.remove(from);
        list.insert(index, element);
        self.list_items.set(list);
    }

    fn remove(&mut self, index: usize) {
        let mut list = (self.list_items)();
        if list.remove(index).is_ok() {
            self.list_items.set(list);
        }
    }

    fn is_focused(&self, index: usize) -> bool {
        (self.focused_index)().is_some_and(|focus| focus == index)
    }

    fn set_focus(&mut self, id: Option<usize>) {
        self.focused_index.set(id);
    }

    fn focus_next(&mut self) {
        let Some(index) = (self.focused_index)() else {
            return;
        };

        let mut next_focused = index.saturating_add(1);

        let count = (self.list_items)().len() - 1;
        if index == count {
            next_focused = 0;
        }

        self.focused_index.set(Some(next_focused));
    }

    fn focus_prev(&mut self) {
        let Some(index) = (self.focused_index)() else {
            return;
        };

        let mut next_focused = index.saturating_sub(1);

        let count = (self.list_items)().len() - 1;
        if index == 0 {
            next_focused = count;
        }

        self.focused_index.set(Some(next_focused));
    }

    fn move_up(&mut self, from: usize) {
        let mut index = (self.drop_to)().unwrap_or(from);

        if (self.drop_position)() == DropPosition::After {
            self.drop_position.set(DropPosition::Before);
        } else if (self.drop_to)().is_some_and(|to| to == 0) {
            index = (self.list_items)().len() - 1;
        } else {
            index -= 1;
        }

        self.drag_over(index);
    }

    fn move_down(&mut self, from: usize) {
        let mut index = (self.drop_to)().unwrap_or(from);

        if (self.drop_position)() == DropPosition::Before {
            self.drop_position.set(DropPosition::After);
        } else {
            let count = (self.list_items)().len();

            if (self.drop_to)().is_some_and(|to| to == count - 1) {
                index = 0;
            } else {
                index += 1;
            }
        }

        self.drag_over(index);
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
    let drop_position = use_signal(|| DropPosition::Undefined);
    let is_dragging = use_signal(|| false);
    let list_items = use_signal(|| props.items.clone());

    use_context_provider(|| DragAndDropContext {
        drag_from,
        drop_to,
        drop_position,
        is_dragging,
        list_items,
        focused_index: Signal::new(None),
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
                { display_list(list_items()).iter() }
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

    let mut item_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    use_effect(move || {
        if ctx.is_focused(index) {
            if let Some(md) = item_ref() {
                spawn(async move {
                    let _ = md.set_focus(true).await;
                });
            }
        }
    });

    let render_drop_indicator = move |to: Option<usize>| match to {
        None => false,
        Some(v) => v == index,
    };

    let onkeydown = move |event: Event<KeyboardData>| {
        let key = event.key();

        match key {
            Key::ArrowUp => {
                event.prevent_default();
                if (ctx.is_dragging)() {
                    ctx.move_up(index);
                } else {
                    ctx.focus_prev();
                }
            }
            Key::ArrowDown => {
                event.prevent_default();
                if (ctx.is_dragging)() {
                    ctx.move_down(index);
                } else {
                    ctx.focus_next();
                }
            }
            Key::Enter => {
                event.prevent_default();
                if (ctx.is_dragging)() {
                    ctx.drop();
                    ctx.end_drag();
                } else {
                    ctx.start_drag(index);
                    ctx.drag_over(index);
                }
            }
            Key::Escape => {
                event.prevent_default();
                if (ctx.is_dragging)() {
                    ctx.end_drag();
                }
                ctx.set_focus(None);
            }
            _ => {}
        };
    };

    rsx! {
        if (ctx.drop_position)() == DropPosition::Before && render_drop_indicator((ctx.drop_to)()) {
            DropIndicator {  }
        }
        li {
            class: "dnd-list-item",
            draggable: "true",
            tabindex: if ctx.is_focused(index) { "0" } else { "-1" },
            "is-grabbing": if (ctx.drag_from)().is_some_and(|from| from == index) { "true" },
            "data-focus-visible": if ctx.is_focused(index) && !(ctx.is_dragging)() { "true" },
            onmounted: move |data| item_ref.set(Some(data.data())),
            onfocus: move |_| ctx.set_focus(Some(index)),
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
            onkeydown,
            ..props.attributes,
            div { class: "item-icon-div", DragIcon {} }
            div { class: "item-body-div", {props.children} }
            if props.is_removable {
                 RemoveButton { on_click: move || ctx.remove(index) }
            }
        }
        if (ctx.drop_position)() == DropPosition::After && render_drop_indicator((ctx.drop_to)()) {
            DropIndicator {  }
        }
    }
}

#[component]
fn DropIndicator() -> Element {
    rsx! {
        div {
            class: "drop-indicator",
        }
    }
}

#[component]
fn RemoveButton(on_click: Callback<()>) -> Element {
    rsx! {
        button {
            class: "remove-button",
            onclick: move |_| on_click.call(()),
            Icon {
                // X icon from lucide https://lucide.dev/icons/x
                path { d: "M18 6 6 18" }
                path { d: "m6 6 12 12" }
            }
        }
    }
}

#[component]
fn DragIcon() -> Element {
    rsx! {
        Icon {
            // equal icon from lucide https://lucide.dev/icons/equal
            line { x1: "5", x2: "19", y1: "9", y2: "9", }
            line { x1: "5", x2: "19", y1: "15", y2: "15", }
        }
    }
}
