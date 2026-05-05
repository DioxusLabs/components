//! ComboboxTrigger component.

use dioxus::prelude::*;

use super::super::context::ComboboxContext;

/// Props for the [`ComboboxTrigger`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxTriggerProps {
    /// Additional attributes for the trigger button.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children rendered inside the trigger (typically [`ComboboxValue`]).
    pub children: Element,
}

/// # ComboboxTrigger
///
/// The button that opens / closes the combobox popup. Must be used inside a
/// [`Combobox`](super::combobox::Combobox).
#[component]
pub fn ComboboxTrigger(props: ComboboxTriggerProps) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let mut open = ctx.open;

    rsx! {
        button {
            type: "button",
            disabled: (ctx.disabled)(),

            role: "combobox",
            aria_haspopup: "listbox",
            aria_expanded: open(),
            aria_controls: ctx.list_id,

            "data-state": if open() { "open" } else { "closed" },

            onpointerdown: move |event| {
                // While open, the search input has focus. Clicking the trigger would
                // blur it and trip the input's "close on blur" handler before our
                // own click handler runs. Prevent default to keep the input focused
                // until the click toggles `open` to false on its own.
                if open() {
                    event.prevent_default();
                }
            },
            onclick: move |_| {
                open.toggle();
            },
            onkeydown: move |event| {
                match event.key() {
                    Key::ArrowDown | Key::ArrowUp => {
                        if !open() {
                            open.set(true);
                        }
                        event.prevent_default();
                        event.stop_propagation();
                    }
                    Key::Enter | Key::Character(_) if !open() => {
                        open.set(true);
                    }
                    _ => {}
                }
            },

            ..props.attributes,
            {props.children}
        }
    }
}
