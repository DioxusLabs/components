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

/// Resolves the final insertion index from a hovered item and pointer position.
fn resolve_drop_index(from: usize, hovered: usize, position: DropPosition) -> usize {
    let slot = match position {
        DropPosition::Before | DropPosition::Undefined => hovered,
        DropPosition::After => hovered + 1,
    };

    if from < slot {
        slot - 1
    } else {
        slot
    }
}

/// Resolves whether the final insertion index is before or after the source item.
fn resolve_drop_position(from: usize, to: usize) -> DropPosition {
    if to < from {
        DropPosition::Before
    } else if to > from {
        DropPosition::After
    } else {
        DropPosition::Undefined
    }
}

#[derive(Clone, Copy)]
struct DragAndDropContext {
    drag_from: Signal<Option<usize>>,
    drop_to: Signal<Option<usize>>,
    drop_position: Signal<DropPosition>,
    is_dragging: Signal<bool>,
    list_items: Signal<Vec<Element>>,
    focused_index: Signal<Option<usize>>,
    announcement: Signal<String>,
}

impl DragAndDropContext {
    fn start_drag(&mut self, index: usize) {
        self.drag_from.set(Some(index));
        self.drop_to.set(None);
        self.drop_position.set(DropPosition::Undefined);
        self.is_dragging.set(true);
    }

    fn end_drag(&mut self) {
        let focus_target = (self.drop_to)().or((self.drag_from)());
        self.set_focus(focus_target);
        self.drag_from.set(None);
        self.drop_to.set(None);
        self.drop_position.set(DropPosition::Undefined);
        self.is_dragging.set(false);
    }

    fn cancel_drag(&mut self) {
        self.set_focus((self.drag_from)());
        self.drag_from.set(None);
        self.drop_to.set(None);
        self.drop_position.set(DropPosition::Undefined);
        self.is_dragging.set(false);
    }

    fn drag_over(&mut self, hovered: usize, position: DropPosition) {
        let from = (self.drag_from)().unwrap_or(hovered);
        let resolved = resolve_drop_index(from, hovered, position);

        self.drop_to.set(Some(resolved));
        self.drop_position
            .set(resolve_drop_position(from, resolved));
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
            let new_len = list.len();
            self.list_items.set(list);
            self.focused_index
                .set(new_len.checked_sub(1).map(|last| index.min(last)));
            self.announcement.set(format!(
                "Removed item from position {}. {} items remaining",
                index + 1,
                new_len
            ));
        }
    }

    fn announce(&mut self, msg: String) {
        self.announcement.set(msg);
    }

    fn item_count(&self) -> usize {
        (self.list_items)().len()
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
        let len = (self.list_items)().len();
        self.focused_index.set(Some((index + 1) % len));
    }

    fn focus_prev(&mut self) {
        let Some(index) = (self.focused_index)() else {
            return;
        };
        let len = (self.list_items)().len();
        self.focused_index
            .set(Some(index.checked_sub(1).unwrap_or(len - 1)));
    }

    fn move_up(&mut self, from: usize) {
        let current = (self.drop_to)().unwrap_or(from);
        let len = (self.list_items)().len();
        let new_index = current.checked_sub(1).unwrap_or(len - 1);
        self.drop_to.set(Some(new_index));
        self.update_keyboard_drop_position(from);
    }

    fn move_down(&mut self, from: usize) {
        let current = (self.drop_to)().unwrap_or(from);
        let len = (self.list_items)().len();
        let new_index = (current + 1) % len;
        self.drop_to.set(Some(new_index));
        self.update_keyboard_drop_position(from);
    }

    fn update_keyboard_drop_position(&mut self, from: usize) {
        let drag_from = (self.drag_from)().unwrap_or(from);
        let drop_to = (self.drop_to)().unwrap_or(from);
        self.drop_position
            .set(resolve_drop_position(drag_from, drop_to));
    }

    fn announce_move(&mut self, index: usize) {
        let pos = (self.drop_to)().unwrap_or(index) + 1;
        let count = self.item_count();
        self.announce(format!(
            "You have moved the item to position {pos} of {count}"
        ));
    }

    fn toggle_drag(&mut self, index: usize) {
        if (self.is_dragging)() {
            let from = (self.drag_from)().unwrap_or(index) + 1;
            let to = (self.drop_to)().unwrap_or(index) + 1;
            self.drop();
            self.end_drag();
            self.announce(format!(
                "You have dropped the item. It has moved from position {from} to position {to}"
            ));
        } else {
            let count = self.item_count();
            self.start_drag(index);
            self.drag_over(index, DropPosition::Undefined);
            self.announce(format!(
                "You have lifted an item in position {} of {count}",
                index + 1
            ));
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

    /// Accessible label for the list
    #[props(default)]
    pub aria_label: Option<String>,

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
    let announcement = use_signal(String::new);

    use_context_provider(|| DragAndDropContext {
        drag_from,
        drop_to,
        drop_position,
        is_dragging,
        list_items,
        focused_index: Signal::new(None),
        announcement,
    });

    let label = props
        .aria_label
        .as_deref()
        .unwrap_or("Sortable list")
        .to_string();

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
            div {
                id: "dnd-instructions",
                style: "position:absolute;width:1px;height:1px;overflow:hidden;clip:rect(0,0,0,0);",
                "Press Enter to start reordering. Use Arrow keys to change position. Press Enter to confirm or Escape to cancel."
            }
            ul {
                class: "dnd-list-ul",
                aria_label: "{label}",
                aria_roledescription: "sortable list",
                aria_describedby: "dnd-instructions",
                { display_list(list_items()).iter() }
            }
            div {
                role: "status",
                aria_live: "assertive",
                aria_atomic: "true",
                style: "position:absolute;width:1px;height:1px;overflow:hidden;clip:rect(0,0,0,0);",
                "{announcement}"
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
                    ctx.announce_move(index);
                } else {
                    ctx.focus_prev();
                }
            }
            Key::ArrowDown => {
                event.prevent_default();
                if (ctx.is_dragging)() {
                    ctx.move_down(index);
                    ctx.announce_move(index);
                } else {
                    ctx.focus_next();
                }
            }
            Key::Enter => {
                event.prevent_default();
                ctx.toggle_drag(index);
            }
            Key::Character(ref c) if c == " " => {
                event.prevent_default();
                ctx.toggle_drag(index);
            }
            Key::Escape => {
                event.prevent_default();
                if (ctx.is_dragging)() {
                    let pos = (ctx.drag_from)().unwrap_or(index) + 1;
                    ctx.cancel_drag();
                    ctx.announce(format!(
                        "Movement cancelled. The item has returned to its starting position of {pos}"
                    ));
                }
            }
            Key::Delete | Key::Backspace => {
                event.prevent_default();
                if !(ctx.is_dragging)() && props.is_removable {
                    ctx.remove(index);
                }
            }
            Key::Home => {
                event.prevent_default();
                if !(ctx.is_dragging)() {
                    ctx.set_focus(Some(0));
                }
            }
            Key::End => {
                event.prevent_default();
                if !(ctx.is_dragging)() {
                    ctx.set_focus(ctx.item_count().checked_sub(1));
                }
            }
            _ => {}
        };
    };

    let is_tab_reachable = ctx.is_focused(index) || ((ctx.focused_index)().is_none() && index == 0);

    rsx! {
        if (ctx.drop_position)() == DropPosition::Before && render_drop_indicator((ctx.drop_to)()) {
            DropIndicator {  }
        }
        li {
            class: "dnd-list-item",
            aria_roledescription: "sortable item",
            draggable: "true",
            tabindex: if is_tab_reachable { "0" } else { "-1" },
            aria_grabbed: if (ctx.drag_from)().is_some_and(|from| from == index) { "true" } else { "false" },
            "data-is-grabbing": if (ctx.drag_from)().is_some_and(|from| from == index) { "true" },
            "data-focus-visible": if ctx.is_focused(index) { "true" },
            onmounted: move |data| item_ref.set(Some(data.data())),
            onfocus: move |_| {
                if !(ctx.is_dragging)() {
                    ctx.set_focus(Some(index));
                }
            },
            ondragstart: move |event: Event<DragData>| {
                ctx.start_drag(index);
                // Note: this is only for Firefox (without it, DnD won't work)
                let _ = event.data_transfer().set_data("text/html", "");
            },
            ondragend: move |_| ctx.end_drag(),
            ondragover: move |event: Event<DragData>| {
                event.prevent_default();
                async move {
                    if let Some(md) = item_ref() {
                        let cursor_y = event.client_coordinates().y;
                        if let Ok(rect) = md.get_client_rect().await {
                            let mid_y = rect.origin.y + rect.size.height / 2.0;
                            let position = if cursor_y < mid_y {
                                DropPosition::Before
                            } else {
                                DropPosition::After
                            };
                            ctx.drag_over(index, position);
                        }
                    }
                }
            },
            ondrop: move |_| ctx.drop(),
            //ondragleave: move |_| ctx.drop_to.set(None),
            onkeydown,
            ..props.attributes,
            div { class: "item-icon-div", aria_hidden: "true", DragIcon {} }
            div { class: "item-body-div", {props.children} }
            if props.is_removable {
                 RemoveButton { index, on_click: move || ctx.remove(index) }
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
fn RemoveButton(index: usize, on_click: Callback<()>) -> Element {
    let label = format!("Remove item {}", index + 1);
    rsx! {
        button {
            class: "remove-button",
            aria_label: "{label}",
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
