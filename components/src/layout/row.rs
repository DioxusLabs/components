use dioxus::prelude::*;

const _STYLE: &str = manganis::mg!(file("./css-out/row.css"));

props!(RowProps {
    #[props(into)]
    children: Element,
});

/// Creates a new `Row` element.
///
/// # Arguments
///
/// * `props` - A `RowProps` struct that defines the properties of the `Row`.
///
/// # Properties
///
/// * `id` - The ID of the `Row`. Default is `None`.
/// * `class` - The CSS class of the `Row`.
/// * `style` - The CSS styles of the `Row`. Default is `None`.
/// * `children` - The child elements of the `Row`.
///
/// # Example
///
/// ```
///   Row {
///      id: "hi",
///      style: "background-color: #f5f5f5; padding: 20px;",
///      class: "bg-gray-100 p-5",
///      Element
///     }
/// ```
pub fn Row(props: RowProps) -> Element {
    rsx! {
        div {
            id: props.id,
            class: props.class,
            style: props.style,
            class: "dxc-row",
            {props.children}
        }
    }
}
