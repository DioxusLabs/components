use dioxus::prelude::*;
use dioxus_primitives::{
    separator::Separator,
    tabs::{TabContent, TabTrigger, Tabs},
};

mod components;

#[derive(Clone, PartialEq)]
struct ComponentDemoData {
    name: &'static str,
    rs_source: &'static str,
    css_source: &'static str,
    component: fn() -> Element,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn ComponentDemo(
    demo: ComponentDemoData,
) -> Element {
    let ComponentDemoData { name, rs_source, css_source, component: Comp } = demo;

    rsx! {
        div { class: "component-demo",
            h3 { class: "component-title", {name} }
            div { class: "component-description", "Component Description" }
            div { class: "component-preview", Comp {} }
            div { class: "component-code",
                Tabs { class: "tabs", default_value: "main.rs",
                    div { class: "tabs-list",
                        TabTrigger {
                            class: "tabs-trigger",
                            value: "main.rs",
                            index: 0usize,
                            "main.rs"
                        }
                        TabTrigger {
                            class: "tabs-trigger",
                            value: "style.css",
                            index: 1usize,
                            "style.css"
                        }
                    }

                    TabContent { class: "tabs-content", value: "main.rs", div { dangerous_inner_html: rs_source } }
                    TabContent { class: "tabs-content", value: "style.css", div { dangerous_inner_html: css_source } }
                }
            }
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/hero.css") }

        div { id: "hero",
            h1 { "Dioxus Primitives" }
            h2 { "Accessible, unstyled foundational components for Dioxus." }
        }
        Separator { id: "hero-separator", class: "separator", horizontal: true }

        for demo in components::DEMOS.iter().cloned() {
            ComponentDemo { demo }
        }
    }
}
