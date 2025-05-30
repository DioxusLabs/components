use dioxus::prelude::*;
use dioxus_primitives::{
    separator::Separator,
    tabs::{TabContent, TabTrigger, Tabs},
};
mod components;
#[derive(Clone, PartialEq)]
struct ComponentDemoData {
    name: &'static str,
    docs: &'static str,
    rs_highlighted: HighlightedCode,
    css_highlighted: HighlightedCode,
    component: fn() -> Element,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[derive(Routable, Clone, PartialEq)]
pub(crate) enum Route {
    #[layout(NavigationLayout)]
    #[route("/")]
    Home,
    #[route("/component/:component_name")]
    ComponentDemo { component_name: String },
}

#[component]
fn NavigationLayout() -> Element {
    rsx! {
        Navbar {}
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/hero.css") }
        document::Link { rel: "stylesheet", href: asset!("/src/components/tabs/style.css") }
        Outlet::<Route> {}
    }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        nav { class: "navbar",
            Link {
                to: Route::Home,
                class: "navbar-brand",
                img {
                    src: asset!("/assets/dioxus_color.svg"),
                    alt: "Dioxus Logo",
                    width: "32",
                    height: "32",
                }
            }
            div { class: "navbar-links",
                Link {
                    to: Route::Home,
                    class: "navbar-link",
                    "Home"
                }
                Link {
                    to: "https://docs.rs/crate/dioxus-components/latest",
                    class: "navbar-link",
                    "docs.rs"
                }
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
struct HighlightedCode {
    light: &'static str,
    dark: &'static str,
}

#[component]
fn CodeBlock(source: HighlightedCode, collapsed: bool) -> Element {
    let mut copied = use_signal(|| false);
    rsx! {
        pre {
            class: "code-block code-block-dark",
            "data-collapsed": "{collapsed}",
            dangerous_inner_html: source.dark,
        }
        pre {
            class: "code-block code-block-light",
            "data-collapsed": "{collapsed}",
            dangerous_inner_html: source.light,
        }
        button {
            class: "copy-button",
            "data-copied": copied,
            "onclick": "navigator.clipboard.writeText(this.parentNode.firstChild.innerText);",
            onclick: move |_| copied.set(true),
            if copied() {
                "Copied!"
            }
            CopyIcon {}
        }
    }
}

#[component]
fn CopyIcon() -> Element {
    rsx! {
        svg {
            width: "24",
            height: "24",
            stroke_width: "1.5",
            fill: "none",
            stroke: "currentColor",
            path { d: "M8 16c0 1.886 0 2.828.586 3.414C9.172 20 10.114 20 12 20h4c1.886 0 2.828 0 3.414-.586C20 18.828 20 17.886 20 16v-4c0-1.886 0-2.828-.586-3.414C18.828 8 17.886 8 16 8m-8 8h4c1.886 0 2.828 0 3.414-.586C16 14.828 16 13.886 16 12V8m-8 8c-1.886 0-2.828 0-3.414-.586C4 14.828 4 13.886 4 12V8c0-1.886 0-2.828.586-3.414C5.172 4 6.114 4 8 4h4c1.886 0 2.828 0 3.414.586C16 5.172 16 6.114 16 8" }
        }
    }
}

#[component]
fn ComponentCode(rs_highlighted: HighlightedCode, css_highlighted: HighlightedCode) -> Element {
    let mut collapsed = use_signal(|| true);

    rsx! {
        Tabs { class: "tabs", default_value: "main.rs",
            border_bottom_left_radius: "0.5rem",
            border_bottom_right_radius: "0.5rem",
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
            div {
                width: "100%",
                height: "100%",
                display: "flex",
                flex_direction: "column",
                justify_content: "center",
                align_items: "center",
                TabContent { class: "tabs-content", value: "main.rs", width: "100%",  position: "relative",
                    CodeBlock { source: rs_highlighted, collapsed: collapsed() }
                }
                TabContent { class: "tabs-content", value: "style.css", width: "100%", position: "relative",
                    CodeBlock { source: css_highlighted, collapsed: collapsed() }
                }
                button {
                    width: "100%",
                    height: "2rem",
                    color: "var(--text-color)",
                    background_color: "rgba(0, 0, 0, 0)",
                    border_radius: "0 0 0.5rem 0.5rem",
                    border: "none",
                    text_align: "center",
                    onclick: move |_| {
                        collapsed.toggle();
                    },
                    if collapsed() {
                        "↓"
                    } else {
                        "↑"
                    }
                }
            }
        }
    }
}

#[component]
fn ComponentDemo(component_name: String) -> Element {
    let Some(demo) = components::DEMOS
        .iter()
        .find(|demo| demo.name == component_name)
        .cloned()
    else {
        return rsx! {
            div { class: "component-demo-not-found",
                h3 { "Component not found" }
                p { "The requested component does not exist." }
            }
        };
    };
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/prism.css") }
        script { src: asset!("/assets/prism.js") }
        ComponentHighlight { demo }
    }
}

#[component]
fn ComponentHighlight(demo: ComponentDemoData) -> Element {
    let ComponentDemoData {
        name,
        docs,
        rs_highlighted,
        css_highlighted,
        component: Comp,
    } = demo;
    let name = name.replace("_", " ");
    rsx! {
        div { class: "component-demo",
            h3 { class: "component-title", {name} }
            div { class: "component-preview",
                div { class: "component-preview-contents",
                    div { class: "component-preview-frame", Comp {} }
                    Separator {
                        class: "component-preview-separator",
                        horizontal: true,
                    }
                    div { class: "component-code",
                        ComponentCode {
                            rs_highlighted: rs_highlighted,
                            css_highlighted: css_highlighted,
                        }
                    }
                }
            }
            div { class: "component-description",
                div { dangerous_inner_html: docs }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    let mut search = use_signal(|| String::new());

    rsx! {
        div { id: "hero",
            h1 { "Dioxus Primitives" }
            h2 {
                b{ "Accessible" }
                ", "
                i { "unstyled" }
                " foundational components for Dioxus."
            }
            div {
                id: "hero-search-container",
                input {
                    id: "hero-search-input",
                    type: "search",
                    placeholder: "Search components...",
                    value: search,
                    oninput: move |e| {
                        search.set(e.value());
                    },
                }
            }
        }
        ComponentGallery {
            search
        }
    }
}

#[component]
fn ComponentGallery(search: String) -> Element {
    rsx! {
        div { class: "masonry-with-columns",
            for ComponentDemoData { component : Comp, name, .. } in components::DEMOS.iter().cloned() {
                if search.is_empty() || name.to_lowercase().contains(&search.to_lowercase()) {
                    div { class: "masonry-preview-frame", position: "relative",
                        h3 { class: "component-title", {name.replace("_", " ")} }
                        GotoIcon {
                            class: "goto-icon",
                            position: "absolute",
                            margin: "0.5rem",
                            top: "0",
                            right: "0",
                            to: Route::ComponentDemo {
                                component_name: name.to_string(),
                            },
                        }
                        div { class: "masonry-component-frame",
                            Comp {}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn GotoIcon(mut props: LinkProps) -> Element {
    props.children = rsx! {
        svg {
            width: "20",
            height: "20",
            view_box: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M5 21q-.825 0-1.412-.587T3 19V5q0-.825.588-1.412T5 3h7v2H5v14h14v-7h2v7q0 .825-.587 1.413T19 21zm4.7-5.3l-1.4-1.4L17.6 5H14V3h7v7h-2V6.4z",
                fill: "var(--text-color)",
            }
        }
    };
    Link(props)
}
