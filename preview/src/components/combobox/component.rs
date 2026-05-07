use dioxus::prelude::*;
use dioxus_primitives::combobox::{
    self, ComboboxEmptyProps, ComboboxInputProps, ComboboxListProps, ComboboxOptionProps,
    ComboboxProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, icon, merge_attributes};

#[css_module("/src/components/combobox/style.css")]
struct Styles;

#[component]
pub fn Combobox<T: Clone + PartialEq + 'static>(props: ComboboxProps<T>) -> Element {
    let base = attributes!(div { class: Styles::dx_combobox });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::Combobox {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            query: props.query,
            default_query: props.default_query,
            on_query_change: props.on_query_change,
            roving_loop: props.roving_loop,
            filter: props.filter,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn ComboboxInput(props: ComboboxInputProps) -> Element {
    rsx! {
        div { class: Styles::dx_combobox_input_wrapper,
            combobox::ComboboxInput {
                class: Styles::dx_combobox_input,
                placeholder: props.placeholder,
                id: props.id,
                attributes: props.attributes,
            }
            icon::Icon {
                class: Styles::dx_combobox_expand_icon,
                width: "16px",
                height: "16px",
                path { d: "m7 15 5 5 5-5" }
                path { d: "m7 9 5-5 5 5" }
            }
        }
    }
}

#[component]
pub fn ComboboxList(props: ComboboxListProps) -> Element {
    let base = attributes!(div { class: Styles::dx_combobox_list });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::ComboboxList {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn ComboboxEmpty(props: ComboboxEmptyProps) -> Element {
    let base = attributes!(div { class: Styles::dx_combobox_empty });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::ComboboxEmpty {
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn ComboboxOption<T: Clone + PartialEq + 'static>(props: ComboboxOptionProps<T>) -> Element {
    let base = attributes!(div { class: Styles::dx_combobox_option });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::ComboboxOption::<T> {
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            id: props.id,
            index: props.index,
            aria_label: props.aria_label,
            aria_roledescription: props.aria_roledescription,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn ComboboxItemIndicator() -> Element {
    rsx! {
        combobox::ComboboxItemIndicator {
            icon::Icon {
                class: Styles::dx_combobox_check_icon,
                width: "16px",
                height: "16px",
                path { d: "M20 6 9 17l-5-5" }
            }
        }
    }
}
