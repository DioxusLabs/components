use dioxus::prelude::*;

use crate::theme::use_theme;

props!(ContainerProps { children: Element });

/// A Container is a ``div`` that can be styled. A good use of this is to apply
/// consistent margins between your components and the page border.
pub fn Container(props: ContainerProps) -> Element {
    let theme = use_theme();

    rsx! {
        div { 
            id: props.id, 
            class: props.class, 
            style: props.style, 
            class: "dxc-container {theme().0}",
            {props.children} 
        }
    }
}
