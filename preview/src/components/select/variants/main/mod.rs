use dioxus::prelude::*;
use dioxus_primitives::select::{
    Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
    SelectTrigger,
};
#[component]
pub fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/select/variants/main/style.css"),
        }
        Select {
            class: "select",
            placeholder: "Select a fruit...",
            SelectTrigger {
                class: "select-trigger",
                aria_label: "Select Trigger",
                width: "12rem",
                svg {
                    class: "select-expand-icon",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    polyline { points: "6 9 12 15 18 9" }
                }
            }
            SelectList {
                class: "select-list",
                aria_label: "Select Demo",
                SelectGroup {
                    class: "select-group",
                    SelectGroupLabel {
                        class: "select-group-label",
                        "Fruits"
                    }
                    SelectOption {
                        index: 0usize,
                        class: "select-option",
                        value: "apple".to_string(),
                        "Apple"
                        SelectItemIndicator {
                            svg {
                                class: "select-check-icon",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M5 13l4 4L19 7" }
                            }
                        }
                    }
                    SelectOption {
                        index: 1usize,
                        class: "select-option",
                        value: "banana".to_string(),
                        "Banana"
                        SelectItemIndicator {
                            svg {
                                class: "select-check-icon",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M5 13l4 4L19 7" }
                            }
                        }
                    }
                    SelectOption {
                        index: 2usize,
                        class: "select-option",
                        value: "orange".to_string(),
                        "Orange"
                        SelectItemIndicator {
                            svg {
                                class: "select-check-icon",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M5 13l4 4L19 7" }
                            }
                        }
                    }
                    SelectOption {
                        index: 3usize,
                        class: "select-option",
                        value: "strawberry".to_string(),
                        "Strawberry"
                        SelectItemIndicator {
                            svg {
                                class: "select-check-icon",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M5 13l4 4L19 7" }
                            }
                        }
                    }
                    SelectOption {
                        index: 4usize,
                        class: "select-option",
                        value: "watermelon".to_string(),
                        "Watermelon"
                        SelectItemIndicator {
                            svg {
                                class: "select-check-icon",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M5 13l4 4L19 7" }
                            }
                        }
                    }
                }
                SelectGroup {
                    class: "select-group",
                    SelectGroupLabel {
                        class: "select-group-label",
                        "Other"
                    }
                    SelectOption {
                        index: 5usize,
                        class: "select-option",
                        value: "other".to_string(),
                        "Other"
                        SelectItemIndicator {
                            svg {
                                class: "select-check-icon",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M5 13l4 4L19 7" }
                            }
                        }
                    }
                }
            }
        }
    }
}
