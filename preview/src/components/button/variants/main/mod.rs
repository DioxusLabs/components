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
            button {
                class: "button",
                "data-style": "primary",
                "Primary"
            }

            button {
                class: "button",
                "data-style": "secondary",
                "Secondary"
            }

            button {
                class: "button",
                "data-style": "destructive",
                "Destructive"
            }

            button {
                class: "button",
                "data-style": "outline",
                "Outline"
            }

            button {
                class: "button",
                "data-style": "ghost",
                "Ghost"
            }
        }
    }
}
