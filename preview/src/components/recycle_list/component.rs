use dioxus::prelude::*;
pub use dioxus_primitives::recycle_list::RecycleListProps;

/// Styled wrapper around the primitive `RecycleList`.
#[component]
pub fn RecycleList(props: RecycleListProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        dioxus_primitives::recycle_list::RecycleList {
            count: props.count,
            buffer: props.buffer,
            render_item: props.render_item,
            attributes: props.attributes,
        }
    }
}
