#![allow(non_snake_case)]
use dioxus::prelude::*;

#[macro_use]
mod props;

mod accordion;
pub use accordion::*;

props!(ContainerProps { children: Element });

/// A Container is a ``div`` that can be styled. A good use of this is to apply
/// consistent margins between your components and the page border.
pub fn Container(props: ContainerProps) -> Element {
    rsx! {
        div {
            id: props.id,
            class: props.class,
            style: props.style,
            {props.children}
        }
    }
}
