use dioxus::prelude::*;
use dioxus_primitives::{
    collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
    separator::Separator,
    tabs::{TabContent, TabTrigger, Tabs},
    toast::ToastProvider,
};

macro_rules! example {
    ($name:ident) => {
        mod $name;
        use $name::*;
    };
}

example!(accordion);
example!(aspect_ratio);
example!(avatar);
example!(calendar);
example!(context_menu);
example!(dropdown_menu);
example!(hover_card);
example!(form);
example!(menubar);
example!(progress);
example!(radio_group);
example!(scroll_area);
example!(select);
example!(separator);
example!(slider);
example!(switch);
example!(tabs);
example!(toast);
example!(toggle_group);
example!(toolbar);
example!(tooltip);

fn main() {
    dioxus::launch(|| {
        rsx! {
            ComponentDemo {
                rs_html: syntect_html::syntect_html! {
                    r#"fn main() {
    println!("Hello, world!");
}"#,
                    "rs", "base16-ocean.dark"
                },
                css_html: syntect_html::syntect_html! {
                    r#"body {
    margin: 0;
    padding: 0;
}"#,
                    "css", "base16-ocean.dark"
                },
                "component body"
            }
        }
    });
}

#[component]
fn ComponentDemo(rs_html: String, css_html: String, children: Element) -> Element {
    rsx! {
        div { class: "component-demo",
            h3 { class: "component-title", "Component Title" }
            div { class: "component-description", "Component Description" }
            div { class: "component-preview", {children} }
            div { class: "component-code",
                document::Link { rel: "stylesheet", href: asset!("/src/tabs/style.css") }
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

                    TabContent { class: "tabs-content", value: "main.rs", div { dangerous_inner_html: rs_html } }
                    TabContent { class: "tabs-content", value: "style.css", div { dangerous_inner_html: css_html } }
                }
            }
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        ToastProvider {
            document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }

            document::Link { rel: "stylesheet", href: asset!("/assets/hero.css") }
            div { id: "hero",
                h1 { "Dioxus Primitives" }
                h2 { "Accessible, unstyled foundational components for Dioxus." }
            }
            Separator { id: "hero-separator", class: "separator", horizontal: true }


            Collapsible {
                CollapsibleTrigger { "Toggle Group Example" }
                CollapsibleContent { ToggleGroupExample {} }
            }


            Collapsible {
                CollapsibleTrigger { "Form Example" }
                CollapsibleContent { FormExample {} }
            }


            Collapsible {
                CollapsibleTrigger { "Aspect Ratio Example" }
                CollapsibleContent { AspectRatioExample {} }
            }

            Separator {
                class: "separator",
                style: "margin: 15px 0;",
                horizontal: true,
                decorative: true,
            }

            document::Link { rel: "stylesheet", href: asset!("/src/progress/style.css") }
            Collapsible {
                CollapsibleTrigger { "Progress Example" }
                CollapsibleContent { ProgressExample {} }
            }

            Separator {
                class: "separator",
                style: "margin: 15px 0;",
                horizontal: true,
                decorative: true,
            }

            Collapsible {
                CollapsibleTrigger { "Accordion Example" }
                CollapsibleContent { AccordionExample {} }
            }

            Separator {
                class: "separator",
                style: "margin: 15px 0;",
                horizontal: true,
                decorative: true,
            }

            document::Link { rel: "stylesheet", href: asset!("/src/switch/style.css") }
            Collapsible {
                CollapsibleTrigger { "Switch Example" }
                CollapsibleContent { SwitchExample {} }
            }

            Separator {
                class: "separator",
                style: "margin: 15px 0;",
                horizontal: true,
                decorative: true,
            }

            document::Link { rel: "stylesheet", href: asset!("/src/slider/style.css") }
            Collapsible {
                CollapsibleTrigger { "Slider Example" }
                CollapsibleContent { SliderExample {} }
            }

            Separator {
                class: "separator",
                style: "margin: 15px 0;",
                horizontal: true,
                decorative: true,
            }

            document::Link { rel: "stylesheet", href: asset!("/src/toast/style.css") }
            Collapsible {
                CollapsibleTrigger { "Toast Example" }
                CollapsibleContent { ToastExample {} }
            }

            Separator {
                class: "separator",
                style: "margin: 15px 0;",
                horizontal: true,
                decorative: true,
            }
        }

        Collapsible {
            CollapsibleTrigger { "Avatar Example" }
            CollapsibleContent { AvatarExample {} }
        }

        Separator {
            class: "separator",
            style: "margin: 15px 0;",
            horizontal: true,
            decorative: true,
        }

        Collapsible {
            CollapsibleTrigger { "Radio Group Example" }
            CollapsibleContent { RadioGroupExample {} }
        }

        Separator {
            class: "separator",
            style: "margin: 15px 0;",
            horizontal: true,
            decorative: true,
        }

        Collapsible {
            CollapsibleTrigger { "Tabs Example" }
            CollapsibleContent { TabsExample {} }
        }

        Separator {
            class: "separator",
            style: "margin: 15px 0;",
            horizontal: true,
            decorative: true,
        }

        Collapsible {
            CollapsibleTrigger { "Dropdown Menu Example" }
            CollapsibleContent { DropdownMenuExample {} }
        }

        Separator {
            class: "separator",
            style: "margin: 15px 0;",
            horizontal: true,
            decorative: true,
        }

        Collapsible {
            CollapsibleTrigger { "Menubar Example" }
            CollapsibleContent { MenubarExample {} }
        }

        Separator {
            class: "separator",
            style: "margin: 15px 0;",
            horizontal: true,
            decorative: true,
        }

        Collapsible {
            CollapsibleTrigger { "Scroll Area Example" }
            CollapsibleContent { ScrollAreaExample {} }
        }

        Separator {
            class: "separator",
            style: "margin: 15px 0;",
            horizontal: true,
            decorative: true,
        }

        Collapsible {
            CollapsibleTrigger { "Context Menu Example" }
            CollapsibleContent { ContextMenuExample {} }
        }

        Separator {
            class: "separator",
            style: "margin: 15px 0;",
            horizontal: true,
            decorative: true,
        }

        Collapsible {
            CollapsibleTrigger { "Toolbar Example" }
            CollapsibleContent { ToolbarExample {} }
        }

        Separator {
            class: "separator",
            style: "margin: 15px 0;",
            horizontal: true,
            decorative: true,
        }

        Collapsible {
            CollapsibleTrigger { "Hover Card Example" }
            CollapsibleContent { HoverCardExample {} }
        }

        Separator {
            class: "separator",
            style: "margin: 15px 0;",
            horizontal: true,
            decorative: true,
        }

        document::Link { rel: "stylesheet", href: asset!("/src/tooltip/style.css") }
        Collapsible {
            CollapsibleTrigger { "Tooltip Example" }
            CollapsibleContent { TooltipExample {} }
        }

        Separator {
            class: "separator",
            style: "margin: 15px 0;",
            horizontal: true,
            decorative: true,
        }

        document::Link { rel: "stylesheet", href: asset!("/src/select/style.css") }
        Collapsible {
            CollapsibleTrigger { "Select Example" }
            CollapsibleContent { SelectExample {} }
        }

        Separator {
            class: "separator",
            style: "margin: 15px 0;",
            horizontal: true,
            decorative: true,
        }

        document::Link { rel: "stylesheet", href: asset!("/src/calendar/style.css") }
        Collapsible {
            CollapsibleTrigger { "Calendar Example" }
            CollapsibleContent { CalendarExample {} }
        }
    }
}
