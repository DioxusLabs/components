use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; align-items: center; gap: 1rem; min-width: 16rem;",
            Skeleton { style: "width: 3rem; height: 3rem; border-radius: 50%; flex-shrink: 0;" }
            div { style: "display: flex; flex-direction: column; gap: 0.5rem; flex: 1;",
                Skeleton { style: "width: 100%; height: 0.85rem;" }
                Skeleton { style: "width: 80%; height: 0.85rem;" }
                Skeleton { style: "width: 60%; height: 0.85rem;" }
            }
        }
    }
}
