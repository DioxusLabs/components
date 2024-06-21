use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AlertProps {
    #[props(optional, default = "dxa-alert".into())]
    class: String,
    children: Element,
}

/// The `Alert` ARIA pattern.
/// 
/// When rendered or content is changed, screen readers can interrupt any current text-to-speech
/// with the new alert content. Items within an alert should not interrupt the current keyboard focus.
/// 
/// Alerts should be used infrequently and should not disappear automatically.
/// 
/// See the [alert pattern](https://www.w3.org/WAI/ARIA/apg/patterns/alert/).
#[component]
pub fn Alert(props: AlertProps) -> Element {
    rsx! {
        div {
            class: "{props.class}",
            role: "alert",
            {props.children}
        }
    }
}