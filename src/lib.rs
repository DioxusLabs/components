#![allow(non_snake_case)]
use dioxus::prelude::*;

#[macro_use]
mod props;


props!(AccordianProps {
    #[props(into)]
    title: Option<String>,
    children: Element,
});

pub fn Accordian(props: AccordianProps) -> Element {
    rsx! {
        div {
            if let Some(title) = props.title {
                h2 { {title} }
            }

            {props.children}
        }
    }
}

props!(AccordianItemProps {
    #[props(into)]
    title: String,

    #[props(into)]
    content: String,
});

pub fn AccordianItem(props: AccordianItemProps) -> Element {
    rsx! {
        div {
            id: props.id,
            class: props.class,

            h3 { "{props.title}" }
            p { "{props.content}" }
        }
    }
}
