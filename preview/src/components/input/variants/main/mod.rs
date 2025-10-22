use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut name = use_signal(String::new);
    rsx! {
        Input {
            oninput: move |e: FormEvent| name.set(e.value()),
            placeholder: "Enter your name",
            value: name,
        }
        if !name.read().is_empty() {
            p { "Hello, {name}!" }
        }
    }
}
