use dioxus::prelude::*;
use dioxus_primitives::combobox::{
    self, ComboboxContentProps, ComboboxEmptyProps, ComboboxGroupLabelProps, ComboboxGroupProps,
    ComboboxInputProps, ComboboxListProps, ComboboxMultiProps, ComboboxOptionProps, ComboboxProps,
};
use dioxus_primitives::icon;

#[component]
pub fn Combobox<T: Clone + PartialEq + 'static>(props: ComboboxProps<T>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        combobox::Combobox {
            class: "dx-combobox",
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            name: props.name,
            roving_loop: props.roving_loop,
            filter: props.filter,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ComboboxMulti<T: Clone + PartialEq + 'static>(props: ComboboxMultiProps<T>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        combobox::ComboboxMulti {
            class: "dx-combobox",
            values: props.values,
            default_values: props.default_values,
            on_values_change: props.on_values_change,
            disabled: props.disabled,
            name: props.name,
            roving_loop: props.roving_loop,
            filter: props.filter,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ComboboxInput(props: ComboboxInputProps) -> Element {
    rsx! {
        div { class: "dx-combobox-input-wrapper",
            combobox::ComboboxInput {
                class: "dx-combobox-input",
                placeholder: props.placeholder,
                id: props.id,
                attributes: props.attributes,
            }
            icon::Icon {
                class: "dx-combobox-expand-icon",
                width: "16px",
                height: "16px",
                path { d: "m7 15 5 5 5-5" }
                path { d: "m7 9 5-5 5 5" }
            }
        }
    }
}

#[component]
pub fn ComboboxContent(props: ComboboxContentProps) -> Element {
    rsx! {
        combobox::ComboboxContent {
            class: "dx-combobox-content",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ComboboxList(props: ComboboxListProps) -> Element {
    rsx! {
        combobox::ComboboxList {
            class: "dx-combobox-list",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ComboboxEmpty(props: ComboboxEmptyProps) -> Element {
    rsx! {
        combobox::ComboboxEmpty { class: "dx-combobox-empty", attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ComboboxGroup(props: ComboboxGroupProps) -> Element {
    rsx! {
        combobox::ComboboxGroup {
            class: "dx-combobox-group",
            disabled: props.disabled,
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ComboboxGroupLabel(props: ComboboxGroupLabelProps) -> Element {
    rsx! {
        combobox::ComboboxGroupLabel {
            class: "dx-combobox-group-label",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ComboboxOption<T: Clone + PartialEq + 'static>(props: ComboboxOptionProps<T>) -> Element {
    rsx! {
        combobox::ComboboxOption::<T> {
            class: "dx-combobox-option",
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            id: props.id,
            index: props.index,
            aria_label: props.aria_label,
            aria_roledescription: props.aria_roledescription,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ComboboxItemIndicator() -> Element {
    // Always render the check; the option's data-selected attribute drives
    // opacity in CSS so unselected rows reserve the same width (matching
    // shadcn's `Check className={cn("ml-auto", value === ... ? "opacity-100" : "opacity-0")}`).
    rsx! {
        icon::Icon {
            class: "dx-combobox-check-icon",
            width: "16px",
            height: "16px",
            path { d: "M20 6 9 17l-5-5" }
        }
    }
}
