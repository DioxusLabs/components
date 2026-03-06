use dioxus::prelude::*;
use dioxus_primitives::navbar::{
    self, NavbarContentProps, NavbarItemProps, NavbarNavProps, NavbarProps, NavbarTriggerProps,
};
use dioxus_primitives::icon;

#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        navbar::Navbar {
            class: "navbar",
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarNav(props: NavbarNavProps) -> Element {
    rsx! {
        navbar::NavbarNav {
            class: "navbar-nav",
            index: props.index,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarTrigger(props: NavbarTriggerProps) -> Element {
    rsx! {
        navbar::NavbarTrigger { class: "navbar-trigger", attributes: props.attributes,
            {props.children}
            icon::Icon {
                class: "navbar-expand-icon",
                width: 20,
                height: 20,
                polyline { points: "6 9 12 15 18 9" }
            }
        }
    }
}

#[component]
pub fn NavbarContent(props: NavbarContentProps) -> Element {
    rsx! {
        navbar::NavbarContent {
            class: "navbar-content",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarItem(props: NavbarItemProps) -> Element {
    rsx! {
        navbar::NavbarItem {
            class: "navbar-item",
            index: props.index,
            value: props.value,
            disabled: props.disabled,
            new_tab: props.new_tab,
            to: props.to,
            active_class: props.active_class,
            attributes: props.attributes,
            on_select: props.on_select,
            onclick: props.onclick,
            onmounted: props.onmounted,
            {props.children}
        }
    }
}
