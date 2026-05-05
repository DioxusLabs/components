use dioxus::prelude::*;

use super::super::component::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            class: "dx-badge-example",
            display: "flex",
            flex_wrap: "wrap",
            justify_content: "center",
            gap: "0.5rem",
            max_width: "16rem",
            Badge { "Primary" }
            Badge { variant: BadgeVariant::Secondary, "Secondary" }
            Badge { variant: BadgeVariant::Destructive, "Destructive" }
            Badge { variant: BadgeVariant::Outline, "Outline" }
        }
    }
}
