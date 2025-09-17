use dioxus::prelude::*;

#[component]
pub fn Input(
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/input/style.css"),
        }
        input {
            class: "input",
            ..attributes,
            {children}
        }
    }
}
