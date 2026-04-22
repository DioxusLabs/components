use dioxus::prelude::*;
use dioxus_primitives::select::{
    self, SelectGroupLabelProps, SelectGroupProps, SelectListProps, SelectOptionProps, SelectProps,
    SelectTriggerProps, SelectValueProps,
};
use dioxus_primitives::icon;

#[component]
pub fn Select<T: Clone + PartialEq + 'static>(props: SelectProps<T>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        select::Select {
            class: "dx-select",
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            name: props.name,
            placeholder: props.placeholder,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SelectTrigger(props: SelectTriggerProps) -> Element {
    rsx! {
        select::SelectTrigger { class: "dx-select-trigger", attributes: props.attributes,
            {props.children}
            icon::Icon {
                class: "dx-select-expand-icon",
                width: "20px",
                height: "20px",
                stroke: "var(--primary-color-7)",
                polyline { points: "6 9 12 15 18 9" }
            }
        }
    }
}

#[component]
pub fn SelectValue(props: SelectValueProps) -> Element {
    rsx! {
        select::SelectValue { attributes: props.attributes }
    }
}

#[component]
pub fn SelectList(props: SelectListProps) -> Element {
    rsx! {
        select::SelectList {
            class: "dx-select-list",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SelectGroup(props: SelectGroupProps) -> Element {
    rsx! {
        select::SelectGroup {
            class: "dx-select-group",
            disabled: props.disabled,
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SelectGroupLabel(props: SelectGroupLabelProps) -> Element {
    rsx! {
        select::SelectGroupLabel {
            class: "dx-select-group-label",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SelectOption<T: Clone + PartialEq + 'static>(props: SelectOptionProps<T>) -> Element {
    rsx! {
        select::SelectOption::<T> {
            class: "dx-select-option",
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
pub fn SelectItemIndicator() -> Element {
    rsx! {
        select::SelectItemIndicator {
            icon::Icon {
                class: "dx-select-check-icon",
                width: "1rem",
                height: "1rem",
                stroke: "var(--secondary-color-5)",
                path { d: "M5 13l4 4L19 7" }
            }
        }
    }
}
