use core::panic;

use crate::components::input::component::Input;
use crate::components::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarGroup, SidebarGroupContent,
    SidebarGroupLabel, SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuButton, SidebarMenuItem,
    SidebarProvider, SidebarTrigger,
};
use crate::components::{separator::Separator, tabs::component::*};
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
    #[route("/component/?:name&:iframe&:dark_mode")]
    ComponentDemo {
        name: String,
        iframe: Option<bool>,
        dark_mode: Option<bool>,
    },
    #[route("/?:iframe&:dark_mode")]
    Home {
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

    let search = use_signal(String::new);
    let in_iframe = Route::in_iframe().unwrap_or_default();

    let stylesheets = rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/dx-components-theme.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/hero.css") }
    };

    if in_iframe {
        let in_component = matches!(router().current(), Route::ComponentDemo { .. });
        return rsx! {
            {stylesheets}
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
            Outlet::<Route> {}
        };
    }

    rsx! {
        {stylesheets}
        SidebarProvider {
            Sidebar { collapsible: SidebarCollapsible::Offcanvas,
                AppSidebarBody { search }
            }
            SidebarInset {
                header { class: "dx-home-topbar",
                    SidebarTrigger {}
                    Separator { horizontal: false, decorative: true }
                    Link { to: Route::home(), class: "dx-navbar-brand",
                        img {
                            src: asset!("/assets/dioxus_color.svg"),
                            alt: "Dioxus Logo",
                            width: "32",
                            height: "32",
                        }
                    }
                    div { style: "margin-left: auto; display: flex; align-items: center; gap: 1rem;",
                        Link {
                            to: Route::EmailClientDashboard { dark_mode: Route::in_dark_mode() },
                            class: "dx-demos-link",
                            "Demos"
                        }
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
                div { class: "dx-app-main",
                    Outlet::<Route> {}
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
        variants,
        component,
        style,
    } = demo;
    let name = raw_name.replace("_", " ");
    let [main, variants @ ..] = variants else {
        unreachable!("Expected at least one variant for component: {}", name);
    };

    rsx! {
        main { class: "dx-component-demo",
            h1 { class: "dx-component-title", "{name}" }
            div { class: "dx-component-preview",
                div { class: "dx-component-preview-contents",
                    match r#type {
                        ComponentType::Normal => rsx! {
                            ComponentVariantHighlight { variant: main.clone(), main_variant: true }
                        },
                        ComponentType::Block => rsx! {
                            BlockComponentVariantHighlight { variant: main.clone(), main_variant: true, component_name: raw_name }
                        },
                    }
                    div { class: "dx-component-installation",
                        h2 { "Installation" }
                        Tabs {
                            default_value: "Automatic",
                            border_bottom_left_radius: "0.5rem",
                            border_bottom_right_radius: "0.5rem",
                            horizontal: true,
                            width: "100%",
                            variant: TabsVariant::Ghost,
                            TabList {
                                TabTrigger { value: "Automatic", index: 0usize, "Automatic" }
                                TabTrigger { value: "Manual", index: 1usize, "Manual" }
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
                                    value: "Automatic",
                                    width: "100%",
                                    position: "relative",
                                    CliComponentInstallation { name: raw_name }
                                }
                                TabContent {
                                    index: 1usize,
                                    value: "Manual",
                                    width: "100%",
                                    position: "relative",
                                    ManualComponentInstallation { component, style }
                                }
                            }
                        }
                    }
                    div { class: "dx-component-description",
                        div { dangerous_inner_html: docs }
                    }
                    if !variants.is_empty() {
                        h2 { class: "dx-component-variants-title", "Variants" }
                        for variant in variants {
                            match r#type {
                                ComponentType::Normal => rsx! {
                                    ComponentVariantHighlight { variant: variant.clone(), main_variant: false }
                                },
                                ComponentType::Block => rsx! {
                                    BlockComponentVariantHighlight { variant: variant.clone(), main_variant: false, component_name: raw_name }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ManualComponentInstallation(component: HighlightedCode, style: HighlightedCode) -> Element {
    rsx! {
        ol { class: "dx-component-installation-list",
            li {
                "If you haven't already, add the dx-components-theme.css file to your project and import it in the root of your app."
            }
            li { "Add the style.css file to your project." }
            li { "Create a component based on the main.rs below." }
            li { "Modify your components and styles as needed." }
        }
        ComponentCode {
            rs_highlighted: component,
            css_highlighted: style,
            component_type: ComponentType::Normal,
        }
    }
}

#[component]
fn CliComponentInstallation(name: String) -> Element {
    rsx! {
        ol { class: "dx-component-installation-list",
            li {
                "Install the 0.7.0 version of the CLI"
                div { id: "hero-installation",
                    "> "
                    div {
                        width: "100%",
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "space-between",
                        align_items: "center",
                        "cargo install dioxus-cli"
                        CopyButton {}
                    }
                }
            }
            li {
                "Add the component to your project using the dx components add command:"
                div { id: "hero-installation",
                    "> "
                    div {
                        width: "100%",
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "space-between",
                        align_items: "center",
                        "dx components add {name}"
                        CopyButton {}
                    }
                }
            }
        }
    }
}

#[component]
fn ComponentVariantHighlight(variant: ComponentVariantDemoData, main_variant: bool) -> Element {
    let ComponentVariantDemoData {
        name,
        rs_highlighted: highlighted,
        css_highlighted: _,
        component: Comp,
    } = variant;
    rsx! {
        if !main_variant {
            h3 { "{name}" }
        }
        Tabs {
            default_value: "Demo",
            border_bottom_left_radius: "0.5rem",
            border_bottom_right_radius: "0.5rem",
            horizontal: true,
            width: "100%",
            variant: TabsVariant::Ghost,
            TabList {
                TabTrigger { value: "Demo", index: 0usize, "DEMO" }
                TabTrigger { value: "Code", index: 1usize, "CODE" }
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
            h3 { "{name}" }
        }
        Tabs {
            default_value: "Preview",
            border_bottom_left_radius: "0.5rem",
            border_bottom_right_radius: "0.5rem",
            horizontal: true,
            width: "100%",
            variant: TabsVariant::Ghost,
            TabList {
                TabTrigger { value: "Preview", index: 0usize, "PREVIEW" }
                TabTrigger { value: "Code", index: 1usize, "CODE" }
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
        Hero {}
        section { class: "dx-gallery-section",
            div { class: "dx-gallery-header",
                h2 { class: "dx-gallery-title", "All components" }
            }
            ComponentGallery {}
        }
        CtaStrip {}
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        div { class: "hero-snippet",
            div { class: "hero-snippet-text",
                h1 { class: "hero-snippet-title", "Dioxus Components" }
                p { class: "hero-snippet-tagline",
                    "Accessible components that just feel nice."
                }
                div { class: "hero-snippet-actions",
                    a { href: "#components", class: "hero-snippet-cta",
                        "Browse components"
                        span { aria_hidden: true, " →" }
                    }
                    a {
                        href: "https://github.com/DioxusLabs/components",
                        class: "hero-snippet-cta hero-snippet-cta-ghost",
                        "GitHub"
                    }
                }
            }
            div { class: "hero-snippet-code",
                div { class: "hero-snippet-code-tab", "src/main.rs" }
                pre { class: "hero-snippet-code-block",
                    span { class: "hero-snippet-tok-kw", "use " }
                    "dioxus::prelude::*;\n"
                    span { class: "hero-snippet-tok-kw", "use " }
                    "components::"
                    span { class: "hero-snippet-tok-type", "{{Button, ButtonVariant}}" }
                    ";\n\n"
                    span { class: "hero-snippet-tok-kw", "rsx! " }
                    "{{\n    "
                    span { class: "hero-snippet-tok-type", "Button" }
                    " {{\n        variant: ButtonVariant::"
                    span { class: "hero-snippet-tok-prop", "Primary" }
                    ",\n        "
                    span { class: "hero-snippet-tok-str", "\"Get started\"" }
                    "\n    }}\n}}"
                }
            }
        }
    }
}


#[component]
fn AppSidebarBody(search: Signal<String>) -> Element {
    let needle = search.read().to_lowercase();
    let matches: Vec<&ComponentDemoData> = components::DEMOS
        .iter()
        .filter(|c| needle.is_empty() || c.name.to_lowercase().contains(&needle))
        .collect();
    let count = matches.len();

    rsx! {
        SidebarHeader {
            Input {
                r#type: "search",
                placeholder: "Search components…",
                aria_label: "Search components",
                value: search,
                oninput: move |e: FormEvent| {
                    search.set(e.value());
                },
            }
        }
        SidebarContent {
            SidebarGroup {
                SidebarGroupLabel { "Components ({count})" }
                SidebarGroupContent {
                    if matches.is_empty() {
                        div { "No matches" }
                    } else {
                        SidebarMenu {
                            for component in matches.iter() {
                                SidebarMenuItem { key: "{component.name}",
                                    SidebarMenuButton {
                                        as: {
                                            let name = component.name;
                                            move |attrs: Vec<Attribute>| rsx! {
                                                Link {
                                                    to: Route::component(name),
                                                    attributes: attrs,
                                                    {name.replace("_", " ")}
                                                }
                                            }
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
}

#[component]
fn CtaStrip() -> Element {
    rsx! {
        section { class: "dx-cta",
            div { class: "dx-cta-text",
                span { class: "dx-cta-eyebrow", "Open source · MIT" }
                h3 { class: "dx-cta-title",
                    "Built on Dioxus primitives. Yours to customize."
                }
                p { class: "dx-cta-body",
                    "Copy the source straight into your project — every component, every theme variable, every animation."
                }
            }
            div { display: "flex", flex_direction: "column", align_items: "flex-end",
                div { class: "dx-cta-snippet",
                    span { color: "var(--highlight-color-main)", "›" }
                    "dx components add button"
                }
                div { class: "dx-cta-actions",
                    Link {
                        to: Route::EmailClientDashboard { dark_mode: Route::in_dark_mode() },
                        class: "dx-cta-button",
                        "View demos"
                        Icon {
                            width: "14px",
                            height: "14px",
                            stroke: "currentColor",
                            stroke_width: 2.2,
                            path { d: "M5 12h14M13 5l7 7-7 7" }
                        }
                    }
                    Link {
                        to: "https://github.com/DioxusLabs/components",
                        class: "dx-cta-button dx-cta-button-secondary",
                        "Star on GitHub"
                    }
                }
            }
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
        variants,
        ..
    } = component;

    let first_variant = &variants[0];
    let Comp = first_variant.component;

    let preview = match r#type {
        ComponentType::Normal => rsx! {
            Comp {}
        },
        ComponentType::Block => rsx! {
            BlockMockup {}
        },
    };

    rsx! {
        div { class: "dx-gallery-card",
            div {
                class: "dx-gallery-card-preview",
                "data-name": name,
                div { class: "dx-gallery-card-preview-stage", {preview} }
            }
            Link {
                to: Route::component(name),
                class: "dx-gallery-card-footer",
                aria_label: "{name} details",
                h3 { class: "dx-gallery-card-title", {name.replace("_", " ")} }
                Icon {
                    class: "dx-gallery-card-goto",
                    width: "16px",
                    height: "16px",
                    stroke: "currentColor",
                    stroke_width: 2.0,
                    path { d: "M5 12h14M13 5l7 7-7 7" }
                }
            }
        }
    }
}

#[component]
fn BlockMockup() -> Element {
    rsx! {
        div {
            display: "flex",
            width: "16rem",
            height: "10rem",
            border: "1px solid var(--primary-color-6)",
            border_radius: "0.5rem",
            background: "var(--primary-color-2)",
            overflow: "hidden",
            div {
                width: "5rem",
                background: "var(--primary-color-3)",
                border_right: "1px solid var(--primary-color-6)",
                display: "flex",
                flex_direction: "column",
                gap: "0.4rem",
                padding: "0.6rem 0.5rem",
                for w in [60, 80, 50, 70, 65] {
                    div {
                        height: "0.5rem",
                        width: "{w}%",
                        border_radius: "0.25rem",
                        background: "var(--primary-color-5)",
                    }
                }
            }
            div {
                flex: "1",
                padding: "0.75rem",
                display: "flex",
                flex_direction: "column",
                gap: "0.45rem",
                div {
                    height: "0.65rem",
                    width: "65%",
                    border_radius: "0.25rem",
                    background: "var(--primary-color-5)",
                }
                div {
                    height: "0.45rem",
                    width: "100%",
                    border_radius: "0.25rem",
                    background: "var(--primary-color-4)",
                }
                div {
                    height: "0.45rem",
                    width: "85%",
                    border_radius: "0.25rem",
                    background: "var(--primary-color-4)",
                }
                div {
                    height: "0.45rem",
                    width: "70%",
                    border_radius: "0.25rem",
                    background: "var(--primary-color-4)",
                }
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
