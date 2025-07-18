//! Defines the [`Checkbox`] component and its subcomponents, which manage checkbox inputs with controlled state.

use crate::{use_controlled, use_unique_id};
use dioxus::{document::eval, prelude::*};
use std::ops::Not;

/// The state of a [`Checkbox`] component.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckboxState {
    /// The checkbox is checked.
    Checked,
    /// The checkbox is in an indeterminate state, meaning it is neither checked nor unchecked.
    Indeterminate,
    /// The checkbox is unchecked.
    Unchecked,
}

impl CheckboxState {
    fn to_aria_checked(self) -> &'static str {
        match self {
            CheckboxState::Checked => "true",
            CheckboxState::Indeterminate => "mixed",
            CheckboxState::Unchecked => "false",
        }
    }

    fn to_data_state(self) -> &'static str {
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
    checked: Memo<CheckboxState>,
    disabled: ReadOnlySignal<bool>,
}

/// The props for the [`Checkbox`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    /// The controlled state of the checkbox.
    pub checked: ReadOnlySignal<Option<CheckboxState>>,

    /// The default state of the checkbox when it is not controlled.
    #[props(default = CheckboxState::Unchecked)]
    pub default_checked: CheckboxState,

    /// Whether the checkbox is required in a form.
    #[props(default)]
    pub required: ReadOnlySignal<bool>,

    /// Whether the checkbox is disabled.
    #[props(default)]
    pub disabled: ReadOnlySignal<bool>,

    /// The name of the checkbox, used in forms.
    #[props(default)]
    pub name: ReadOnlySignal<String>,

    /// The value of the checkbox, which can be used in forms.
    #[props(default = ReadOnlySignal::new(Signal::new(String::from("on"))))]
    pub value: ReadOnlySignal<String>,

    /// Callback that is called when the checked state changes.
    #[props(default)]
    pub on_checked_change: Callback<CheckboxState>,

    /// Additional attributes to apply to the checkbox element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children to render inside the checkbox.
    children: Element,
}

/// # Checkbox
///
/// The `Checkbox` component is a controlled checkbox input that allows users to toggle a state. It can be used in forms or standalone.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Checkbox {
///             name: "tos-check",
///             aria_label: "Demo Checkbox",
///             CheckboxIndicator {
///                 "✅"
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Checkbox`] component defines the following data attributes you can use to control styling:
/// - `data-state`: The state of the checkbox. Possible values are `checked`, `indeterminate`, or `unchecked`.
/// - `data-disabled`: Indicates if the checkbox is disabled. values are `true` or `false`.
#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let (checked, set_checked) = use_controlled(
        props.checked,
        props.default_checked,
        props.on_checked_change,
    );

    use_context_provider(|| CheckboxCtx {
        checked,
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

/// # CheckboxIndicator
///
/// The indicator for the [`Checkbox`] component, which visually represents the checkbox state. The
/// children will only be rendered when the checkbox is checked.
///
/// This must be used inside a [`Checkbox`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Checkbox {
///             name: "tos-check",
///             aria_label: "Demo Checkbox",
///             CheckboxIndicator {
///                 "✅"
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`CheckboxIndicator`] component defines the following data attributes you can use to control styling:
/// - `data-state`: The state of the checkbox. Possible values are `checked`, `indeterminate`, or `unchecked`.
/// - `data-disabled`: Indicates if the checkbox is disabled. values are `true` or `false`.
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
            tabindex: "-1",
            position: "absolute",
            pointer_events: "none",
            opacity: "0",
            margin: "0",
            transform: "translateX(-100%)",

            // Default checked
            checked: default_checked != CheckboxState::Unchecked,

            ..attributes,
        }
    }
}
