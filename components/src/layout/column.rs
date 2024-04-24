use dioxus::prelude::*;

use crate::theme::use_theme;

const _: &str = manganis::mg!(file("./css-out/column.css"));

props!(ColumnProps {
    #[props(into)]
    children: Element,
});

/// Creates a new `Column` element.
///
/// # Arguments
///
/// * `props` - A `ColumnProps` struct that defines the properties of the `Column`.
///
/// # Properties
///
/// * `id` - The ID of the `Column`. Default is `None`.
/// * `class` - The CSS class of the `Column`.
/// * `style` - The CSS styles of the `Column`. Default is `None`.
/// * `children` - The child elements of the `Column`.
///
/// # Example
///
/// ```
///   Column {
///      id: "hi",
///      style: "background-color: #f5f5f5; padding: 20px;",
///      class: "bg-gray-100 p-5",
///      Element
///     }
/// ```
pub fn Column(props: ColumnProps) -> Element {
    let theme = use_theme();

    rsx! {
        div {
            id: props.id,
            class: props.class,
            style: props.style,
            class: "dxc-column {theme().0}",
            {props.children}
        }
    }
}
