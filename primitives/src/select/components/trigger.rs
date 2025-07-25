//! SelectTrigger component implementation.

use dioxus::prelude::*;

use super::super::context::SelectContext;

/// The props for the [`SelectTrigger`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectTriggerProps {
    /// Additional attributes for the trigger button
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children to render inside the trigger
    children: Element,
}

/// # SelectTrigger
///
/// The trigger button for the [`Select`] component which controls if the [`SelectList`] is rendered.
///
/// This must be used inside a [`Select`] component.
#[component]
pub fn SelectTrigger<T: Clone + PartialEq + 'static>(props: SelectTriggerProps) -> Element {
    let mut ctx = use_context::<SelectContext<T>>();
    let mut open = ctx.open;

    rsx! {
        button {
            // Standard HTML attributes
            disabled: (ctx.disabled)(),

            onclick: move |_| {
                open.toggle();
            },
            onkeydown: move |event| {
                match event.key() {
                    Key::ArrowUp => {
                        open.set(true);
                        ctx.focus_state.focus_last();
                        event.prevent_default();
                        event.stop_propagation();
                    }
                    Key::ArrowDown => {
                        open.set(true);
                        ctx.focus_state.focus_first();
                        event.prevent_default();
                        event.stop_propagation();
                    }
                    _ => {}
                }
            },

            // ARIA attributes
            aria_haspopup: "listbox",
            aria_expanded: open(),
            aria_controls: ctx.list_id,

            // Pass through other attributes
            ..props.attributes,

            // Add placeholder option if needed
            span {
                "data-placeholder": ctx.cursor.read().display == ctx.placeholder.cloned(),
                {ctx.cursor.read().display.clone()}
            }

            // Render children (options)
            {props.children}
        }
    }
}
