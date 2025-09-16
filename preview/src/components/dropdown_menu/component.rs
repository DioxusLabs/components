use dioxus::prelude::*;
use dioxus_primitives::dropdown_menu::{
    self, DropdownMenuContentProps, DropdownMenuItemProps, DropdownMenuProps,
    DropdownMenuTriggerProps,
};

#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        dropdown_menu::DropdownMenu {
            class: "dropdown-menu",
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn DropdownMenuTrigger(props: DropdownMenuTriggerProps) -> Element {
    rsx! {
        dropdown_menu::DropdownMenuTrigger { class: "dropdown-menu-trigger", attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn DropdownMenuContent(props: DropdownMenuContentProps) -> Element {
    rsx! {
        dropdown_menu::DropdownMenuContent {
            class: "dropdown-menu-content",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn DropdownMenuItem<T: Clone + PartialEq + 'static>(
    props: DropdownMenuItemProps<T>,
) -> Element {
    rsx! {
        dropdown_menu::DropdownMenuItem {
            class: "dropdown-menu-item",
            disabled: props.disabled,
            value: props.value,
            index: props.index,
            on_select: props.on_select,
            attributes: props.attributes,
            {props.children}
        }
    }
}
