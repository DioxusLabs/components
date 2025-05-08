use dioxus::{document::eval, prelude::*};
use dioxus_primitives::{
    accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger},
    aspect_ratio::AspectRatio,
    avatar::{Avatar, AvatarFallback, AvatarImage},
    calendar::{Calendar, CalendarDate, CalendarGrid, CalendarHeader, CalendarNavigation},
    checkbox::{Checkbox, CheckboxIndicator},
    collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
    context_menu::{ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger},
    dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger},
    hover_card::{HoverCard, HoverCardAlign, HoverCardContent, HoverCardSide, HoverCardTrigger},
    menubar::{Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger},
    progress::{Progress, ProgressIndicator},
    radio_group::{RadioGroup, RadioItem},
    scroll_area::{ScrollArea, ScrollDirection},
    select::{Select, SelectGroup, SelectOption},
    separator::Separator,
    slider::{Slider, SliderRange, SliderThumb, SliderTrack, SliderValue},
    switch::{Switch, SwitchThumb},
    tabs::{TabContent, TabTrigger, Tabs},
    toast::{ToastOptions, ToastProvider, use_toast},
    toggle_group::{ToggleGroup, ToggleItem},
    toolbar::{Toolbar, ToolbarButton, ToolbarSeparator},
    tooltip::{Tooltip, TooltipContent, TooltipSide, TooltipTrigger},
};
use std::time::Duration;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        ToastProvider {
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

            document::Link { rel: "stylesheet", href: asset!("/assets/toast.css") }
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

        document::Link { rel: "stylesheet", href: asset!("/assets/scroll-area.css") }
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

        document::Link { rel: "stylesheet", href: asset!("./assets/context-menu.css") }
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

        document::Link { rel: "stylesheet", href: asset!("./assets/tooltip.css") }
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

        document::Link { rel: "stylesheet", href: asset!("./assets/select.css") }
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

        document::Link { rel: "stylesheet", href: asset!("./assets/calendar.css") }
        Collapsible {
            CollapsibleTrigger { "Calendar Example" }
            CollapsibleContent { CalendarExample {} }
        }
    }
}

#[component]
fn TooltipExample() -> Element {
    rsx! {
        div {
            class: "tooltip-example",
            style: "padding: 50px; display: flex; gap: 20px;",
            // Basic tooltip
            Tooltip { class: "tooltip",
                TooltipTrigger { class: "tooltip-trigger",
                    button { "Hover me" }
                }
                TooltipContent { class: "tooltip-content", "This is a basic tooltip" }
            }
            // Tooltip with different position
            Tooltip { class: "tooltip",
                TooltipTrigger { class: "tooltip-trigger",
                    button { "Right tooltip" }
                }
                TooltipContent { class: "tooltip-content", side: TooltipSide::Right,
                    "This tooltip appears on the right"
                }
            }
            // Tooltip with HTML content
            Tooltip { class: "tooltip",
                TooltipTrigger { class: "tooltip-trigger",
                    button { "Rich content" }
                }
                TooltipContent { class: "tooltip-content", style: "width: 200px;",
                    h4 { style: "margin-top: 0; margin-bottom: 8px;", "Tooltip title" }
                    p { style: "margin: 0;", "This tooltip contains rich HTML content with styling." }
                }
            }
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
fn ToastExample() -> Element {
    // Get the toast API
    let toast_api = use_toast();

    rsx! {
        div { class: "toast-example",
            h3 { "Toast Notifications" }
            p { "Click the buttons below to show different types of toast notifications." }

            h4 { "Timed Toasts (auto-dismiss)" }
            div { class: "toast-buttons",
                button {
                    onclick: move |_| {
                        toast_api
                            .success(
                                "Success".to_string(),
                                Some(ToastOptions {
                                    duration: Some(Duration::from_secs(3)),
                                    ..Default::default()
                                }),
                            );
                    },
                    "Success (3s)"
                }

                button {
                    onclick: move |_| {
                        toast_api
                            .error(
                                "Error".to_string(),
                                Some(ToastOptions {
                                    duration: Some(Duration::from_secs(5)),
                                    ..Default::default()
                                }),
                            );
                    },
                    "Error (5s)"
                }

                button {
                    onclick: move |_| {
                        toast_api
                            .warning(
                                "Warning".to_string(),
                                Some(ToastOptions {
                                    description: Some("This action might cause issues".to_string()),
                                    duration: Some(Duration::from_secs(3)),
                                    ..Default::default()
                                }),
                            );
                    },
                    "Warning (3s)"
                }
            }

            h4 { "Permanent Toasts (manual close)" }
            div { class: "toast-buttons",
                button {
                    onclick: move |_| {
                        toast_api
                            .success(
                                "Important".to_string(),
                                Some(ToastOptions {
                                    permanent: true,
                                    ..Default::default()
                                }),
                            );
                    },
                    "Permanent Success"
                }

                button {
                    onclick: move |_| {
                        toast_api
                            .error(
                                "Critical Error".to_string(),
                                Some(ToastOptions {
                                    permanent: true,
                                    ..Default::default()
                                }),
                            );
                    },
                    "Permanent Error"
                }

                button {
                    onclick: move |_| {
                        toast_api
                            .info(
                                "Custom Toast".to_string(),
                                Some(ToastOptions {
                                    description: Some(
                                        "This is a custom toast with specific settings".to_string(),
                                    ),
                                    duration: Some(Duration::from_secs(10)),
                                    permanent: false,
                                }),
                            );
                    },
                    "Custom Info (10s)"
                }
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
    let mut avatar_state = use_signal(|| "No state yet".to_string());

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./assets/avatar.css") }

        // Basic examples section
        div { class: "avatar-example-section",
            h4 { "Basic Examples" }
            div { class: "avatar-example",
                // Basic Avatar with image and fallback
                div { class: "avatar-item",
                    p { class: "avatar-label", "Basic Usage" }
                    Avatar {
                        class: "avatar",
                        on_state_change: move |state| {
                            avatar_state.set(format!("Avatar 1: {:?}", state));
                        },

                        AvatarImage {
                            src: "https://github.com/DioxusLabs.png",
                            alt: "User avatar",
                        }

                        AvatarFallback { class: "avatar-fallback", "UA" }
                    }
                }

                // Avatar with error state (fallback shown)
                div { class: "avatar-item",
                    p { class: "avatar-label", "Error State" }
                    Avatar {
                        class: "avatar",
                        on_state_change: move |state| {
                            avatar_state.set(format!("Avatar 2: {:?}", state));
                        },

                        AvatarImage {
                            src: "https://invalid-url.example/image.jpg",
                            alt: "Invalid image",
                        }

                        AvatarFallback { class: "avatar-fallback", "JD" }
                    }
                }

                // Avatar with emoji fallback
                div { class: "avatar-item",
                    p { class: "avatar-label", "Emoji Fallback" }
                    Avatar {
                        class: "avatar",
                        on_state_change: move |state| {
                            avatar_state.set(format!("Avatar 3: {:?}", state));
                        },

                        AvatarImage {
                            src: "https://invalid-url.example/image.jpg",
                            alt: "Invalid image",
                        }

                        AvatarFallback { class: "avatar-fallback", "👤" }
                    }
                }

                // Avatar with different size
                div { class: "avatar-item",
                    p { class: "avatar-label", "Large Size" }
                    Avatar {
                        class: "avatar avatar-lg",
                        on_state_change: move |state| {
                            avatar_state.set(format!("Avatar 4: {:?}", state));
                        },

                        AvatarImage {
                            src: "https://github.com/DioxusLabs.png",
                            alt: "Large avatar",
                        }

                        AvatarFallback { class: "avatar-fallback", "LG" }
                    }
                }
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

            RadioItem {
                class: "radio-item",
                value: ReadOnlySignal::new(Signal::new("option1".to_string())),
                index: ReadOnlySignal::new(Signal::new(0)),
                "Option 1"
            }
            RadioItem {
                class: "radio-item",
                value: ReadOnlySignal::new(Signal::new("option2".to_string())),
                index: ReadOnlySignal::new(Signal::new(1)),
                "Option 2"
            }
            RadioItem {
                class: "radio-item",
                value: ReadOnlySignal::new(Signal::new("option3".to_string())),
                index: ReadOnlySignal::new(Signal::new(2)),
                "Option 3"
            }
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
                    index: ReadOnlySignal::new(Signal::new(0)),
                    "Tab 1"
                }
                TabTrigger {
                    class: "tabs-trigger",
                    value: "tab2".to_string(),
                    index: ReadOnlySignal::new(Signal::new(1)),
                    "Tab 2"
                }
                TabTrigger {
                    class: "tabs-trigger",
                    value: "tab3".to_string(),
                    index: ReadOnlySignal::new(Signal::new(2)),
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
                    value: ReadOnlySignal::new(Signal::new("item1".to_string())),
                    index: ReadOnlySignal::new(Signal::new(0)),
                    on_select: move |value| {
                        eval(&format!("console.log('Selected: {}')", value));
                    },
                    "Item 1"
                }

                DropdownMenuItem {
                    class: "dropdown-menu-item",
                    value: ReadOnlySignal::new(Signal::new("item2".to_string())),
                    index: ReadOnlySignal::new(Signal::new(1)),
                    on_select: move |value| {
                        eval(&format!("console.log('Selected: {}')", value));
                    },
                    "Item 2"
                }

                DropdownMenuItem {
                    class: "dropdown-menu-item",
                    value: ReadOnlySignal::new(Signal::new("item3".to_string())),
                    index: ReadOnlySignal::new(Signal::new(2)),
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

#[component]
fn ScrollAreaExample() -> Element {
    rsx! {
        div { class: "scroll-area-demo",
            // Vertical scroll example
            div { class: "scroll-demo-section",
                h3 { "Vertical Scroll" }
                ScrollArea {
                    class: "demo-scroll-area",
                    direction: ScrollDirection::Vertical,

                    div { class: "scroll-content",
                        for i in 1..=20 {
                            p { "Scrollable content item {i}" }
                        }
                    }
                }
            }

            // Horizontal scroll example
            div { class: "scroll-demo-section",
                h3 { "Horizontal Scroll" }
                ScrollArea {
                    class: "demo-scroll-area",
                    direction: ScrollDirection::Horizontal,

                    div { class: "scroll-content-horizontal",
                        for i in 1..=20 {
                            span { "Column {i} " }
                        }
                    }
                }
            }

            // Both directions example
            div { class: "scroll-demo-section",
                h3 { "Both Directions" }
                ScrollArea {
                    class: "demo-scroll-area",
                    direction: ScrollDirection::Both,

                    div { class: "scroll-content-both",
                        for i in 1..=20 {
                            div {
                                for j in 1..=20 {
                                    span { "Cell {i},{j} " }
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
fn ContextMenuExample() -> Element {
    let mut selected_value = use_signal(String::new);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./assets/context-menu.css") }
        div { class: "context-menu-example",
            ContextMenu {
                ContextMenuTrigger { class: "context-menu-trigger", "Right click here to open context menu" }

                ContextMenuContent { class: "context-menu-content",
                    ContextMenuItem {
                        class: "context-menu-item",
                        value: ReadOnlySignal::new(Signal::new("edit".to_string())),
                        index: ReadOnlySignal::new(Signal::new(0)),
                        on_select: move |value| {
                            selected_value.set(value);
                        },
                        "Edit"
                    }

                    ContextMenuItem {
                        class: "context-menu-item",
                        value: ReadOnlySignal::new(Signal::new("duplicate".to_string())),
                        index: ReadOnlySignal::new(Signal::new(1)),
                        on_select: move |value| {
                            selected_value.set(value);
                        },
                        "Duplicate"
                    }

                    ContextMenuItem {
                        class: "context-menu-item",
                        value: ReadOnlySignal::new(Signal::new("delete".to_string())),
                        index: ReadOnlySignal::new(Signal::new(2)),
                        on_select: move |value| {
                            selected_value.set(value);
                        },
                        "Delete"
                    }
                }
            }

            div { class: "selected-value",
                if selected_value().is_empty() {
                    "No action selected"
                } else {
                    "Selected action: {selected_value()}"
                }
            }
        }
    }
}

#[component]
fn ToolbarExample() -> Element {
    let mut text_style = use_signal(Vec::new);
    let mut text_align = use_signal(|| String::from("left"));

    let mut toggle_style = move |style: &str| {
        let mut current_styles = text_style();
        if current_styles.contains(&style.to_string()) {
            current_styles.retain(|s| s != style);
        } else {
            current_styles.push(style.to_string());
        }
        text_style.set(current_styles);
    };

    let mut set_align = move |align: &str| {
        text_align.set(align.to_string());
    };

    let text_classes = use_memo(move || {
        let mut classes = Vec::new();
        for style in text_style() {
            match style.as_str() {
                "bold" => classes.push("toolbar-bold"),
                "italic" => classes.push("toolbar-italic"),
                "underline" => classes.push("toolbar-underline"),
                _ => {}
            }
        }
        classes.join(" ")
    });

    let text_align_style = use_memo(move || format!("text-align: {};", text_align()));

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/toolbar.css") }

        div { class: "toolbar-example",
            h3 { "Text Formatting Toolbar" }

            Toolbar { class: "toolbar", aria_label: "Text formatting",

                ToolbarButton {
                    class: "toolbar-button",
                    index: ReadOnlySignal::new(Signal::new(0)),
                    on_click: move |_| toggle_style("bold"),
                    "Bold"
                }

                ToolbarButton {
                    class: "toolbar-button",
                    index: ReadOnlySignal::new(Signal::new(1)),
                    on_click: move |_| toggle_style("italic"),
                    "Italic"
                }

                ToolbarButton {
                    class: "toolbar-button",
                    index: ReadOnlySignal::new(Signal::new(2)),
                    on_click: move |_| toggle_style("underline"),
                    "Underline"
                }

                ToolbarSeparator { class: "toolbar-separator" }

                ToolbarButton {
                    class: "toolbar-button",
                    index: ReadOnlySignal::new(Signal::new(3)),
                    on_click: move |_| set_align("left"),
                    "Align Left"
                }

                ToolbarButton {
                    class: "toolbar-button",
                    index: ReadOnlySignal::new(Signal::new(4)),
                    on_click: move |_| set_align("center"),
                    "Align Center"
                }

                ToolbarButton {
                    class: "toolbar-button",
                    index: ReadOnlySignal::new(Signal::new(5)),
                    on_click: move |_| set_align("right"),
                    "Align Right"
                }
            }

            div { class: "toolbar-content",
                p { class: text_classes, style: text_align_style,
                    "This is a sample text that will be formatted according to the toolbar buttons you click. Try clicking the buttons above to see how the text formatting changes."
                }
            }
        }
    }
}

#[component]
fn HoverCardExample() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/hover-card.css") }

        div {
            class: "hover-card-example",
            style: "padding: 50px; display: flex; gap: 40px;",
            // User profile hover card
            HoverCard { class: "hover-card",
                HoverCardTrigger { class: "hover-card-trigger",
                    button { class: "user-trigger", "@johndoe" }
                }

                HoverCardContent { class: "hover-card-content", side: HoverCardSide::Bottom,
                    div { class: "user-card",
                        div { class: "user-card-header",
                            img {
                                class: "user-card-avatar",
                                src: "https://github.com/DioxusLabs.png",
                                alt: "User avatar",
                            }
                            div {
                                h4 { class: "user-card-name", "John Doe" }
                                p { class: "user-card-username", "@johndoe" }
                            }
                        }

                        p { class: "user-card-bio",
                            "Software developer passionate about Rust and web technologies. Building awesome UI components with Dioxus."
                        }

                        div { class: "user-card-stats",
                            div { class: "user-card-stat",
                                span { class: "user-card-stat-value", "142" }
                                span { class: "user-card-stat-label", "Posts" }
                            }
                            div { class: "user-card-stat",
                                span { class: "user-card-stat-value", "2.5k" }
                                span { class: "user-card-stat-label", "Followers" }
                            }
                            div { class: "user-card-stat",
                                span { class: "user-card-stat-value", "350" }
                                span { class: "user-card-stat-label", "Following" }
                            }
                        }
                    }
                }
            }

            // Product hover card
            HoverCard { class: "hover-card",
                HoverCardTrigger { class: "hover-card-trigger",
                    button { class: "product-trigger", "View Product" }
                }

                HoverCardContent {
                    class: "hover-card-content",
                    side: HoverCardSide::Right,
                    align: HoverCardAlign::Start,
                    div { class: "product-card",
                        img {
                            class: "product-card-image",
                            src: "https://images.unsplash.com/photo-1505740420928-5e560c06d30e",
                            alt: "Product image",
                        }
                        h4 { class: "product-card-title", "Wireless Headphones" }
                        p { class: "product-card-price", "$129.99" }
                        p { class: "product-card-description",
                            "High-quality wireless headphones with noise cancellation and 30-hour battery life."
                        }
                        div { class: "product-card-rating", "★★★★☆ (4.5)" }
                    }
                }
            }

            // Link hover card
            HoverCard { class: "hover-card",
                HoverCardTrigger { class: "hover-card-trigger",
                    a { href: "#", "Hover over this link" }
                }

                HoverCardContent {
                    class: "hover-card-content",
                    side: HoverCardSide::Top,
                    align: HoverCardAlign::Center,
                    div { style: "padding: 8px;",
                        p { style: "margin: 0;", "This link will take you to an external website." }
                    }
                }
            }
        }
    }
}

#[component]
fn SelectExample() -> Element {
    let mut selected = use_signal(|| None::<String>);

    // Debug output for selected value
    use_effect(move || {
        if let Some(value) = selected() {
            println!("Selected value: {}", value);
        }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./assets/select.css") }
        div { class: "select-example",
            h3 { "Select Example" }

            // Basic select
            div { class: "select-container",
                // Label for the select
                label { class: "select-label", "Choose a fruit:" }

                // Native select element
                Select {
                    class: "select",
                    value: selected,
                    on_value_change: move |value| selected.set(value),
                    placeholder: "Select a fruit...",

                    // Fruits group
                    SelectGroup { label: "Fruits".to_string(),

                        SelectOption { value: "apple".to_string(), "Apple" }
                        SelectOption { value: "banana".to_string(), "Banana" }
                        SelectOption { value: "orange".to_string(), "Orange" }
                        SelectOption { value: "strawberry".to_string(), "Strawberry" }
                        SelectOption { value: "watermelon".to_string(), "Watermelon" }
                    }

                    // Other options group
                    SelectGroup { label: "Other".to_string(),

                        SelectOption { value: "other".to_string(), "Other" }
                    }
                }
            }

            // Display selected value
            div { class: "selected-value",
                if let Some(value) = selected() {
                    "Selected: {value}"
                } else {
                    "No selection"
                }
            }
        }
    }
}

#[component]
fn CalendarExample() -> Element {
    let mut selected_date = use_signal(|| None::<CalendarDate>);
    let mut view_date = use_signal(|| CalendarDate::new(2024, 5, 15));

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/calendar.css") }

        div { class: "calendar-example", style: "padding: 20px;",
            // Basic calendar
            div { class: "calendar",
                Calendar {
                    selected_date: selected_date(),
                    on_date_change: move |date| {
                        println!("Selected date: {:?}", date);
                        selected_date.set(date);
                    },
                    view_date: view_date(),
                    on_view_change: move |new_view: CalendarDate| {
                        println!("View changed to: {}-{}", new_view.year, new_view.month);
                        view_date.set(new_view);
                    },

                    CalendarHeader { CalendarNavigation {} }

                    CalendarGrid {}
                }
            }

            // Display selected date
            div { class: "selected-date", style: "margin-top: 20px;",
                if let Some(date) = selected_date() {
                    p { style: "font-weight: bold;", "Selected date: {date}" }
                } else {
                    p { style: "color: #666;", "No date selected" }
                }
            }
        }
    }
}
