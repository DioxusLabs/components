//! SelectList component implementation.

use crate::{
    select::context::SelectListContext, use_animated_open, use_effect, use_id_or, use_unique_id,
};
use dioxus::prelude::*;

use super::super::context::SelectContext;

/// The props for the [`SelectList`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectListProps {
    /// The ID of the list for ARIA attributes
    #[props(default)]
    pub id: ReadOnlySignal<Option<String>>,

    /// Additional attributes for the list
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children to render inside the list
    children: Element,
}

/// # SelectList
///
/// The dropdown list container for the [`Select`](super::select::Select) component that contains the
/// [`SelectOption`](super::option::SelectOption)s. The list will only be rendered when the select is open.
///
/// This must be used inside a [`Select`](super::select::Select) component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::select::{
///     Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
///     SelectTrigger, SelectValue,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Select::<String> {
///             placeholder: "Select a fruit...",
///             SelectTrigger {
///                 aria_label: "Select Trigger",
///                 width: "12rem",
///                 SelectValue {}
///             }
///             SelectList {
///                 aria_label: "Select Demo",
///                 SelectGroup {
///                     SelectGroupLabel { "Fruits" }
///                     SelectOption::<String> {
///                         index: 0usize,
///                         value: "apple",
///                         "Apple"
///                         SelectItemIndicator { "✔️" }
///                     }
///                     SelectOption::<String> {
///                         index: 1usize,
///                         value: "banana",
///                         "Banana"
///                         SelectItemIndicator { "✔️" }
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn SelectList(props: SelectListProps) -> Element {
    let mut ctx = use_context::<SelectContext>();

    let id = use_unique_id();
    let id = use_id_or(id, props.id);
    use_effect(move || {
        ctx.list_id.set(Some(id()));
    });

    let mut open = ctx.open;
    let mut listbox_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    let focused = move || open() && !ctx.focus_state.any_focused();

    use_effect(move || {
        let Some(listbox_ref) = listbox_ref() else {
            return;
        };
        if focused() {
            spawn(async move {
                _ = listbox_ref.set_focus(true);
            });
        }
    });

    let onkeydown = move |event: KeyboardEvent| {
        let key = event.key();
        let code = event.code();

        // Learn from keyboard events for adaptive matching
        if let Key::Character(actual_char) = &key {
            if let Some(actual_char) = actual_char.chars().next() {
                ctx.learn_from_keyboard_event(&code.to_string(), actual_char);
            }
        }

        let mut arrow_key_navigation = |event: KeyboardEvent| {
            // Clear the typeahead buffer
            ctx.typeahead_buffer.take();
            event.prevent_default();
            event.stop_propagation();
        };

        match key {
            Key::Character(new_text) => {
                if new_text == " " {
                    ctx.select_current_item();
                    event.prevent_default();
                    event.stop_propagation();
                    return;
                }

                ctx.add_to_typeahead_buffer(&new_text);
            }
            Key::ArrowUp => {
                arrow_key_navigation(event);
                ctx.focus_state.focus_prev();
            }
            Key::End => {
                arrow_key_navigation(event);
                ctx.focus_state.focus_last();
            }
            Key::ArrowDown => {
                arrow_key_navigation(event);
                ctx.focus_state.focus_next();
            }
            Key::Home => {
                arrow_key_navigation(event);
                ctx.focus_state.focus_first();
            }
            Key::Enter => {
                ctx.select_current_item();
                open.set(false);
                event.prevent_default();
                event.stop_propagation();
            }
            Key::Escape => {
                open.set(false);
                event.prevent_default();
                event.stop_propagation();
            }
            _ => {}
        }
    };

    let render = use_animated_open(id, open);
    let render = use_memo(render);

    use_context_provider(|| SelectListContext {
        render: render.into(),
    });

    rsx! {
        if render() {
            div {
                id,
                role: "listbox",
                tabindex: if focused() { "0" } else { "-1" },

                // Data attributes
                "data-state": if open() { "open" } else { "closed" },

                onmounted: move |evt| listbox_ref.set(Some(evt.data())),
                onkeydown,
                onblur: move |_| {
                    if focused() {
                        open.set(false);
                    }
                },

                ..props.attributes,
                {props.children}
            }
        } else {
            // If not rendering, return children directly so we can populate the selected list, but they should choose to not render themselves
            {props.children}
        }
    }
}
