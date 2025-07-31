use dioxus::prelude::*;
use dioxus_primitives::select::{
    SelectValue, Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption, SelectTrigger
};
use strum::{EnumCount, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumCount, strum::EnumIter, strum::Display)]
enum Fruit {
    Apple,
    Banana,
    Orange,
    Strawberry,
    Watermelon,
}

impl Fruit {
    const fn emoji(&self) -> &'static str {
        match self {
            Fruit::Apple => "🍎",
            Fruit::Banana => "🍌",
            Fruit::Orange => "🍊",
            Fruit::Strawberry => "🍓",
            Fruit::Watermelon => "🍉",
        }
    }
}

#[component]
pub fn Demo() -> Element {
    let fruits = Fruit::iter().enumerate().map(|(i, f)| {
        rsx! {
            SelectOption::<Option<Fruit>> {
                index: i,
                class: "select-option",
                value: f,
                text_value: "{f}",
                {format!("{} {f}", f.emoji())}
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
    });

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/select/variants/main/style.css"),
        }
        Select::<Option<Fruit>> {
            class: "select",
            placeholder: "Select a fruit...",
            SelectTrigger {
                class: "select-trigger",
                aria_label: "Select Trigger",
                width: "12rem",
                SelectValue {}
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
                    {fruits}
                }
                SelectGroup {
                    class: "select-group",
                    SelectGroupLabel {
                        class: "select-group-label",
                        "Other"
                    }
                    SelectOption::<Option<Fruit>> {
                        index: Fruit::COUNT,
                        class: "select-option",
                        value: None,
                        text_value: "Other",
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
