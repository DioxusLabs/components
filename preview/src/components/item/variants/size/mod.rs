use super::super::component::*;
use crate::components::button::{Button, ButtonVariant};
use dioxus::prelude::*;
use dioxus_primitives::icon;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            gap: "1.5rem",
            width: "100%",
            max_width: "28rem",

            Item { variant: ItemVariant::Outline,
                ItemContent {
                    ItemTitle { "Basic Item" }
                    ItemDescription { "A simple item with title and description." }
                }
                ItemActions {
                    Button { variant: ButtonVariant::Outline, "Action" }
                }
            }

            Item {
                variant: ItemVariant::Outline,
                size: ItemSize::Sm,
                as: move |attrs: Vec<Attribute>| rsx! {
                    a { href: "#", ..attrs,
                        ItemMedia {
                            BadgeCheckIcon {}
                        }
                        ItemContent {
                            ItemTitle { "Your profile has been verified." }
                        }
                        ItemActions {
                            ChevronRightIcon {}
                        }
                    }
                },
            }
        }
    }
}

#[component]
fn BadgeCheckIcon() -> Element {
    rsx! {
        icon::Icon {
            width: "20",
            height: "20",
            path { d: "M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" }
            path { d: "m9 12 2 2 4-4" }
        }
    }
}

#[component]
fn ChevronRightIcon() -> Element {
    rsx! {
        icon::Icon {
            width: "16",
            height: "16",
            path { d: "m9 18 6-6-6-6" }
        }
    }
}
