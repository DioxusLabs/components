use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
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
