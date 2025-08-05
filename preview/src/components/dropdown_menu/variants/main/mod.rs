use dioxus::prelude::*;
use dioxus_primitives::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use strum::IntoEnumIterator;

#[derive(Clone, Copy, strum::Display, strum::EnumIter, PartialEq)]
enum Operation {
    Edit,
    Undo,
    Duplicate,
    Delete,
}

#[component]
pub fn Demo() -> Element {
    let operations = Operation::iter().enumerate().map(|(i, o)| {
        rsx! {
            DropdownMenuItem::<Operation> {
                class: "dropdown-menu-item",
                value: o,
                index: i,
                disabled: matches!(o, Operation::Undo),
                on_select: move |value| {
                    tracing::info!("Selected: {value}");
                },
                {o.to_string()}
            }
        }
    });

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/dropdown_menu/variants/main/style.css"),
        }
        DropdownMenu { class: "dropdown-menu", default_open: false,
            DropdownMenuTrigger { class: "dropdown-menu-trigger", "Open Menu" }
            DropdownMenuContent { class: "dropdown-menu-content",
                {operations}
            }
        }
    }
}
