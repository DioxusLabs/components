//! ComboboxValue component.

use dioxus::prelude::*;

use super::super::context::ComboboxContext;

/// Props for [`ComboboxValue`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxValueProps {
    /// Additional attributes for the value element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// # ComboboxValue
///
/// Displays the currently selected option's text value, or the placeholder if
/// nothing is selected. Must be used inside a
/// [`ComboboxTrigger`](super::trigger::ComboboxTrigger).
#[component]
pub fn ComboboxValue(props: ComboboxValueProps) -> Element {
    let ctx = use_context::<ComboboxContext>();

    let selected_text_value = use_memo(move || {
        let value = ctx.value.read();
        value.as_ref().and_then(|v| {
            ctx.options
                .read()
                .iter()
                .find(|opt| opt.value == *v)
                .map(|opt| opt.text_value.clone())
        })
    });

    let display_value = selected_text_value().unwrap_or_else(|| ctx.placeholder.cloned());

    rsx! {
        span {
            "data-placeholder": ctx.value.read().is_none(),
            ..props.attributes,
            {display_value}
        }
    }
}
