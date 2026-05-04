use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut value = use_signal(String::new);
    rsx! {
        OneTimePasswordInput {
            maxlength: 6usize,
            value: value(),
            pattern: "[0-9]*",
            on_value_change: move |v| value.set(v),
            aria_label: "One-time password",
            OneTimePasswordGroup {
                OneTimePasswordSlot { index: 0usize }
                OneTimePasswordSlot { index: 1usize }
                OneTimePasswordSlot { index: 2usize }
            }
            OneTimePasswordSeparator {}
            OneTimePasswordGroup {
                OneTimePasswordSlot { index: 3usize }
                OneTimePasswordSlot { index: 4usize }
                OneTimePasswordSlot { index: 5usize }
            }
        }
    }
}
