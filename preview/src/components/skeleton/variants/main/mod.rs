use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; align-items: center; gap: 2rem;",
            SkeletonInfoDemo {}
            SkeletonCardDemo {}
        }
    }
}

#[component]
fn SkeletonInfoDemo() -> Element {
    rsx! {
        div { style: "display: flex; align-items: center; gap: 1rem;",
            Skeleton { style: "width: 3rem; height: 3rem; border-radius: 50%;" }
            div { style: "display: flex; flex-direction: column; gap: 0.5rem;",
                Skeleton { style: "width: 11.625rem; height: 1rem;" }
                Skeleton { style: "width: 8.5rem; height: 1rem;" }
            }
        }
    }
}

#[component]
fn SkeletonCardDemo() -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 0.75rem;",
            Skeleton { style: "width: 15rem; height: 8rem; border-radius: 0.75rem;" }
            div { style: "display: flex; flex-direction: column; gap: 0.5rem;",
                Skeleton { style: "width: 15.625rem; height: 1rem;" }
                Skeleton { style: "width: 12.5rem; height: 1rem;" }
            }
        }
    }
}
