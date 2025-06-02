use dioxus::prelude::*;
use dioxus_primitives::checkbox::{CheckboxIndicator, CheckboxRoot, CheckboxState};

#[component]
fn CheckIcon() -> Element {
    rsx! {
        svg {
            width: "14",
            height: "14",
            view_box: "0 0 18 18",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2.5",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M4 9l4 4 6-6" }
        }
    }
}

#[component]
pub(super) fn Demo() -> Element {
    let mut checked = use_signal(|| false);

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/checkbox/style.css"),
        }
        form {
            div { style: "display: flex; align-items: center;",
                CheckboxRoot {
                    id: "c1",
                    default_checked: if checked() { CheckboxState::Checked } else { CheckboxState::Unchecked },
                    on_checked_change: move |state| checked.set(state == CheckboxState::Checked),
                    class: "CheckboxRoot",
                    CheckboxIndicator { class: "CheckboxIndicator", CheckIcon {} }
                }
                label { class: "Label", r#for: "c1", "Accept terms and conditions." }
            }
        }
    }
}
