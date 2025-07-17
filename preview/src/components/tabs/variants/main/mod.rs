use dioxus::prelude::*;
use dioxus_primitives::tabs::{TabContent, TabTrigger, Tabs, TabList};
#[component]
pub fn Demo() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/components/tabs/variants/main/style.css") }
        Tabs {
            class: "tabs",
            default_value: "tab1".to_string(),
            horizontal: true,
            max_width: "16rem",
            TabList { class: "tabs-list",
                TabTrigger {
                    class: "tabs-trigger",
                    value: "tab1".to_string(),
                    index: 0usize,
                    "Tab 1"
                }
                TabTrigger {
                    class: "tabs-trigger",
                    value: "tab2".to_string(),
                    index: 1usize,
                    "Tab 2"
                }
                TabTrigger {
                    class: "tabs-trigger",
                    value: "tab3".to_string(),
                    index: 2usize,
                    "Tab 3"
                }
            }
            TabContent {
                index: 0usize,
                class: "tabs-content",
                value: "tab1".to_string(),
                div {
                    width: "100%",
                    height: "5rem",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    "Tab 1 Content"
                }
            }
            TabContent {
                index: 1usize,
                class: "tabs-content",
                value: "tab2".to_string(),
                div {
                    width: "100%",
                    height: "5rem",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    "Tab 2 Content"
                }
            }
            TabContent {
                index: 2usize,
                class: "tabs-content",
                value: "tab3".to_string(),
                div {
                    width: "100%",
                    height: "5rem",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    "Tab 3 Content"
                }
            }
        }
    }
}
