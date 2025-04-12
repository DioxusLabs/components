use dioxus::{document::eval, prelude::*};
use primitives::{
    Avatar, AvatarFallback,
    accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger},
    aspect_ratio::AspectRatio,
    checkbox::{Checkbox, CheckboxIndicator},
    collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
    dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger},
    menubar::{Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger},
    progress::{Progress, ProgressIndicator},
    radio_group::{RadioGroup, RadioItem},
    separator::Separator,
    slider::{Slider, SliderRange, SliderThumb, SliderTrack, SliderValue},
    switch::{Switch, SwitchThumb},
    tabs::{TabContent, TabTrigger, Tabs},
    toggle_group::{ToggleGroup, ToggleItem},
};

fn main() {
    dioxus::launch(App);
}

#[component]
fn TestBtn(#[props(extends = button)] attributes: Vec<Attribute>) -> Element {
    rsx! {
        button { ..attributes,"button!" }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/separator.css") }

        document::Link { rel: "stylesheet", href: asset!("/assets/hero.css") }
        div { id: "hero",
            h1 { "Dioxus Primitives" }
            h2 { "Accessible, unstyled foundational components for Dioxus." }
        }
        Separator { id: "hero-separator", class: "separator", horizontal: true }


        document::Link { rel: "stylesheet", href: asset!("/assets/toggle-group.css") }
        ToggleGroup { class: "toggle-group", horizontal: true,
            ToggleItem { class: "toggle-item", index: 0, "Align Left" }
            ToggleItem { class: "toggle-item", index: 1, "Align Middle" }
            ToggleItem { class: "toggle-item", index: 2, "Align Right" }
        }


        Collapsible {
            CollapsibleTrigger { "Form Example" }
            CollapsibleContent { FormExample {} }
        }

        Separator {
            class: "separator",
            style: "margin: 15px 0;",
            horizontal: true,
            decorative: true,
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

        document::Link { rel: "stylesheet", href: asset!("/assets/progress.css") }
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

        document::Link { rel: "stylesheet", href: asset!("/assets/switch.css") }
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

        document::Link { rel: "stylesheet", href: asset!("/assets/slider.css") }
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

        document::Link { rel: "stylesheet", href: asset!("/assets/radio-group.css") }
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
    }
}

#[component]
fn FormExample() -> Element {
    rsx! {
        form {
            onsubmit: move |e| {
                println!("{:?}", e.values());
            },

            Checkbox { id: "tos-check", name: "tos-check",
                CheckboxIndicator { "+" }
            }
            label { r#for: "tos-check", "I agree to the terms presented." }
            br {}
            button { r#type: "submit", "Submit" }
        }
    }
}

#[component]
fn AspectRatioExample() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/aspect-ratio.css") }
        div { class: "aspect-ratio-container",
            AspectRatio { ratio: 4.0 / 3.0,
                img {
                    class: "aspect-ratio-image",
                    src: "https://upload.wikimedia.org/wikipedia/commons/thumb/e/ea/Van_Gogh_-_Starry_Night_-_Google_Art_Project.jpg/1280px-Van_Gogh_-_Starry_Night_-_Google_Art_Project.jpg",
                }
            }
        }
    }
}

#[component]
fn AccordionExample() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/accordion.css") }
        Accordion {
            class: "accordion",
            allow_multiple_open: false,
            horizontal: false,

            for i in 0..4 {
                AccordionItem {
                    class: "accordion-item",
                    index: i,

                    on_change: move |open| {
                        eval(&format!("console.log({open});"));
                    },
                    on_trigger_click: move || {
                        eval("console.log('trigger');");
                    },

                    AccordionTrigger { class: "accordion-trigger", "the quick brown fox" }
                    AccordionContent { class: "accordion-content",
                        div { class: "accordion-content-inner",
                            p { "lorem ipsum lorem ipsum" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ProgressExample() -> Element {
    let mut progress = use_signal(|| 80.0);

    rsx! {
        Progress { class: "progress", value: Some(progress.into()),
            ProgressIndicator { class: "progress-indicator" }
        }
        button { onclick: move |_| progress.set(progress() + 10.0), "Increment" }
        button { onclick: move |_| progress.set(progress() - 10.0), "Decrement" }
        button { onclick: move |_| progress.set(0.0), "Reset" }
        button { onclick: move |_| progress.set(100.0), "Complete" }
    }
}

#[component]
fn SwitchExample() -> Element {
    let mut checked = use_signal(|| false);

    rsx! {
        div { class: "switch-example",
            label { "Airplane Mode" }
            Switch {
                class: "switch",
                checked,
                on_checked_change: move |new_checked| {
                    checked.set(new_checked);
                    eval(&format!("console.log('Switch toggled: {}')", new_checked));
                },

                SwitchThumb { class: "switch-thumb" }
            }
        }
    }
}

#[component]
fn SliderExample() -> Element {
    let mut value = use_signal(|| SliderValue::Single(50.0));
    let mut range_value = use_signal(|| SliderValue::Range(25.0, 75.0));

    rsx! {
        div { class: "slider-example",
            // Single value slider
            div {
                label { "Single Value Slider" }
                div { style: "display: flex; align-items: center; gap: 1rem;",
                    Slider {
                        class: "slider",
                        value,
                        on_value_change: move |v| {
                            value.set(v);
                        },

                        SliderTrack { class: "slider-track",
                            SliderRange { class: "slider-range" }
                            SliderThumb { class: "slider-thumb" }
                        }
                    }
                    input {
                        r#type: "text",
                        readonly: true,
                        value: match value() {
                            SliderValue::Single(v) => format!("{:.1}", v),
                            _ => String::new(),
                        },
                    }
                }
            }

            // Range slider
            div {
                label { "Range Slider" }
                div { style: "display: flex; align-items: center; gap: 1rem;",
                    Slider {
                        class: "slider",
                        value: range_value,
                        on_value_change: move |v| {
                            range_value.set(v);
                        },

                        SliderTrack { class: "slider-track",
                            SliderRange { class: "slider-range" }
                            SliderThumb { class: "slider-thumb", index: 0 }
                            SliderThumb { class: "slider-thumb", index: 1 }
                        }
                    }
                    input {
                        r#type: "text",
                        readonly: true,
                        value: match range_value() {
                            SliderValue::Range(start, end) => format!("{:.1}, {:.1}", start, end),
                            _ => String::new(),
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn AvatarExample() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./assets/avatar.css") }
        div { class: "avatar-example",
            // Avatar with image
            Avatar {
                class: "avatar",
                src: "https://github.com/DioxusLabs.png",
                alt: "Dioxus Labs",
            }

            // Avatar with fallback text
            Avatar { class: "avatar", alt: "John Doe" }

            // Avatar with custom fallback
            Avatar {
                class: "avatar",
                src: "invalid-url",
                fallback: rsx! {
                    AvatarFallback { class: "avatar-fallback", "👤" }
                },
            }
        }
    }
}

#[component]
fn RadioGroupExample() -> Element {
    let mut value = use_signal(|| String::from("option1"));

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/radio-group.css") }
        RadioGroup {
            class: "radio-group",
            value,
            on_value_change: move |new_value| {
                value.set(new_value);
            },

            RadioItem { class: "radio-item", value: "option1".to_string(), index: 0, "Option 1" }
            RadioItem { class: "radio-item", value: "option2".to_string(), index: 1, "Option 2" }
            RadioItem { class: "radio-item", value: "option3".to_string(), index: 2, "Option 3" }
        }

        div { style: "margin-top: 1rem;", "Selected value: {value()}" }
    }
}

#[component]
fn TabsExample() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/tabs.css") }
        Tabs { class: "tabs", default_value: "tab1".to_string(),

            div { class: "tabs-list",
                TabTrigger {
                    class: "tabs-trigger",
                    value: "tab1".to_string(),
                    index: 0,
                    "Tab 1"
                }
                TabTrigger {
                    class: "tabs-trigger",
                    value: "tab2".to_string(),
                    index: 1,
                    "Tab 2"
                }
                TabTrigger {
                    class: "tabs-trigger",
                    value: "tab3".to_string(),
                    index: 2,
                    "Tab 3"
                }
            }

            TabContent { class: "tabs-content", value: "tab1".to_string(), "Tab 1 Content" }
            TabContent { class: "tabs-content", value: "tab2".to_string(), "Tab 2 Content" }
            TabContent { class: "tabs-content", value: "tab3".to_string(), "Tab 3 Content" }
        }
    }
}

#[component]
fn DropdownMenuExample() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./assets/dropdown-menu.css") }
        DropdownMenu { class: "dropdown-menu", default_open: false,

            DropdownMenuTrigger { class: "dropdown-menu-trigger", "Open Menu" }

            DropdownMenuContent { class: "dropdown-menu-content",

                DropdownMenuItem {
                    class: "dropdown-menu-item",
                    value: "item1".to_string(),
                    index: 0,
                    on_select: move |value| {
                        eval(&format!("console.log('Selected: {}')", value));
                    },
                    "Item 1"
                }

                DropdownMenuItem {
                    class: "dropdown-menu-item",
                    value: "item2".to_string(),
                    index: 1,
                    on_select: move |value| {
                        eval(&format!("console.log('Selected: {}')", value));
                    },
                    "Item 2"
                }

                DropdownMenuItem {
                    class: "dropdown-menu-item",
                    value: "item3".to_string(),
                    index: 2,
                    on_select: move |value| {
                        eval(&format!("console.log('Selected: {}')", value));
                    },
                    "Item 3"
                }
            }
        }
    }
}

#[component]
fn MenubarExample() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/menubar.css") }
        div { class: "menubar-example",
            Menubar { class: "menubar",
                MenubarMenu { class: "menubar-menu", index: 0,
                    MenubarTrigger { class: "menubar-trigger", "File" }
                    MenubarContent { class: "menubar-content",
                        MenubarItem {
                            class: "menubar-item",
                            value: "new".to_string(),
                            on_select: move |value| {
                                eval(&format!("console.log('Selected: {}')", value));
                            },
                            "New"
                        }
                        MenubarItem {
                            class: "menubar-item",
                            value: "open".to_string(),
                            on_select: move |value| {
                                eval(&format!("console.log('Selected: {}')", value));
                            },
                            "Open"
                        }
                        MenubarItem {
                            class: "menubar-item",
                            value: "save".to_string(),
                            on_select: move |value| {
                                eval(&format!("console.log('Selected: {}')", value));
                            },
                            "Save"
                        }
                    }
                }

                MenubarMenu { class: "menubar-menu", index: 1,
                    MenubarTrigger { class: "menubar-trigger", "Edit" }
                    MenubarContent { class: "menubar-content",
                        MenubarItem {
                            class: "menubar-item",
                            value: "cut".to_string(),
                            on_select: move |value| {
                                eval(&format!("console.log('Selected: {}')", value));
                            },
                            "Cut"
                        }
                        MenubarItem {
                            class: "menubar-item",
                            value: "copy".to_string(),
                            on_select: move |value| {
                                eval(&format!("console.log('Selected: {}')", value));
                            },
                            "Copy"
                        }
                        MenubarItem {
                            class: "menubar-item",
                            value: "paste".to_string(),
                            on_select: move |value| {
                                eval(&format!("console.log('Selected: {}')", value));
                            },
                            "Paste"
                        }
                    }
                }
            }
        }
    }
}
