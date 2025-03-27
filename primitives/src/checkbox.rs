use crate::use_unique_id;
use dioxus_lib::{document::eval, prelude::*};
use std::ops::Not;

/*
TODO

- Test controlled version
- Test other options
- Test form value bubbling
- Docs
*/

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckboxState {
    Checked,
    Indeterminate,
    Unchecked,
}

impl CheckboxState {
    pub fn to_aria_checked(&self) -> &str {
        match self {
            CheckboxState::Checked => "true",
            CheckboxState::Indeterminate => "mixed",
            CheckboxState::Unchecked => "false",
        }
    }

    pub fn to_data_state(&self) -> &str {
        match self {
            CheckboxState::Checked => "checked",
            CheckboxState::Indeterminate => "indeterminate",
            CheckboxState::Unchecked => "unchecked",
        }
    }
}

impl From<CheckboxState> for bool {
    fn from(value: CheckboxState) -> Self {
        match value {
            CheckboxState::Unchecked => false,
            _ => true,
        }
    }
}

impl Not for CheckboxState {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Unchecked => Self::Checked,
            _ => Self::Unchecked,
        }
    }
}

#[derive(Clone, Copy)]
struct CheckboxCtx {
    checked: ReadOnlySignal<CheckboxState>,
    disabled: ReadOnlySignal<bool>,
}

#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    checked: Option<Signal<CheckboxState>>,

    #[props(default = CheckboxState::Unchecked)]
    default_checked: CheckboxState,

    #[props(default)]
    form_control: ReadOnlySignal<bool>,

    #[props(default)]
    required: ReadOnlySignal<bool>,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(default)]
    name: ReadOnlySignal<String>,

    #[props(default = ReadOnlySignal::new(Signal::new(String::from("on"))))]
    value: ReadOnlySignal<String>,

    #[props(default)]
    on_checked_changed: Callback<CheckboxState>,

    #[props(default)]
    on_click: Callback<Event<MouseData>>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let mut internal_checked = use_signal(|| props.default_checked);
    let checked = use_memo(move || props.checked.unwrap_or(internal_checked)());
    let _ctx = use_context_provider(|| CheckboxCtx {
        checked: checked.into(),
        disabled: props.disabled,
    });

    let mut user_propogation_stopped = use_signal(|| false);

    // Call the checked changed.
    use_effect(move || {
        let checked = checked();
        props.on_checked_changed.call(checked);
    });

    rsx! {
        button {
            type: "button",
            value: props.value,
            role: "checkbox",
            aria_checked: checked().to_aria_checked(),
            aria_required: props.required,
            disabled: props.disabled,
            "data-state": checked().to_data_state(),
            "data-disabled": props.disabled,

            onclick: move |e| {
                if (props.form_control)() {
                    user_propogation_stopped.set(e.propagates());
                    if !user_propogation_stopped() {
                        e.stop_propagation();
                    }
                }

                internal_checked.set(!checked());
                props.on_click.call(e);
            },

            onkeydown: move |e| {
                if e.key() == Key::Enter {
                    e.prevent_default();
                }
            },

            ..props.attributes,
            {props.children}
        }
        BubbleInput {
            checked: checked,
            default_checked: props.default_checked,

            required: props.required,
            name: props.name,
            value: props.value,
            disabled: props.disabled,
        }
    }
}

#[component]
pub fn CheckboxIndicator(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx: CheckboxCtx = use_context();
    let checked = (ctx.checked)();

    rsx! {
        span {
            "data-state": checked.to_data_state(),
            "data-disabled": ctx.disabled,
            ..attributes,

            if checked.into() {
                {children}
            }
        }
    }
}

#[component]
fn BubbleInput(
    checked: ReadOnlySignal<CheckboxState>,
    default_checked: CheckboxState,
    #[props(extends = input)] attributes: Vec<Attribute>,
) -> Element {
    let id = use_unique_id();

    // Update the actual input state to match our virtual state.
    use_effect(move || {
        let checked = checked();
        let js = eval(
            r#"
            let id = await dioxus.recv();
            let action = await dioxus.recv();
            let input = document.getElementById(id);

            switch(action) {
                case "checked":
                    input.checked = true;
                    input.indeterminate = false;
                    break;
                case "indeterminate":
                    input.indeterminate = true;
                    input.checked = true;
                    break;
                case "unchecked": 
                    input.checked = false;
                    input.indeterminate = false;
                    break;
            }
            "#,
        );

        let _ = js.send(id());
        let _ = js.send(checked.to_data_state());
    });

    rsx! {
        input {
            id,
            type: "checkbox",
            aria_hidden: "true",
            tabindex: -1,
            position: "absolute",
            pointer_events: "none",
            opacity: 0,
            margin: 0,
            style: "transform: 'translateX(-100%)';",

            // Default checked
            checked: default_checked != CheckboxState::Unchecked,

            ..attributes,
        }
    }
}
