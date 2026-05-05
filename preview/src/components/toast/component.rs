use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use dioxus_primitives::toast::{self, Toast, ToastProps, ToastProviderProps};

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
pub fn ToastProvider(props: ToastProviderProps) -> Element {
    rsx! {
        toast::ToastProvider {
            class: Styles::dx_toast_container,
            default_duration: props.default_duration,
            max_toasts: props.max_toasts,
            render_toast: Callback::new(|p: toast::ToastPropsWithOwner| {
                rsx! { {DynamicNode::Component(p.into_vcomponent(StyledToast))} }
            }),
            list_class: Some(Styles::dx_toast_list.to_string()),
            item_class: Some(Styles::dx_toast_item.to_string()),
            {props.children}
        }
    }
}
