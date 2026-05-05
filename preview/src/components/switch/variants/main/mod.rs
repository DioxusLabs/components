use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            gap: "0.75rem",
            min_width: "16rem",
            SettingRow { label: "Enable notifications", initial: true }
            SettingRow { label: "Marketing emails", initial: false }
            SettingRow { label: "Auto-update", initial: true }
        }
    }
}

#[component]
fn SettingRow(label: String, initial: bool) -> Element {
    let mut checked = use_signal(|| initial);
    rsx! {
        label {
            display: "flex",
            align_items: "center",
            justify_content: "space-between",
            gap: "1rem",
            cursor: "pointer",
            span { "{label}" }
            div { class: "dx-switch-example",
                Switch {
                    checked: checked(),
                    aria_label: label,
                    on_checked_change: move |v| checked.set(v),
                    SwitchThumb {}
                }
            }
        }
    }
}
