//! Defines the [`Switch`] component and its sub-components.

use crate::use_controlled;
use dioxus::prelude::*;

/// The props for the [`Switch`] component.
#[derive(Props, Clone, PartialEq)]
pub struct SwitchProps {
    /// The controlled checked state of the switch.
    pub checked: ReadOnlySignal<Option<bool>>,

    /// The default checked state when uncontrolled.
    #[props(default = false)]
    pub default_checked: bool,

    /// Whether the switch is disabled.
    #[props(default = ReadOnlySignal::new(Signal::new(false)))]
    pub disabled: ReadOnlySignal<bool>,

    /// Whether the switch is required in a form.
    #[props(default)]
    pub required: ReadOnlySignal<bool>,

    /// The name attribute for form submission.
    #[props(default)]
    pub name: ReadOnlySignal<String>,

    /// The value attribute for form submission.
    #[props(default = ReadOnlySignal::new(Signal::new(String::from("on"))))]
    pub value: ReadOnlySignal<String>,

    /// Callback fired when the checked state changes.
    #[props(default)]
    pub on_checked_change: Callback<bool>,

    /// Additional attributes to apply to the switch element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children of the switch component.
    children: Element,
}

/// # Switch
///
/// The `Switch` component is a toggle control that allows users to switch a state on or off.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::switch::{Switch, SwitchThumb};
/// #[component]
/// fn Demo() -> Element {
///     let mut checked = use_signal(|| false);
///     rsx! {
///         Switch {
///             checked: checked(),
///             aria_label: "Switch Demo",
///             SwitchThumb {}
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Switch`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the state of the switch. Values are `checked` or `unchecked`.
/// - `data-disabled`: Indicates if the switch is disabled. Values are `true` or `false`.
#[component]
pub fn Switch(props: SwitchProps) -> Element {
    let (checked, set_checked) = use_controlled(
        props.checked,
        props.default_checked,
        props.on_checked_change,
    );

    rsx! {
        button {
            type: "button",
            role: "switch",
            value: props.value,
            aria_checked: checked,
            aria_required: props.required,
            disabled: props.disabled,
            "data-state": if checked() { "checked" } else { "unchecked" },
            // Only add data-disabled when actually disabled
            "data-disabled": if (props.disabled)() { "true" } else { "false" },

            onclick: move |_| {
                let new_checked = !checked();
                set_checked.call(new_checked);
            },

            // Switches should only toggle on Space, not Enter
            onkeydown: move |e| {
                if e.key() == Key::Enter {
                    e.prevent_default();
                }
            },

            ..props.attributes,
            {props.children}
        }

        // Hidden input for form submission
        input {
            type: "checkbox",
            aria_hidden: true,
            tabindex: -1,
            name: props.name,
            value: props.value,
            checked,
            disabled: props.disabled,
            style: "transform: translateX(-100%); position: absolute; pointer-events: none; opacity: 0; margin: 0; width: 0; height: 0;",
        }
    }
}

/// The props for the [`SwitchThumb`] component.
#[derive(Props, Clone, PartialEq)]
pub struct SwitchThumbProps {
    /// Additional attributes to apply to the thumb element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// The children of the thumb component.
    children: Element,
}

/// # SwitchThumb
///
/// The `SwitchThumb` component represents the visual thumb indicator that moves when the switch is toggled.
///
/// This must be used inside a [`Switch`] component.
///
/// ## Example
///
/// ```rust
///
/// use dioxus::prelude::*;
/// use dioxus_primitives::switch::{Switch, SwitchThumb};
/// #[component]
/// fn Demo() -> Element {
///     let mut checked = use_signal(|| false);
///     rsx! {
///         Switch {
///             checked: checked(),
///             aria_label: "Switch Demo",
///             SwitchThumb {}
///         }
///     }
/// }
/// ```
#[component]
pub fn SwitchThumb(props: SwitchThumbProps) -> Element {
    rsx! {
        span { ..props.attributes, {props.children} }
    }
}
