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
            gap: "0.75rem",

            Button {
                variant: ButtonVariant::Outline,
                size: ButtonSize::Icon,
                Icon {
                    width: "16px",
                    height: "16px",
                    path { d: "M7 17 17 7" }
                    path { d: "M7 7h10v10" }
                }
            }

            Button {
                variant: ButtonVariant::Outline,
                size: ButtonSize::Icon,
                border_radius: "50%",
                Icon {
                    width: "16px",
                    height: "16px",
                    path { d: "M7 17 17 7" }
                    path { d: "M7 7h10v10" }
                }
            }

            Button {
                variant: ButtonVariant::Outline,
                size: ButtonSize::Sm,
                Icon {
                    width: "16px",
                    height: "16px",
                    circle { cx: "18", cy: "18", r: "3" }
                    circle { cx: "6", cy: "6", r: "3" }
                    path { d: "M6 21V9a9 9 0 0 0 9 9" }
                }
                "Merge"
            }
        }
    }
}
