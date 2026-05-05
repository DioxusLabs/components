use super::super::component::*;
use dioxus::prelude::*;
use dioxus_primitives::icon::Icon;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            flex_wrap: "wrap",
            align_items: "flex-start",
            gap: "2rem",

            div {
                display: "flex", align_items: "flex-start", gap: "0.5rem",
                Button { size: ButtonSize::Xs, variant: ButtonVariant::Outline, "Extra Small" }
                Button {
                    size: ButtonSize::IconXs,
                    variant: ButtonVariant::Outline,
                    aria_label: "Submit",
                    ArrowUpRightIcon { width: "12px", height: "12px" }
                }
            }

            div {
                display: "flex", align_items: "flex-start", gap: "0.5rem",
                Button { size: ButtonSize::Sm, variant: ButtonVariant::Outline, "Small" }
                Button {
                    size: ButtonSize::IconSm,
                    variant: ButtonVariant::Outline,
                    aria_label: "Submit",
                    ArrowUpRightIcon { width: "14px", height: "14px" }
                }
            }

            div {
                display: "flex", align_items: "flex-start", gap: "0.5rem",
                Button { variant: ButtonVariant::Outline, "Default" }
                Button {
                    size: ButtonSize::Icon,
                    variant: ButtonVariant::Outline,
                    aria_label: "Submit",
                    ArrowUpRightIcon { width: "16px", height: "16px" }
                }
            }

            div {
                display: "flex", align_items: "flex-start", gap: "0.5rem",
                Button { size: ButtonSize::Lg, variant: ButtonVariant::Outline, "Large" }
                Button {
                    size: ButtonSize::IconLg,
                    variant: ButtonVariant::Outline,
                    aria_label: "Submit",
                    ArrowUpRightIcon { width: "18px", height: "18px" }
                }
            }
        }
    }
}

#[component]
fn ArrowUpRightIcon(#[props(extends = GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    rsx! {
        Icon {
            path { d: "M7 17 17 7" }
            path { d: "M7 7h10v10" }
        }
    }
}
