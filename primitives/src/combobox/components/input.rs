//! ComboboxInput — the single `<input>` that doubles as the combobox trigger
//! and the search field.

use dioxus::prelude::*;

use super::super::context::ComboboxContext;
use crate::{use_id_or, use_unique_id};

/// Props for [`ComboboxInput`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxInputProps {
    /// Placeholder shown when the input is empty (no selected value and no
    /// query in flight).
    #[props(default)]
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
/// The text input that doubles as the combobox trigger. Following WAI-ARIA
/// 1.2's combobox pattern, this is the only element with `role="combobox"`:
/// it owns `aria-expanded`, `aria-controls`, and `aria-activedescendant`,
/// and the listbox lives in a separate popup that DOM focus never moves into.
///
/// - Closed: shows the selected option's text (or the placeholder).
/// - Open: shows the user's typed query.
/// - Click, typing, and arrow keys open the popup; Enter selects; Escape and
///   blur close.
#[component]
pub fn ComboboxInput(props: ComboboxInputProps) -> Element {
    let mut ctx = use_context::<ComboboxContext>();

    let id = use_unique_id();
    let id = use_id_or(id, props.id);

    let mut open = ctx.open;
    let mut query = ctx.query;

    let active_descendant = use_memo(move || {
        if !open() {
            return None;
        }
        let idx = ctx.focus_state.current_focus()?;
        let options = ctx.options.read();
        options
            .iter()
            .find(|opt| opt.tab_index == idx)
            .map(|opt| opt.id.clone())
    });

    let display_value = use_memo(move || {
        if open() {
            query.cloned()
        } else {
            ctx.selected_text().unwrap_or_default()
        }
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
        Key::Home if open() => {
            ctx.focus_first_visible();
            event.prevent_default();
            event.stop_propagation();
        }
        Key::End if open() => {
            ctx.focus_last_visible();
            event.prevent_default();
            event.stop_propagation();
        }
        Key::Enter if open() => {
            ctx.select_focused();
            event.prevent_default();
            event.stop_propagation();
        }
        Key::Escape if open() => {
            open.set(false);
            event.prevent_default();
            event.stop_propagation();
        }
        _ => {}
    };

    rsx! {
        input {
            id,
            r#type: "text",
            value: display_value(),
            placeholder: props.placeholder,
            autocomplete: "off",
            spellcheck: "false",
            disabled: (ctx.disabled)(),

            role: "combobox",
            aria_autocomplete: "list",
            aria_haspopup: "listbox",
            aria_expanded: open(),
            aria_controls: ctx.list_id,
            aria_activedescendant: active_descendant(),

            "data-state": if open() { "open" } else { "closed" },

            onclick: move |_| {
                if !open() {
                    open.set(true);
                }
            },
            oninput: move |event| {
                query.set(event.value());
                if !open() {
                    open.set(true);
                }
                ctx.focus_first_visible();
            },
            onkeydown,
            onblur: move |_| {
                // The popup's onpointerdown prevents focus loss for in-popup
                // clicks, so a real blur means focus moved outside.
                if open() {
                    open.set(false);
                }
            },

            ..props.attributes,
        }
    }
}
