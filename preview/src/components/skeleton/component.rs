use dioxus::prelude::*;

#[component]
pub fn Skeleton(#[props(extends=GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "skeleton", ..attributes }
    }
}
