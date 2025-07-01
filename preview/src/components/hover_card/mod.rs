use dioxus::prelude::*;
use dioxus_primitives::{
    hover_card::{HoverCard, HoverCardContent, HoverCardTrigger},
    ContentSide,
};

#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/hover_card/style.css"),
        }
        div {
            style: "padding: 50px; display: flex; flex-direction: row; flex-wrap: wrap; gap: 40px; justify-content: center; align-items: center;",
            HoverCard { class: "hover-card",
                HoverCardTrigger { class: "hover-card-trigger",
                    i { "Dioxus" }
                }
                HoverCardContent { class: "hover-card-content", side: ContentSide::Bottom,
                    div {
                        padding: "1rem",
                        "Dioxus is"
                        i { " the " }
                        "Rust framework for building fullstack web, desktop, and mobile apps. Iterate with live hotreloading, add server functions, and deploy in record time."
                    }
                }
            }
        }
    }
}
