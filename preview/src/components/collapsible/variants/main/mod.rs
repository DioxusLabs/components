use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/collapsible/variants/main/style.css"),
        }
        Collapsible {
            CollapsibleTrigger {
                b { "Recent Activity" }
            }
            CollapsibleList {
                CollapsibleItem {
                    "Added a new feature to the collapsible component",
                }
                CollapsibleContent {
                    CollapsibleItem {
                        "Fixed a bug in the collapsible component",
                    }
                    CollapsibleItem {
                        "Updated the documentation for the collapsible component",
                    }
                }
            }
        }
    }
}
