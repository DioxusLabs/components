use dioxus::prelude::*;
use dioxus_primitives::switch::{Switch, SwitchThumb};
#[component]
pub(super) fn Demo() -> Element {
    let mut checked = use_signal(|| false);
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/switch/style.css"),
        }
        div { class: "switch-example",
            Switch {
                class: "switch",
                checked: checked(),
                aria_label: "Switch Demo",
                on_checked_change: move |new_checked| {
                    checked.set(new_checked);
                    tracing::info!("Switch toggled: {new_checked}");
                },
                SwitchThumb { class: "switch-thumb" }
            }
        }
    }
}
