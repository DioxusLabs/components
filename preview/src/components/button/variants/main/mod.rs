// demo.rs
use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/button/variants/main/style.css"),
        }

        div {
            display: "flex",
            flex_direction: "column",
            gap: "0.5rem",
            Button {
                "Primary"
            }

            Button {
                variant: ButtonVariant::Secondary,
                "Secondary"
            }

            Button {
                variant: ButtonVariant::Destructive,
                "Destructive"
            }

            Button {
                variant: ButtonVariant::Outline,
                "Outline"
            }

            Button {
                variant: ButtonVariant::Ghost,
                "Ghost"
            }
        }
    }
}
