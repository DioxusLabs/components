use super::super::component::*;
use crate::components::button::{Button, ButtonVariant};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Card { style: "width: 100%; max-width: 22rem;",
            CardHeader {
                CardTitle { "New release" }
                CardDescription { "Dioxus components v0.7 is now available." }
                CardAction {
                    Button { variant: ButtonVariant::Ghost, "Dismiss" }
                }
            }
            CardFooter { style: "gap: 0.5rem;",
                Button { style: "flex: 1;", "Read more" }
                Button { variant: ButtonVariant::Outline, style: "flex: 1;", "Later" }
            }
        }
    }
}
