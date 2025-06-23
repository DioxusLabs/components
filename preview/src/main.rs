use dioxus::prelude::*;
use dioxus_primitives::tabs::{TabContent, TabList, TabTrigger, Tabs};

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
    #[route("/?:iframe")]
    Home { iframe: bool },
    #[route("/component/:component_name?:iframe")]
    ComponentDemo {
        component_name: String,
        iframe: bool,
    },
}

impl Route {
    fn iframe(&self) -> bool {
        match self {
            Route::Home { iframe } => *iframe,
            Route::ComponentDemo { iframe, .. } => *iframe,
        }
    }

    fn in_iframe() -> bool {
        let route: Self = router().current();
        route.iframe()
    }

    fn home() -> Self {
        let iframe = Self::in_iframe();
        Self::Home { iframe }
    }

    fn component(component_name: impl ToString) -> Self {
        let iframe = Self::in_iframe();
        Self::ComponentDemo {
            component_name: component_name.to_string(),
            iframe,
        }
    }
}

#[component]
fn NavigationLayout() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/theme.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/hero.css") }
        document::Link { rel: "stylesheet", href: asset!("/src/components/tabs/style.css") }
        Navbar {}
        Outlet::<Route> {}
    }
}

#[component]
fn Navbar() -> Element {
    let in_iframe = Route::in_iframe();
    let in_component = matches!(router().current(), Route::ComponentDemo { .. });
    if in_iframe {
        return rsx! {
            nav {
                class: "preview-navbar",
                border: "none",
                padding: "1rem",
                justify_content: if !in_component { "flex-end" },
                if in_component {
                    Link { to: Route::home(), class: "navbar-brand",
                        aria_label: "Back",
                        svg {
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "2rem",
                            height: "2rem",
                            fill: "none",
                            stroke: "var(--secondary-color-4)",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: 2,
                            path {
                                d: "M15 18 L9 12 L15 6",
                            }
                        }
                    }
                }
                DarkModeToggle {}
            }
        };
    }
    rsx! {
        nav { class: "preview-navbar",
            Link { to: Route::home(), class: "navbar-brand",
                img {
                    src: asset!("/assets/dioxus_color.svg"),
                    alt: "Dioxus Logo",
                    width: "32",
                    height: "32",
                }
            }
            div { class: "navbar-links",
                Link {
                    to: "https://crates.io/crates/dioxus-components",
                    class: "navbar-link",
                    aria_label: "Dioxus components crates.io",
                    svg {
                        "viewBox": "0 0 576 512",
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "24",
                        height: "24",
                        path {
                            d: "M290.8 48.6l78.4 29.7L288 109.5 206.8 78.3l78.4-29.7c1.8-.7 3.8-.7 5.7 0zM136 92.5l0 112.2c-1.3 .4-2.6 .8-3.9 1.3l-96 36.4C14.4 250.6 0 271.5 0 294.7L0 413.9c0 22.2 13.1 42.3 33.5 51.3l96 42.2c14.4 6.3 30.7 6.3 45.1 0L288 457.5l113.5 49.9c14.4 6.3 30.7 6.3 45.1 0l96-42.2c20.3-8.9 33.5-29.1 33.5-51.3l0-119.1c0-23.3-14.4-44.1-36.1-52.4l-96-36.4c-1.3-.5-2.6-.9-3.9-1.3l0-112.2c0-23.3-14.4-44.1-36.1-52.4l-96-36.4c-12.8-4.8-26.9-4.8-39.7 0l-96 36.4C150.4 48.4 136 69.3 136 92.5zM392 210.6l-82.4 31.2 0-89.2L392 121l0 89.6zM154.8 250.9l78.4 29.7L152 311.7 70.8 280.6l78.4-29.7c1.8-.7 3.8-.7 5.7 0zm18.8 204.4l0-100.5L256 323.2l0 95.9-82.4 36.2zM421.2 250.9c1.8-.7 3.8-.7 5.7 0l78.4 29.7L424 311.7l-81.2-31.1 78.4-29.7zM523.2 421.2l-77.6 34.1 0-100.5L528 323.2l0 90.7c0 3.2-1.9 6-4.8 7.3z",
                            fill: "currentColor",
                            fill_rule: "nonzero",
                        }
                    }
                }
                Link {
                    to: "https://github.com/DioxusLabs/components",
                    class: "navbar-link",
                    img {
                        class: "light-mode-only",
                        src: asset!("/assets/github-mark/github-mark.svg"),
                        alt: "GitHub",
                        width: "24",
                        height: "24",
                    }
                    img {
                        class: "dark-mode-only",
                        src: asset!("/assets/github-mark/github-mark-white.svg"),
                        alt: "GitHub",
                        width: "24",
                        height: "24",
                    }
                }
                DarkModeToggle {}
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
        div {
            class: "code-block dark-code-block",
            tabindex: "0",
            "data-collapsed": "{collapsed}",
            dangerous_inner_html: source.dark,
        }
        div {
            class: "code-block light-code-block",
            tabindex: "0",
            "data-collapsed": "{collapsed}",
            dangerous_inner_html: source.light,
        }
        button {
            class: "copy-button",
            aria_label: "Copy code",
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
fn DarkModeToggle() -> Element {
    let mut is_dark_mode = use_signal(|| false);

    rsx! {
        button {
            class: "dark-mode-toggle dark-mode-only",
            onclick: move |_| {
                is_dark_mode.toggle();
                _ = document::eval("document.documentElement.setAttribute('data-theme', 'light');");
            },
            aria_label: "Enable light mode",
            DarkModeIcon {}
        }
        button {
            class: "dark-mode-toggle light-mode-only",
            onclick: move |_| {
                is_dark_mode.toggle();
                _ = document::eval("document.documentElement.setAttribute('data-theme', 'dark');");
            },
            aria_label: "Enable dark mode",
            LightModeIcon {}
        }
    }
}

#[component]
fn DarkModeIcon() -> Element {
    rsx! {
        svg {
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M21 12.79A9 9 0 1 1 11.21 3a7 7 0 0 0 9.79 9.79z" }
        }
    }
}

#[component]
fn LightModeIcon() -> Element {
    rsx! {
        svg {
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            circle { cx: "12", cy: "12", r: "4" }
            line { x1: "12", y1: "1", x2: "12", y2: "3" }
            line { x1: "12", y1: "21", x2: "12", y2: "23" }
            line { x1: "4.22", y1: "4.22", x2: "5.64", y2: "5.64" }
            line { x1: "18.36", y1: "18.36", x2: "19.78", y2: "19.78" }
            line { x1: "1", y1: "12", x2: "3", y2: "12" }
            line { x1: "21", y1: "12", x2: "23", y2: "12" }
            line { x1: "4.22", y1: "19.78", x2: "5.64", y2: "18.36" }
            line { x1: "18.36", y1: "5.64", x2: "19.78", y2: "4.22" }
        }
    }
}

#[component]
fn ComponentCode(rs_highlighted: HighlightedCode, css_highlighted: HighlightedCode) -> Element {
    let mut collapsed = use_signal(|| true);

    let expand = rsx! {
        button {
            aria_label: if collapsed() { "Expand code" } else { "Collapse code" },
            width: "100%",
            height: "2rem",
            color: "var(--secondary-color-4)",
            background_color: "rgba(0, 0, 0, 0)",
            border_radius: "0 0 0.5rem 0.5rem",
            border: "none",
            text_align: "center",
            onclick: move |_| {
                collapsed.toggle();
            },
            if collapsed() {
                svg {
                    fill: "none",
                    xmlns: "http://www.w3.org/2000/svg",
                    stroke: "var(--secondary-color-4)",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    width: "20px",
                    height: "20px",
                    view_box: "0 0 24 24",
                    polyline { points: "6 9 12 15 18 9" }
                }
            } else {
                svg {
                    fill: "none",
                    xmlns: "http://www.w3.org/2000/svg",
                    stroke: "var(--secondary-color-4)",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    width: "20px",
                    height: "20px",
                    view_box: "0 0 24 24",
                    polyline { points: "6 15 12 9 18 15" }
                }
            }
        }
    };

    rsx! {
        Tabs {
            class: "tabs",
            default_value: "main.rs",
            border_bottom_left_radius: "0.5rem",
            border_bottom_right_radius: "0.5rem",
            horizontal: true,
            width: "100%",
            TabList { class: "tabs-list",
                TabTrigger { class: "tabs-trigger", value: "main.rs", index: 0usize, "main.rs" }
                TabTrigger {
                    class: "tabs-trigger",
                    value: "style.css",
                    index: 1usize,
                    "style.css"
                }
                TabTrigger {
                    class: "tabs-trigger",
                    value: "theme.css",
                    index: 2usize,
                    "theme.css"
                }
            }
            div {
                width: "100%",
                height: "100%",
                display: "flex",
                flex_direction: "column",
                justify_content: "center",
                align_items: "center",
                TabContent {
                    index: 0usize,
                    class: "tabs-content",
                    value: "main.rs",
                    width: "100%",
                    position: "relative",
                    CodeBlock { source: rs_highlighted, collapsed: collapsed() }
                    {expand.clone()}
                }
                TabContent {
                    index: 1usize,
                    class: "tabs-content",
                    value: "style.css",
                    width: "100%",
                    position: "relative",
                    CodeBlock { source: css_highlighted, collapsed: collapsed() }
                    {expand.clone()}
                }
                TabContent {
                    index: 2usize,
                    class: "tabs-content",
                    value: "theme.css",
                    width: "100%",
                    position: "relative",
                    CodeBlock { source: THEME_CSS, collapsed: collapsed() }
                    {expand.clone()}
                }
            }
        }
    }
}

#[component]
fn ComponentDemo(iframe: bool, component_name: String) -> Element {
    let Some(demo) = components::DEMOS
        .iter()
        .find(|demo| demo.name == component_name)
        .cloned()
    else {
        return rsx! {
            main { class: "component-demo-not-found",
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
        main { class: "component-demo",
            h1 { class: "component-title", {name} }
            div { class: "component-preview",
                div { class: "component-preview-contents",
                    div { class: "component-preview-frame", Comp {} }
                    div { class: "component-installation",
                        h2 { "Installation" }
                        ol { class: "component-installation-list",
                            li { "If you haven't already, add the theme.css file to your project and import it in the root of your app." }
                            li { "Add the style.css file to your project." }
                            li { "Create a component based on the main.rs below." }
                            li { "Modify your components and styles as needed." }
                        }
                    }
                    div { class: "component-code",
                        ComponentCode { rs_highlighted, css_highlighted }
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
fn Home(iframe: bool) -> Element {
    let mut search = use_signal(String::new);

    rsx! {
        main {
            role: "main",
            div { id: "hero",
                h1 { "Dioxus Primitives" }
                h2 {
                    b { "Accessible" }
                    ", "
                    i { "unstyled" }
                    " foundational components for Dioxus."
                }
                div { id: "hero-search-container",
                    input {
                        id: "hero-search-input",
                        r#type: "search",
                        placeholder: "Search components...",
                        value: search,
                        oninput: move |e| {
                            search.set(e.value());
                        },
                    }
                }
            }
            ComponentGallery { search }
        }
    }
}

#[component]
fn ComponentGallery(search: String) -> Element {
    rsx! {
        div { class: "masonry-with-columns",
            for ComponentDemoData { component : Comp , name , .. } in components::DEMOS.iter().cloned() {
                if search.is_empty() || name.to_lowercase().contains(&search.to_lowercase()) {
                    div { class: "masonry-preview-frame", position: "relative",
                        h3 { class: "component-title", {name.replace("_", " ")} }
                        GotoIcon {
                            class: "goto-icon",
                            position: "absolute",
                            margin: "0.5rem",
                            top: "0",
                            right: "0",
                            aria_label: "{name} details",
                            to: Route::component(name),
                        }
                        div { class: "masonry-component-frame", Comp {} }
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
                fill: "var(--secondary-color-4)",
            }
        }
    };
    Link(props)
}

const THEME_CSS: HighlightedCode = HighlightedCode {
    light: include_str!(concat!(
        env!("OUT_DIR"),
        "/theme.css.base16-ocean.light.html"
    )),
    dark: include_str!(concat!(
        env!("OUT_DIR"),
        "/theme.css.base16-ocean.dark.html"
    )),
};
