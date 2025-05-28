use dioxus::{prelude::*};
use dioxus_primitives::tabs::{TabContent, TabTrigger, Tabs};


#[component]
pub(super) fn TabsExample() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/tabs/style.css") }
        Tabs { class: "tabs", default_value: "tab1".to_string(),

            div { class: "tabs-list",
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

            TabContent { class: "tabs-content", value: "tab1".to_string(), "Tab 1 Content" }
            TabContent { class: "tabs-content", value: "tab2".to_string(), "Tab 2 Content" }
            TabContent { class: "tabs-content", value: "tab3".to_string(), "Tab 3 Content" }
        }
    }
}
