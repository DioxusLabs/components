use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let items = example_items();
    rsx! {
        DragAndDropList {
            items,
            is_removable: true
        }
    }
}
