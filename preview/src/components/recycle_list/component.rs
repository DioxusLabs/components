use dioxus::prelude::*;
pub use dioxus_primitives::recycle_list::RecycleListProps;

/// Styled wrapper around the primitive `RecycleList`.
#[allow(non_snake_case)]
pub fn RecycleList<T: PartialEq + 'static, F>(props: RecycleListProps<'_, T, F>) -> Element
where
    F: Fn(&T, usize) -> Element,
{
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        {dioxus_primitives::recycle_list::RecycleList(props)}
    }
}
