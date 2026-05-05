use core::panic;

use crate::components::tabs::component::*;
use crate::dioxus_router::LinkProps;
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_primitives::icon::Icon;

use std::str::FromStr;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};
use unic_langid::{langid, LanguageIdentifier};

mod components;
mod dashboard;
mod theme;

#[derive(Copy, Clone, PartialEq)]
enum ComponentType {
    /// Normal component as default.
    Normal,
    /// Component that render the preview inside an iframe for isolation.
    Block,
}

#[derive(Clone, PartialEq)]
struct ComponentDemoData {
    name: &'static str,
    r#type: ComponentType,
    description: &'static str,
    docs: &'static str,
    component: HighlightedCode,
    style: HighlightedCode,
    variants: &'static [ComponentVariantDemoData],
}

#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Clone, PartialEq)]
struct ComponentVariantDemoData {
    name: &'static str,
    rs_highlighted: HighlightedCode,
    css_highlighted: Option<HighlightedCode>,
    component: fn() -> Element,
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}

#[cfg(feature = "server")]
fn main() {
    use dioxus::server::axum::{routing::post, Json, Router};
    use dioxus::server::{DioxusRouterExt, IncrementalRendererConfig, ServeConfig};

    dioxus::server::serve(|| async {
        let cfg = ServeConfig::builder()
            // Enable incremental rendering
            .incremental(
                IncrementalRendererConfig::new()
                    // Store static files in the public directory where other static assets like wasm are stored
                    .static_dir(
                        std::env::current_exe()
                            .unwrap()
                            .parent()
                            .unwrap()
                            .join("public"),
                    )
                    // Don't clear the public folder on every build. The public folder has other files including the wasm
                    // binary and static assets required for the app to run
                    .clear_cache(false),
            )
            .enable_out_of_order_streaming();

        // Workaround for dioxus-cli 0.7.6: with `--base-path`, the `static_routes`
        // server function ends up under `/<base>/api/static_routes`, but the SSG
        // step POSTs to the unprefixed `/api/static_routes` and fails to parse
        // the empty body. Expose a shim at the root that returns the route list.
        let router = Router::new()
            .route(
                "/api/static_routes",
                post(|| async {
                    Json(
                        Route::static_routes()
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<String>>(),
                    )
                }),
            )
            .serve_dioxus_application(cfg, App);

        Ok(router)
    })
}

#[component]
fn App() -> Element {
    use_init_i18n(|| {
        I18nConfig::new(langid!("en-US"))
            .with_locale((langid!("en-US"), include_str!("i18n/en-US.ftl")))
            .with_locale((langid!("fr-FR"), include_str!("i18n/fr-FR.ftl")))
            .with_locale((langid!("es-ES"), include_str!("i18n/es-ES.ftl")))
            .with_locale((langid!("de-DE"), include_str!("i18n/de-DE.ftl")))
    });

    rsx! {
        Router::<Route> {}
    }
}

#[derive(Routable, Clone, PartialEq)]
pub(crate) enum Route {
    #[layout(AppLayout)]
    #[layout(NavigationLayout)]
    #[route("/?:iframe&:dark_mode")]
    Home {
        iframe: Option<bool>,
        dark_mode: Option<bool>,
    },
    #[route("/docs?:dark_mode")]
    Docs { dark_mode: Option<bool> },
    #[route("/component/?:name&:iframe&:dark_mode")]
    ComponentDemo {
        name: String,
        iframe: Option<bool>,
        dark_mode: Option<bool>,
    },
    #[end_layout]
    #[route("/component/block/?:name&:variant&:dark_mode")]
    ComponentBlockDemo {
        name: String,
        variant: Option<String>,
        dark_mode: Option<bool>,
    },
    #[route("/dashboard/email-client?:dark_mode")]
    EmailClientDashboard { dark_mode: Option<bool> },
}

impl Route {
    fn iframe(&self) -> Option<bool> {
        match self {
            Route::Home { iframe, .. } => *iframe,
            Route::Docs { .. } => None,
            Route::ComponentDemo { iframe, .. } => *iframe,
            Route::ComponentBlockDemo { .. } => None,
            Route::EmailClientDashboard { .. } => None,
        }
    }

    fn in_iframe() -> Option<bool> {
        let route: Self = router().current();
        route.iframe()
    }

    fn dark_mode(&self) -> Option<bool> {
        match self {
            Route::Home { dark_mode, .. } => *dark_mode,
            Route::Docs { dark_mode, .. } => *dark_mode,
            Route::ComponentDemo { dark_mode, .. } => *dark_mode,
            Route::ComponentBlockDemo { dark_mode, .. } => *dark_mode,
            Route::EmailClientDashboard { dark_mode, .. } => *dark_mode,
        }
    }

    fn in_dark_mode() -> Option<bool> {
        let route: Self = router().current();
        route.dark_mode()
    }

    fn home() -> Self {
        let iframe = Self::in_iframe();
        let dark_mode = Self::in_dark_mode();
        Self::Home { iframe, dark_mode }
    }

    fn docs() -> Self {
        let dark_mode = Self::in_dark_mode();
        Self::Docs { dark_mode }
    }

    fn component(name: impl ToString) -> Self {
        let iframe = Self::in_iframe();
        let dark_mode = Self::in_dark_mode();
        Self::ComponentDemo {
            name: name.to_string(),
            iframe,
            dark_mode,
        }
    }
}

#[component]
fn AppLayout() -> Element {
    use_effect(move || {
        theme::theme_seed();
        if let Some(dark_mode) = Route::in_dark_mode() {
            theme::set_theme(dark_mode);
        }
    });

    rsx! {
        Outlet::<Route> {}
    }
}

#[component]
fn NavigationLayout() -> Element {
    // Send the route to the parent window if in an iframe
    let mut initial_route = use_hook(|| CopyValue::new(true));
    use_effect(move || {
        let route: Route = router().current();

        // Only send route changes, not the initial route
        if initial_route() || !Route::in_iframe().unwrap_or_default() {
            initial_route.set(false);
            return;
        }

        let eval = document::eval(
            "let route = await dioxus.recv();
            window.top.postMessage({ 'route': route }, '*');",
        );
        let _ = eval.send(route.to_string());
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/dx-components-theme.css"),
        }
        document::Link { rel: "stylesheet", href: asset!("/assets/hero.css") }
        Navbar {}
        Outlet::<Route> {}
    }
}

#[component]
fn Navbar() -> Element {
    let in_iframe = Route::in_iframe().unwrap_or_default();
    let in_component = matches!(router().current(), Route::ComponentDemo { .. });
    if in_iframe {
        return rsx! {
            nav {
                class: "dx-preview-navbar",
                border: "none",
                padding: "1rem",
                justify_content: "flex-start",
                if in_component {
                    Link {
                        to: Route::home(),
                        class: "dx-navbar-brand",
                        aria_label: "Back",
                        Icon {
                            width: "2rem",
                            height: "2rem",
                            stroke: "var(--secondary-color-4)",
                            path { d: "M15 18 L9 12 L15 6" }
                        }
                    }
                }
            }
        };
    }
    rsx! {
        nav { class: "dx-preview-navbar",
            div { class: "dx-navbar-inner",
                div { class: "dx-navbar-primary",
                    Link { to: Route::home(), class: "dx-navbar-brand",
                        img {
                            src: asset!("/assets/dioxus_color.svg"),
                            alt: "Dioxus Logo",
                            width: "28",
                            height: "28",
                        }
                        span { "DioxusUI" }
                    }
                    Link { to: Route::docs(), class: "dx-navbar-link", "Docs" }
                    Link {
                        to: Route::EmailClientDashboard { dark_mode: Route::in_dark_mode() },
                        class: "dx-navbar-link",
                        "Demos"
                    }
                }
                div { class: "dx-navbar-utilities",
                    // TODO: restore once the primitives crate is published
                    // Link {
                    //     to: "https://crates.io/crates/dioxus-components",
                    //     class: "dx-navbar-link",
                    //     aria_label: "DioxusUI crates.io",
                    //     Icon {
                    //         width: "24px",
                    //         height: "24px",
                    //         viewBox: ViewBox::new(0, 0, 576, 512),
                    //         path {
                    //             d: "M290.8 48.6l78.4 29.7L288 109.5 206.8 78.3l78.4-29.7c1.8-.7 3.8-.7 5.7 0zM136 92.5l0 112.2c-1.3 .4-2.6 .8-3.9 1.3l-96 36.4C14.4 250.6 0 271.5 0 294.7L0 413.9c0 22.2 13.1 42.3 33.5 51.3l96 42.2c14.4 6.3 30.7 6.3 45.1 0L288 457.5l113.5 49.9c14.4 6.3 30.7 6.3 45.1 0l96-42.2c20.3-8.9 33.5-29.1 33.5-51.3l0-119.1c0-23.3-14.4-44.1-36.1-52.4l-96-36.4c-1.3-.5-2.6-.9-3.9-1.3l0-112.2c0-23.3-14.4-44.1-36.1-52.4l-96-36.4c-12.8-4.8-26.9-4.8-39.7 0l-96 36.4C150.4 48.4 136 69.3 136 92.5zM392 210.6l-82.4 31.2 0-89.2L392 121l0 89.6zM154.8 250.9l78.4 29.7L152 311.7 70.8 280.6l78.4-29.7c1.8-.7 3.8-.7 5.7 0zm18.8 204.4l0-100.5L256 323.2l0 95.9-82.4 36.2zM421.2 250.9c1.8-.7 3.8-.7 5.7 0l78.4 29.7L424 311.7l-81.2-31.1 78.4-29.7zM523.2 421.2l-77.6 34.1 0-100.5L528 323.2l0 90.7c0 3.2-1.9 6-4.8 7.3z",
                    //             fill: "currentColor",
                    //             fill_rule: "nonzero",
                    //         }
                    //     }
                    // }
                    Link {
                        to: "https://github.com/DioxusLabs/components",
                        class: "dx-navbar-link",
                        img {
                            class: "dx-light-mode-only",
                            src: asset!("/assets/github-mark/github-mark.svg"),
                            alt: "GitHub",
                            width: "24",
                            height: "24",
                        }
                        img {
                            class: "dx-dark-mode-only",
                            src: asset!("/assets/github-mark/github-mark-white.svg"),
                            alt: "GitHub",
                            width: "24",
                            height: "24",
                        }
                    }
                    theme::DarkModeToggle {}
                    LanguageSelect {}
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
    rsx! {
        div {
            class: "dx-code-block dx-dark-code-block",
            tabindex: "0",
            "data-collapsed": "{collapsed}",
            dangerous_inner_html: source.dark,
        }
        div {
            class: "dx-code-block dx-light-code-block",
            tabindex: "0",
            "data-collapsed": "{collapsed}",
            dangerous_inner_html: source.light,
        }
        CopyButton { position: "absolute", top: "0.5em", right: "0.5em" }
    }
}

#[component]
fn CopyButton(#[props(extends=GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    let mut copied = use_signal(|| false);

    rsx! {
        button {
            class: "dx-copy-button",
            r#type: "button",
            aria_label: "Copy code",
            "data-copied": copied,
            "onclick": "navigator.clipboard.writeText(this.parentNode.firstChild.innerText || this.parentNode.innerText);",
            onclick: move |_| copied.set(true),
            ..attributes,
            if copied() {
                CheckIcon {}
            } else {
                CopyIcon {}
            }
        }
    }
}

#[component]
fn CopyIcon() -> Element {
    rsx! {
        Icon {
            width: "24px",
            height: "25px",
            fill: "currentColor",
            stroke: "none",
            stroke_width: 1.5,
            // Clipboard image from octicons (MIT) https://github.com/primer/octicons/blob/v2.0.0/svg/clippy.svg
            path { d: "M18 20h2v3c0 1-1 2-2 2H2c-.998 0-2-1-2-2V5c0-.911.755-1.667 1.667-1.667h5A3.323 3.323 0 0110 0a3.323 3.323 0 013.333 3.333h5C19.245 3.333 20 4.09 20 5v8.333h-2V9H2v14h16v-3zM3 7h14c0-.911-.793-1.667-1.75-1.667H13.5c-.957 0-1.75-.755-1.75-1.666C11.75 2.755 10.957 2 10 2s-1.75.755-1.75 1.667c0 .911-.793 1.666-1.75 1.666H4.75C3.793 5.333 3 6.09 3 7z" }
            path { d: "M4 19h6v2H4zM12 11H4v2h8zM4 17h4v-2H4zM15 15v-3l-4.5 4.5L15 21v-3l8.027-.032L23 15z" }
        }
    }
}

#[component]
fn CheckIcon() -> Element {
    rsx! {
        Icon {
            width: "24px",
            height: "25px",
            path { d: "M5 13l4 4L19 7" }
        }
    }
}

#[derive(PartialEq, Display, EnumIter, EnumString)]
enum Language {
    English,
    French,
    Spanish,
    German,
}

impl Language {
    const fn id(&self) -> LanguageIdentifier {
        match self {
            Language::English => langid!("en-US"),
            Language::French => langid!("fr-FR"),
            Language::Spanish => langid!("es-ES"),
            Language::German => langid!("de-DE"),
        }
    }

    const fn flag(&self) -> &'static str {
        match self {
            Language::English => "🇬🇧",
            Language::French => "🇫🇷",
            Language::Spanish => "🇪🇸",
            Language::German => "🇩🇪",
        }
    }

    fn display_name(&self) -> String {
        format!("{} {}", self.flag(), self.localize_name())
    }

    const fn localize_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::French => "Français",
            Language::Spanish => "Español",
            Language::German => "Deutsch",
        }
    }
}

#[component]
fn LanguageSelect() -> Element {
    let mut current_lang = use_signal(|| Language::English);

    rsx! {
        document::Stylesheet { href: asset!("/assets/language-select.css") }
        div { class: "dx-language-container",
            span { class: "dx-language-select-container",
                select {
                    class: "dx-language-select",
                    aria_label: "Language",
                    onchange: move |e| {
                        let name = e.value().parse().unwrap_or(current_lang.to_string());
                        if let Ok(lang) = Language::from_str(&name) {
                            current_lang.set(lang);
                        }
                        let id = current_lang.read().id();
                        tracing::info!("Current lang: {id}");
                        i18n().set_language(id);
                    },
                    for lang in Language::iter() {
                        option {
                            value: lang.to_string(),
                            selected: lang == *current_lang.read(),
                            {lang.display_name()}
                        }
                    }
                }
                span { class: "dx-language-select-value",
                    {current_lang.read().flag()}
                    Icon {
                        class: "dx-select-expand-icon",
                        width: "20px",
                        height: "20px",
                        stroke: "var(--secondary-color-4)",
                        polyline { points: "6 9 12 15 18 9" }
                    }
                }
            }
        }
    }
}

#[component]
fn ComponentCode(
    rs_highlighted: HighlightedCode,
    css_highlighted: HighlightedCode,
    #[props(default = ComponentType::Normal)] component_type: ComponentType,
) -> Element {
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
            r#type: "button",
            onclick: move |_| {
                collapsed.toggle();
            },
            Icon {
                width: "20px",
                height: "20px",
                stroke: "var(--secondary-color-4)",
                if collapsed() {
                    polyline { points: "6 9 12 15 18 9" }
                } else {
                    polyline { points: "6 15 12 9 18 15" }
                }
            }
        }
    };

    rsx! {
        Tabs {
            default_value: "main.rs",
            border_bottom_left_radius: "0.5rem",
            border_bottom_right_radius: "0.5rem",
            horizontal: true,
            width: "100%",
            TabList {
                TabTrigger { value: "main.rs", index: 0usize, "main.rs" }
                TabTrigger { value: "style.css", index: 1usize, "style.css" }
                if component_type != ComponentType::Block {
                    TabTrigger { value: "dx-components-theme.css", index: 2usize, "dx-components-theme.css" }
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
                    value: "main.rs",
                    width: "100%",
                    position: "relative",
                    CodeBlock { source: rs_highlighted, collapsed: collapsed() }
                    {expand.clone()}
                }
                TabContent {
                    index: 1usize,
                    value: "style.css",
                    width: "100%",
                    position: "relative",
                    CodeBlock { source: css_highlighted, collapsed: collapsed() }
                    {expand.clone()}
                }
                if component_type != ComponentType::Block {
                    TabContent {
                        index: 2usize,
                        value: "dx-components-theme.css",
                        width: "100%",
                        position: "relative",
                        CodeBlock { source: THEME_CSS, collapsed: collapsed() }
                        {expand.clone()}
                    }
                }
            }
        }
    }
}

#[component]
fn CollapsibleCodeBlock(highlighted: HighlightedCode) -> Element {
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
            r#type: "button",
            onclick: move |_| {
                collapsed.toggle();
            },
            Icon {
                width: "20px",
                height: "20px",
                stroke: "var(--secondary-color-4)",
                if collapsed() {
                    polyline { points: "6 9 12 15 18 9" }
                } else {
                    polyline { points: "6 15 12 9 18 15" }
                }
            }
        }
    };

    rsx! {
        div {
            class: "dx-tabs-content-extra",
            width: "100%",
            height: "100%",
            display: "flex",
            flex_direction: "column",
            justify_content: "center",
            align_items: "center",
            border_bottom_left_radius: "0.5rem",
            border_bottom_right_radius: "0.5rem",
            CodeBlock { source: highlighted, collapsed: collapsed() }
            {expand.clone()}
        }
    }
}

#[component]
fn Docs(dark_mode: Option<bool>) -> Element {
    rsx! {
        main { class: "dx-docs-layout",
            DocsSidebar { active_component: None }
            article { class: "dx-docs-page dx-docs-prose",
                header { class: "dx-docs-page-header",
                    p { class: "dx-docs-eyebrow", "Docs" }
                    h1 { "Build with DioxusUI" }
                    p {
                        "DioxusUI is a collection of styled, accessible Dioxus components designed to be copied into your app. Use the CLI when you want the fastest path, or copy the source when you want complete ownership."
                    }
                }
                section { class: "dx-docs-section",
                    h2 { "How it works" }
                    div { class: "dx-docs-feature-grid",
                        div {
                            h3 { "Copy-first components" }
                            p { "Each component ships as source code and CSS you can keep, edit, and theme inside your own project." }
                        }
                        div {
                            h3 { "Shared theme tokens" }
                            p { "Import the theme CSS once, then component styles use the same color, focus, and state variables." }
                        }
                        div {
                            h3 { "Dioxus primitives underneath" }
                            p { "The styled components are built on reusable primitives for accessibility, keyboard interaction, and state." }
                        }
                    }
                }
                section { class: "dx-docs-section",
                    h2 { "Add a component" }
                    p { "Run the add command from your Dioxus app. Swap the final name for any component in the sidebar." }
                    div { class: "dx-docs-command",
                        code { "dx components add button" }
                        CopyCommandButton { command: "dx components add button".to_string() }
                    }
                    p { class: "dx-docs-muted",
                        "If you do not have the Dioxus CLI yet, install it once with cargo install dioxus-cli."
                    }
                }
                section { class: "dx-docs-section",
                    h2 { "Recommended workflow" }
                    ol {
                        li { "Pick a component from the sidebar or catalog." }
                        li { "Preview the default example and variants." }
                        li { "Run the CLI command shown on the component page." }
                        li { "Customize the generated Rust and CSS to fit your app." }
                    }
                }
            }
        }
    }
}

#[component]
fn DocsSidebar(active_component: Option<&'static str>) -> Element {
    rsx! {
        aside { class: "dx-docs-sidebar", aria_label: "Docs navigation",
            nav {
                div { class: "dx-docs-sidebar-section",
                    p { class: "dx-docs-sidebar-heading", "Start" }
                    Link {
                        to: Route::docs(),
                        class: if active_component.is_none() { "dx-docs-sidebar-link dx-docs-sidebar-link-active" } else { "dx-docs-sidebar-link" },
                        "Overview"
                    }
                }
                div { class: "dx-docs-sidebar-section",
                    p { class: "dx-docs-sidebar-heading", "Components" }
                    for component in components::DEMOS {
                        Link {
                            to: Route::component(component.name),
                            class: if active_component == Some(component.name) { "dx-docs-sidebar-link dx-docs-sidebar-link-active" } else { "dx-docs-sidebar-link" },
                            {component.name.replace("_", " ")}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ComponentDemo(iframe: Option<bool>, dark_mode: Option<bool>, name: String) -> Element {
    let route = router().current::<Route>();
    tracing::info!("route: {route}");
    let Some(demo) = components::DEMOS
        .iter()
        .find(|demo| demo.name == name)
        .cloned()
    else {
        return rsx! {
            main { class: "dx-component-demo-not-found",
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
        name: raw_name,
        r#type,
        docs,
        description,
        variants,
        component,
        style,
    } = demo;
    let name = raw_name.replace("_", " ");
    let [main, variants @ ..] = variants else {
        unreachable!("Expected at least one variant for component: {}", name);
    };

    rsx! {
        main { class: "dx-docs-layout",
            DocsSidebar { active_component: Some(raw_name) }
            article { class: "dx-component-page",
                header { class: "dx-component-page-header",
                    p { class: "dx-docs-eyebrow", "Component" }
                    h1 { "{name}" }
                    p { "{description}" }
                }
                section { class: "dx-component-section",
                    match r#type {
                        ComponentType::Normal => rsx! {
                            ComponentVariantHighlight { variant: main.clone(), main_variant: true, component_name: Some(raw_name) }
                        },
                        ComponentType::Block => rsx! {
                            BlockComponentVariantHighlight { variant: main.clone(), main_variant: true, component_name: raw_name, show_install: true }
                        },
                    }
                }
                section { class: "dx-component-section",
                    div { class: "dx-component-section-heading",
                        h2 { "Installation" }
                        p { "Use the CLI command for the common path, or copy the component files manually." }
                    }
                    details { class: "dx-component-manual-install",
                        summary { "Manual installation files" }
                        ManualComponentInstallation { component, style }
                    }
                }
                section { class: "dx-component-section dx-docs-prose",
                    div { class: "dx-component-section-heading",
                        h2 { "Usage notes" }
                    }
                    div { class: "dx-component-description",
                        div { dangerous_inner_html: docs }
                    }
                }
                if !variants.is_empty() {
                    section { class: "dx-component-section",
                        div { class: "dx-component-section-heading",
                            h2 { "Variants" }
                            p { "Alternative examples for common configurations." }
                        }
                        for variant in variants {
                            div { class: "dx-component-variant",
                                match r#type {
                                    ComponentType::Normal => rsx! {
                                        ComponentVariantHighlight { variant: variant.clone(), main_variant: false, component_name: None }
                                    },
                                    ComponentType::Block => rsx! {
                                        BlockComponentVariantHighlight { variant: variant.clone(), main_variant: false, component_name: raw_name, show_install: false }
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ComponentInstallCommand(name: &'static str) -> Element {
    let command = format!("dx components add {name}");

    rsx! {
        div { class: "dx-component-inline-command",
            code { "{command}" }
            CopyCommandButton { command: command.clone() }
        }
    }
}

#[component]
fn ManualComponentInstallation(component: HighlightedCode, style: HighlightedCode) -> Element {
    rsx! {
        p { class: "dx-docs-muted",
            "Copy the component source and CSS into your app. Import the shared theme CSS once near your app root."
        }
        ComponentCode {
            rs_highlighted: component,
            css_highlighted: style,
            component_type: ComponentType::Normal,
        }
    }
}

#[component]
fn ComponentVariantHighlight(
    variant: ComponentVariantDemoData,
    main_variant: bool,
    component_name: Option<&'static str>,
) -> Element {
    let ComponentVariantDemoData {
        name,
        rs_highlighted: highlighted,
        css_highlighted: _,
        component: Comp,
    } = variant;
    rsx! {
        if !main_variant {
            h3 { class: "dx-component-variant-title", "{name}" }
        }
        Tabs {
            default_value: "Demo",
            border_bottom_left_radius: "0.5rem",
            border_bottom_right_radius: "0.5rem",
            horizontal: true,
            width: "100%",
            variant: TabsVariant::Ghost,
            div { class: "dx-component-tabs-header",
                TabList {
                    TabTrigger { value: "Demo", index: 0usize, "DEMO" }
                    TabTrigger { value: "Code", index: 1usize, "CODE" }
                }
                if let Some(component_name) = component_name {
                    ComponentInstallCommand { name: component_name }
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
                    class: "dx-component-preview-frame",
                    id: "component-preview-frame",
                    value: "Demo",
                    width: "100%",
                    position: "relative",
                    Comp {}
                }
                TabContent {
                    index: 1usize,
                    class: "dx-component-preview-frame",
                    value: "Code",
                    width: "100%",
                    position: "relative",
                    CollapsibleCodeBlock { highlighted }
                }
            }
        }
    }
}

#[component]
fn BlockComponentVariantHighlight(
    component_name: &'static str,
    variant: ComponentVariantDemoData,
    main_variant: bool,
    show_install: bool,
) -> Element {
    let ComponentVariantDemoData {
        name,
        rs_highlighted: highlighted,
        css_highlighted,
        component: _,
    } = variant;

    let route_path = Route::ComponentBlockDemo {
        name: component_name.to_string(),
        variant: Some(name.to_string()),
        dark_mode: Route::in_dark_mode(),
    }
    .to_string();

    let iframe_src = match router().prefix() {
        Some(prefix) => format!("{prefix}{route_path}"),
        None => route_path,
    };

    rsx! {
        if !main_variant {
            h3 { class: "dx-component-variant-title", "{name}" }
        }
        Tabs {
            default_value: "Preview",
            border_bottom_left_radius: "0.5rem",
            border_bottom_right_radius: "0.5rem",
            horizontal: true,
            width: "100%",
            variant: TabsVariant::Ghost,
            div { class: "dx-component-tabs-header",
                TabList {
                    TabTrigger { value: "Preview", index: 0usize, "PREVIEW" }
                    TabTrigger { value: "Code", index: 1usize, "CODE" }
                }
                if show_install {
                    ComponentInstallCommand { name: component_name }
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
                    id: "component-preview-frame",
                    value: "Preview",
                    width: "100%",
                    position: "relative",
                    iframe {
                        src: "{iframe_src}",
                        width: "100%",
                        height: "600px",
                        border: "1px solid var(--primary-color-6)",
                        border_radius: "0.5em",
                    }
                }
                TabContent {
                    index: 1usize,
                    value: "Code",
                    width: "100%",
                    position: "relative",
                    if let Some(css) = css_highlighted {
                        ComponentCode {
                            rs_highlighted: highlighted,
                            css_highlighted: css,
                            component_type: ComponentType::Block,
                        }
                    } else {
                        CollapsibleCodeBlock { highlighted }
                    }
                }
            }
        }
    }
}

#[component]
fn EmailClientDashboard(dark_mode: Option<bool>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/dx-components-theme.css") }
        dashboard::views::email_client::EmailClient {}
    }
}

#[component]
fn ComponentBlockDemo(name: String, variant: Option<String>, dark_mode: Option<bool>) -> Element {
    let Some(demo) = components::DEMOS.iter().find(|d| d.name == name).cloned() else {
        return rsx! {
            div { "Block component not found" }
        };
    };

    let variant = match variant.as_deref() {
        Some(wanted) => match demo.variants.iter().find(|v| v.name == wanted) {
            Some(v) => v,
            None => {
                return rsx! {
                    div {
                        style: "min-height: 100vh; display: flex; align-items: center; justify-content: center; padding: 2rem;",
                        "Variant content not found: {wanted}"
                    }
                };
            }
        },
        None => &demo.variants[0],
    };

    let Comp = variant.component;

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/dx-components-theme.css"),
        }
        div { style: "min-height: 100vh;", Comp {} }
    }
}

#[component]
fn Home(iframe: Option<bool>, dark_mode: Option<bool>) -> Element {
    rsx! {
        main { class: "dx-home-page", role: "main",
            div { id: "hero",
                div { class: "dx-hero-shell",
                    h1 { "DioxusUI" }
                    p { class: "dx-hero-summary",
                        "Accessible, themeable interface pieces for Dioxus apps. Browse the catalog, copy the CLI command, and pull only what you need into your project."
                    }
                    div { class: "dx-hero-command",
                        span { class: "dx-hero-prompt", "$" }
                        code { "dx components list" }
                        CopyCommandButton { command: "dx components list".to_string() }
                    }
                    nav { class: "dx-hero-cloud", aria_label: "Component links",
                        for component in components::DEMOS {
                            Link {
                                to: Route::component(component.name),
                                class: "dx-hero-cloud-link",
                                {component.name.replace("_", " ")}
                            }
                        }
                    }
                }
            }
            ComponentGallery {}
        }
    }
}

#[component]
fn ComponentGallery() -> Element {
    rsx! {
        div { class: "dx-component-gallery",
            for component in components::DEMOS.iter().cloned() {
                ComponentGalleryPreview { component }
            }
        }
    }
}

#[component]
fn ComponentGalleryPreview(component: ComponentDemoData) -> Element {
    let ComponentDemoData {
        name,
        r#type,
        description,
        variants,
        ..
    } = component;

    let first_variant = &variants[0];
    let Comp = first_variant.component;
    let display_name = name.replace("_", " ");
    let install_command = format!("dx components add {name}");

    let preview = match r#type {
        ComponentType::Normal => rsx! {
            Comp {}
        },
        ComponentType::Block => rsx! {
            Link {
                to: Route::component(name),
                class: "dx-component-card-block-link",
                "Open full preview"
                Icon {
                    width: "18px",
                    height: "18px",
                    stroke: "currentColor",
                    path { d: "M7 7h10v10" }
                    path { d: "M7 17 17 7" }
                }
            }
        },
    };

    rsx! {
        article { class: "dx-component-card",
            div { class: "dx-component-card-meta",
                h3 { class: "dx-component-card-title",
                    Link {
                        to: Route::component(name),
                        class: "dx-component-card-title-link",
                        "{display_name}"
                        Icon {
                            width: "18px",
                            height: "18px",
                            stroke: "currentColor",
                            path { d: "M7 7h10v10" }
                            path { d: "M7 17 17 7" }
                        }
                    }
                }
                p { class: "dx-component-card-description", "{description}" }
                div { class: "dx-component-card-actions",
                    div { class: "dx-component-card-command",
                        code { "{install_command}" }
                        CopyCommandButton { command: install_command.clone() }
                    }
                }
            }
            div { class: "dx-component-card-preview", {preview} }
        }
    }
}

#[component]
fn CopyCommandButton(command: String) -> Element {
    let mut copied = use_signal(|| false);

    rsx! {
        button {
            class: "dx-copy-button dx-component-card-copy",
            r#type: "button",
            aria_label: "Copy install command",
            "data-command": "{command}",
            "data-copied": copied,
            "onclick": "navigator.clipboard.writeText(this.dataset.command);",
            onclick: move |_| copied.set(true),
            if copied() {
                CheckIcon {}
            } else {
                CopyIcon {}
            }
        }
    }
}

#[component]
fn GotoIcon(mut props: LinkProps) -> Element {
    props.children = rsx! {
        Icon {
            width: "20px",
            height: "20px",
            stroke: "var(--secondary-color-4)",
            stroke_width: 0.25,
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
        "/dx-components-theme.css.base16-ocean.light.html"
    )),
    dark: include_str!(concat!(
        env!("OUT_DIR"),
        "/dx-components-theme.css.base16-ocean.dark.html"
    )),
};
