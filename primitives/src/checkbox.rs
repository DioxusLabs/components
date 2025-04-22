use crate::{use_controlled, use_unique_id};
use dioxus_lib::{document::eval, prelude::*};
use std::ops::Not;

// TODO: Docs

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
        !matches!(value, CheckboxState::Unchecked)
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
    required: ReadOnlySignal<bool>,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(default)]
    name: ReadOnlySignal<String>,

    #[props(default = ReadOnlySignal::new(Signal::new(String::from("on"))))]
    value: ReadOnlySignal<String>,

    #[props(default)]
    on_checked_change: Callback<CheckboxState>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let (checked, set_checked) = use_controlled(
        props.checked,
        props.default_checked,
        props.on_checked_change,
    );

    let _ctx = use_context_provider(|| CheckboxCtx {
        checked: checked.into(),
        disabled: props.disabled,
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

            onclick: move |_| {
                let new_checked = !checked();
                set_checked.call(new_checked);
            },

            // Aria says only spacebar can change state of checkboxes.
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
