use dioxus::prelude::*;
use dioxus_primitives::{
    separator::Separator,
    tabs::{TabContent, TabTrigger, Tabs},
};

mod components;

#[derive(Clone, PartialEq)]
struct ComponentDemoData {
    name: &'static str,
    rs_highlighted: HighlightedCode,
    css_highlighted: HighlightedCode,
    component: fn() -> Element,
}

fn main() {
    dioxus::launch(App);
}

#[derive(Copy, Clone, PartialEq)]
struct HighlightedCode {
    light: &'static str,
    dark: &'static str,
}

#[component]
fn CodeBlock(source: HighlightedCode) -> Element {
    rsx! {
        pre {
            class: "code-block code-block-dark",
            dangerous_inner_html: source.dark
        }
        pre {
            class: "code-block code-block-light",
            dangerous_inner_html: source.light
        }
    }
}

#[component]
fn ComponentHighlight(demo: ComponentDemoData) -> Element {
    let ComponentDemoData {
        name,
        rs_highlighted,
        css_highlighted,
        component: Comp,
    } = demo;
    let name = name.replace("_", " ");

    rsx! {
        div { class: "component-demo",
            h3 { class: "component-title", {name} }
            div { class: "component-description", "Component Description" }
            div { class: "component-preview",
                div {
                    class: "component-preview-contents",
                    div {
                        class: "component-preview-frame",
                        Comp {}
                    }
                    Separator {
                        class: "component-preview-separator",
                        horizontal: true,
                    }
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

                            TabContent { class: "tabs-content", value: "main.rs", CodeBlock { source: rs_highlighted } }
                            TabContent { class: "tabs-content", value: "style.css", CodeBlock { source: css_highlighted } }
                        }
                    }
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

        ComponentGallery {}

        // for demo in components::DEMOS.iter().cloned() {
        //     ComponentHighlight { demo }
        // }
    }
}

#[component]
fn ComponentGallery() -> Element {
    rsx! {
        div { class: "masonry-with-columns",
            for ComponentDemoData { component: Comp, .. } in components::DEMOS.iter().cloned() {
                div {
                    class: "masonry-preview-frame",
                    Comp {}
                }
            }
        }
    }
}
