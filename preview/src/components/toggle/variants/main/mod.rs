use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            display: "flex",
            align_items: "center",
            gap: "0.5rem",
            Toggle { width: "2.5rem", height: "2.5rem",
                b { "B" }
            }
            Toggle { width: "2.5rem", height: "2.5rem",
                i { "I" }
            }
            Toggle { width: "2.5rem", height: "2.5rem",
                u { "U" }
            }
            Toggle { width: "2.5rem", height: "2.5rem",
                s { "S" }
            }
        }
    }
}
