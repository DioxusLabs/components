use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {

        ToggleGroup { horizontal: true, allow_multiple_pressed: true,
            ToggleItem { index: 0usize,
                b { "B" }
            }
            ToggleItem { index: 1usize,
                i { "I" }
            }
            ToggleItem { index: 2usize,
                u { "U" }
            }
        }
    }
}
