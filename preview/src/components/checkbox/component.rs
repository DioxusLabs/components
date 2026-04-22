use dioxus::prelude::*;
use dioxus_primitives::checkbox::{self, CheckboxProps};
use dioxus_primitives::icon;

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        checkbox::Checkbox {
            class: "dx-checkbox",
            checked: props.checked,
            default_checked: props.default_checked,
            required: props.required,
            disabled: props.disabled,
            name: props.name,
            value: props.value,
            on_checked_change: props.on_checked_change,
            attributes: props.attributes,
            checkbox::CheckboxIndicator { class: "dx-checkbox-indicator",
                icon::Icon {
                    class: "dx-checkbox-check-icon",
                    width: "1rem",
                    height: "1rem",
                    path { d: "M5 13l4 4L19 7" }
                }
            }
        }
    }
}
