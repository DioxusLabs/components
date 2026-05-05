//! ComboboxInput component — the search input inside the popup.

use std::rc::Rc;

use dioxus::prelude::*;

use super::super::context::{ComboboxContentContext, ComboboxContext};
use crate::{use_effect, use_id_or, use_unique_id};

/// Props for [`ComboboxInput`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxInputProps {
    /// Placeholder text for the search input.
    #[props(default = ReadSignal::new(Signal::new(String::from("Search..."))))]
    pub placeholder: ReadSignal<String>,

    /// Optional id for the input element.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// # ComboboxInput
///
/// The search input rendered inside [`ComboboxContent`]. Typing here updates
/// the filter query, and arrow keys navigate the visible options without
/// moving DOM focus away from the input.
#[component]
pub fn ComboboxInput(props: ComboboxInputProps) -> Element {
    let mut ctx = use_context::<ComboboxContext>();
    let render = use_context::<ComboboxContentContext>().render;

    let id = use_unique_id();
    let id = use_id_or(id, props.id);

    use_effect(move || {
        ctx.input_id.set(Some(id()));
    });

    // Auto-focus the input every time the popup opens.
    let mut input_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let mut open = ctx.open;
    use_effect(move || {
        if render() {
            if let Some(node) = input_ref() {
                spawn(async move {
                    let _ = node.set_focus(true).await;
                });
            }
        }
    });

    let mut query = ctx.query;

    // Compute aria-activedescendant from the focused option.
    let active_descendant = use_memo(move || {
        let idx = ctx.focus_state.current_focus()?;
        let options = ctx.options.read();
        options
            .iter()
            .find(|opt| opt.tab_index == idx)
            .map(|opt| opt.id.clone())
    });

    let onkeydown = move |event: KeyboardEvent| match event.key() {
        Key::ArrowDown => {
            if !open() {
                open.set(true);
            }
            ctx.focus_next_visible();
            event.prevent_default();
            event.stop_propagation();
        }
        Key::ArrowUp => {
            if !open() {
                open.set(true);
            }
            ctx.focus_prev_visible();
            event.prevent_default();
            event.stop_propagation();
        }
        Key::Home => {
            ctx.focus_first_visible();
            event.prevent_default();
            event.stop_propagation();
        }
        Key::End => {
            ctx.focus_last_visible();
            event.prevent_default();
            event.stop_propagation();
        }
        Key::Enter => {
            ctx.select_focused();
            event.prevent_default();
            event.stop_propagation();
        }
        Key::Escape => {
            open.set(false);
            event.prevent_default();
            event.stop_propagation();
        }
        _ => {}
    };

    if !render() {
        return rsx! {};
    }

    rsx! {
        input {
            id,
            r#type: "text",
            value: query(),
            placeholder: props.placeholder,
            autocomplete: "off",
            spellcheck: "false",

            role: "combobox",
            aria_autocomplete: "list",
            aria_expanded: open(),
            aria_controls: ctx.list_id,
            aria_activedescendant: active_descendant(),

            onmounted: move |evt| input_ref.set(Some(evt.data())),
            oninput: move |event| {
                query.set(event.value());
                // Reset focus to the first visible option whenever the query changes.
                ctx.focus_first_visible();
            },
            onkeydown,
            onblur: move |_| {
                // The popup's onpointerdown prevents focus loss for in-popup clicks,
                // so a real blur means focus moved outside — close the popup.
                if open() {
                    open.set(false);
                }
            },

            ..props.attributes,
        }
    }
}
