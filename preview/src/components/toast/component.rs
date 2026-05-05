use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use dioxus_primitives::toast::{self, Toast, ToastProps};
use std::time::Duration;

#[css_module("/src/components/toast/style.css")]
struct Styles;

#[component]
fn StyledToast(props: ToastProps) -> Element {
    rsx! {
        Toast {
            id: props.id,
            index: props.index,
            title: props.title,
            description: props.description,
            toast_type: props.toast_type,
            on_close: props.on_close,
            permanent: props.permanent,
            duration: props.duration,
            class: Styles::dx_toast,
            content_class: Some(Styles::dx_toast_content.to_string()),
            title_class: Some(Styles::dx_toast_title.to_string()),
            description_class: Some(Styles::dx_toast_description.to_string()),
            close_class: Some(Styles::dx_toast_close.to_string()),
            attributes: props.attributes,
        }
    }
}

#[component]
pub fn ToastProvider(
    #[props(default = ReadSignal::new(Signal::new(Some(Duration::from_secs(5)))))]
    default_duration: ReadSignal<Option<Duration>>,
    #[props(default = ReadSignal::new(Signal::new(10)))] max_toasts: ReadSignal<usize>,
    #[props(default)] render_toast: Option<Callback<toast::ToastPropsWithOwner, Element>>,
    #[props(default)] list_class: Option<String>,
    #[props(default)] item_class: Option<String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let render_toast = render_toast.unwrap_or_else(|| {
        Callback::new(|p: toast::ToastPropsWithOwner| {
            rsx! { {DynamicNode::Component(p.into_vcomponent(StyledToast))} }
        })
    });

    rsx! {
        toast::ToastProvider {
            class: Styles::dx_toast_container,
            default_duration,
            max_toasts,
            render_toast,
            list_class: merge_class(Styles::dx_toast_list.to_string(), list_class),
            item_class: merge_class(Styles::dx_toast_item.to_string(), item_class),
            attributes,
            {children}
        }
    }
}

fn merge_class(base: String, extra: Option<String>) -> Option<String> {
    Some(match extra {
        Some(extra) if !extra.is_empty() => format!("{base} {extra}"),
        _ => base,
    })
}
