use dioxus::prelude::*;

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
    rsx! {
        // style { {include_str!("../../css-out/column.css")} }
        div {
            id: if let Some(id) = props.id { "{id}" },
            class: if let Some(class) = props.class { "{class}" },
            style: if let Some(style) = props.style { "{style}" },
            style: "dxc-column",
            {props.children}
        }
    }
}
