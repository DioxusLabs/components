use dioxus::prelude::*;

#[css_module("/src/components/skeleton/style.css")]
struct Styles;

#[component]
pub fn Skeleton(#[props(extends=GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    rsx! {
        div { class: Styles::dx_skeleton, ..attributes }
    }
}
