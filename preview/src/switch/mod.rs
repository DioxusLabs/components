use dioxus::{prelude::*};
use dioxus_primitives::switch::{Switch, SwitchThumb};


#[component]
pub(super) fn SwitchExample() -> Element {
    let mut checked = use_signal(|| false);

    rsx! {
        div { class: "switch-example",
            label { "Airplane Mode" }
            Switch {
                class: "switch",
                checked,
                on_checked_change: move |new_checked| {
                    checked.set(new_checked);
                    tracing::info!("Switch toggled: {new_checked}");
                },

                SwitchThumb { class: "switch-thumb" }
            }
        }
    }
}
