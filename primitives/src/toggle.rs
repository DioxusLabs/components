//! Defines the [`Toggle`] component for creating toggle buttons.

use crate::use_controlled;
use dioxus::prelude::*;

/// The props for the [`Toggle`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ToggleProps {
    /// The controlled pressed state of the toggle.
    pub pressed: ReadOnlySignal<Option<bool>>,

    /// The default pressed state when uncontrolled.
    #[props(default)]
    pub default_pressed: bool,

    /// Whether the toggle is disabled.
    #[props(default)]
    pub disabled: ReadOnlySignal<bool>,

    /// Callback fired when the pressed state changes.
    #[props(default)]
    pub on_pressed_change: Callback<bool>,

    // https://github.com/DioxusLabs/dioxus/issues/2467
    /// Callback fired when the toggle is mounted.
    #[props(default)]
    pub onmounted: Callback<Event<MountedData>>,
    /// Callback fired when the toggle receives focus.
    #[props(default)]
    pub onfocus: Callback<Event<FocusData>>,
    /// Callback fired when a key is pressed on the toggle.
    #[props(default)]
    pub onkeydown: Callback<Event<KeyboardData>>,

    /// Additional attributes to apply to the toggle element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children of the toggle component.
    children: Element,
}

/// # Toggle
///
/// The `Toggle` component is a button that can be on or off.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::toggle::Toggle;
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Toggle { width: "2rem", height: "2rem",
///             em { "B" }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Toggle`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the state of the toggle. Values are `on` or `off`.
/// - `data-disabled`: Indicates if the toggle is disabled. Values are `true` or `false`.
#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let (pressed, set_pressed) = use_controlled(
        props.pressed,
        props.default_pressed,
        props.on_pressed_change,
    );

    rsx! {
        button {
            onmounted: props.onmounted,
            onfocus: props.onfocus,
            onkeydown: props.onkeydown,

            type: "button",
            disabled: props.disabled,
            aria_pressed: pressed,
            "data-state": if pressed() { "on" } else { "off" },
            "data-disabled": props.disabled,

            onclick: move |_| {
                let new_pressed = !pressed();
                set_pressed.call(new_pressed);
            },

            ..props.attributes,
            {props.children}
        }
    }
}
