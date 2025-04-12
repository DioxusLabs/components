use dioxus_lib::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    /// The current progress value, between 0 and max
    value: Option<ReadOnlySignal<f64>>,

    /// The maximum value. Defaults to 100
    #[props(default = ReadOnlySignal::new(Signal::new(100.0)))]
    max: ReadOnlySignal<f64>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    // Calculate percentage for styling and "data-state"
    let percentage = use_memo(move || {
        props.value.map(|v| {
            let max = (props.max)();
            (v() / max) * 100.0
        })
    });

    let state = use_memo(move || match percentage() {
        Some(_) => "loading",
        None => "indeterminate",
    });

    rsx! {
        div {
            role: "progressbar",
            "aria-valuemin": 0,
            "aria-valuemax": props.max,
            "aria-valuenow": props.value.map(|v| v()),
            "data-state": state,
            "data-value": props.value.map(|v| v().to_string()),
            "data-max": props.max,
            style: percentage().map(|p| format!("--progress-value: {}%", p)),
            ..props.attributes,

            {props.children}
        }
    }
}

/// The indicator that represents the progress visually
#[derive(Props, Clone, PartialEq)]
pub struct ProgressIndicatorProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn ProgressIndicator(props: ProgressIndicatorProps) -> Element {
    rsx! {
        div { ..props.attributes }
    }
}
