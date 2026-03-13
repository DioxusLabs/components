use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let items = vec![
        rsx! {"Cat 🐱"},
        rsx! {"Cow 🐮"},
        rsx! {"Dog 🐶"},
        rsx! {"Fox 🦊"},
        rsx! {"Pig 🐷"},
    ];

    rsx! {
        DragAndDropList { items }
    }
}
