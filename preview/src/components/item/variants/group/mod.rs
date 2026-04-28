use super::super::component::*;
use crate::components::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarImageSize};
use crate::components::button::{Button, ButtonVariant};
use dioxus::prelude::*;
use dioxus_primitives::icon;

const PEOPLE: &[(&str, &str, &str)] = &[
    (
        "jkelleyrtp",
        "jkelleyrtp@dioxuslabs.com",
        "https://github.com/jkelleyrtp.png",
    ),
    (
        "ealmloff",
        "ealmloff@dioxuslabs.com",
        "https://github.com/ealmloff.png",
    ),
    (
        "DioxusLabs",
        "team@dioxuslabs.com",
        "https://github.com/DioxusLabs.png",
    ),
];

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            width: "100%",
            max_width: "28rem",

            ItemGroup {
                for (i , (username , email , avatar)) in PEOPLE.iter().enumerate() {
                    Item {
                        ItemMedia {
                            Avatar { size: AvatarImageSize::Small,
                                AvatarImage { src: "{avatar}", alt: "{username}" }
                                AvatarFallback { "{&username[..1].to_uppercase()}" }
                            }
                        }
                        ItemContent {
                            ItemTitle { "{username}" }
                            ItemDescription { "{email}" }
                        }
                        ItemActions {
                            Button {
                                variant: ButtonVariant::Ghost,
                                aria_label: "Add {username}",
                                PlusIcon {}
                            }
                        }
                    }
                    if i + 1 < PEOPLE.len() {
                        ItemSeparator {}
                    }
                }
            }
        }
    }
}

#[component]
fn PlusIcon() -> Element {
    rsx! {
        icon::Icon {
            width: "16",
            height: "16",
            path { d: "M12 5v14" }
            path { d: "M5 12h14" }
        }
    }
}
