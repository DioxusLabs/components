use dioxus_lib::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LabelProps {
    html_for: ReadOnlySignal<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Label(props: LabelProps) -> Element {
    // TODO: (?) the Radix primitive prevents selection on double click (but not intentional highlighting)
    rsx! {
        label {
            for: props.html_for,
            ..props.attributes,

            {props.children}
        }
    }
}
