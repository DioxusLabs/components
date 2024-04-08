#![allow(non_snake_case)]
use dioxus::prelude::*;

#[macro_use]
mod props;

mod use_unique_id;
pub(crate) use use_unique_id::*;

mod accordion;
pub use accordion::*;

mod row;
pub use row::*;

mod column;
pub use column::*;

props!(ContainerProps { children: Element });

/// A Container is a ``div`` that can be styled. A good use of this is to apply
/// consistent margins between your components and the page border.
pub fn Container(props: ContainerProps) -> Element {
    rsx! {
        div { id: props.id, class: props.class, style: props.style, {props.children} }
    }
}
