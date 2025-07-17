//! Defines the [`Collapsible`] component and its sub-components.

use crate::{use_controlled, use_id_or, use_unique_id};
use dioxus::prelude::*;

// TODO: more docs

#[derive(Clone, Copy)]
struct CollapsibleCtx {
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: ReadOnlySignal<bool>,
    keep_mounted: ReadOnlySignal<bool>,
    aria_controls_id: Signal<String>,
}

/// The props for the [`Collapsible`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleProps {
    /// Keep [`CollapsibleContent`] mounted in the DOM when the collapsible is closed.
    ///
    /// This does not apply any special ARIA or other attributes.
    #[props(default)]
    pub keep_mounted: ReadOnlySignal<bool>,

    /// The default `open` state.
    ///
    /// This will be overridden if the component is controlled.
    #[props(default)]
    pub default_open: bool,

    /// The disabled state of the collapsible.
    #[props(default)]
    pub disabled: ReadOnlySignal<bool>,

    /// The controlled `open` state of the collapsible.
    ///
    /// If this is provided, you must use `on_open_change`.
    pub open: ReadOnlySignal<Option<bool>>,

    /// A callback for when the open state changes.
    ///
    /// The provided argument is a bool of whether the collapsible is open or closed.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Additional attributes for the collapsible element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children of the collapsible component.
    children: Element,
}

/// # Collapsible
///
/// The [`Collapsible`] component is a container that can be expanded or collapsed to show or hide its content.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Collapsible {
///             CollapsibleTrigger {
///                 b { "Recent Activity" }
///             }
///             CollapsibleContent {
///                 div {
///                     "Fixed a bug in the collapsible component",
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Collapsible`] component defines the following data attributes you can use to control styling:
/// - `data-open`: Indicates if the collapsible is open. Values are `true` or `false`.
/// - `data-disabled`: Indicates if the collapsible is disabled. values are `true` or `false`.
#[component]
pub fn Collapsible(props: CollapsibleProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    let aria_controls_id = use_unique_id();
    use_context_provider(|| CollapsibleCtx {
        open,
        set_open,
        disabled: props.disabled,
        keep_mounted: props.keep_mounted,
        aria_controls_id,
    });

    rsx! {
        div {
            "data-open": open,
            "data-disabled": props.disabled,
            ..props.attributes,

            {props.children}
        }
    }
}

/// The props for the [`CollapsibleContent`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleContentProps {
    /// The ID of the collapsible content element.
    pub id: ReadOnlySignal<Option<String>>,

    /// Additional attributes for the collapsible content element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// The children of the collapsible content.
    children: Element,
}

/// # CollapsibleContent
///
/// The [`CollapsibleContent`] component defines the content of a collapsible section. The
/// contents will only be rendered if the collapsible is open, or if the [`CollapsibleProps::keep_mounted`] prop is set to `true`.
///
/// This must be used inside a [`Collapsible`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Collapsible {
///             CollapsibleTrigger {
///                 b { "Recent Activity" }
///             }
///             CollapsibleContent {
///                 div {
///                     "Fixed a bug in the collapsible component",
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`CollapsibleContent`] component defines the following data attributes you can use to control styling:
/// - `data-open`: Indicates if the collapsible is open. Values are `true` or `false`.
/// - `data-disabled`: Indicates if the collapsible is disabled. values are `true` or `false`.
#[component]
pub fn CollapsibleContent(props: CollapsibleContentProps) -> Element {
    let ctx: CollapsibleCtx = use_context();
    let id = use_id_or(ctx.aria_controls_id, props.id);

    let open = ctx.open;

    rsx! {
        div {
            id: id,
            "data-open": open,
            "data-disabled": ctx.disabled,
            ..props.attributes,

            if open() || (ctx.keep_mounted)() {
                {props.children}
            }
        }
    }
}

/// The props for the [`CollapsibleTrigger`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleTriggerProps {
    /// Additional attributes for the collapsible trigger element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// The children of the collapsible trigger.
    children: Element,
}

/// # CollapsibleTrigger
///
/// The [`CollapsibleTrigger`] component is the button or element that toggles the visibility of the collapsible content.
///
/// This must be used inside a [`Collapsible`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Collapsible {
///             CollapsibleTrigger {
///                 b { "Recent Activity" }
///             }
///             CollapsibleContent {
///                 div {
///                     "Fixed a bug in the collapsible component",
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`CollapsibleTrigger`] component defines the following data attributes you can use to control styling:
/// - `data-open`: Indicates if the collapsible is open. Values are `true` or `false`.
/// - `data-disabled`: Indicates if the collapsible is disabled. values are `true` or `false`.
#[component]
pub fn CollapsibleTrigger(props: CollapsibleTriggerProps) -> Element {
    let ctx: CollapsibleCtx = use_context();

    let open = ctx.open;

    rsx! {

        button {
            type: "button",
            "data-open": open,
            "data-disabled": ctx.disabled,
            disabled: ctx.disabled,

            aria_controls: ctx.aria_controls_id,
            aria_expanded: open,

            onclick: move |_| {
                let new_open = !open();
                ctx.set_open.call(new_open);
            },

            ..props.attributes,
            {props.children}
        }
    }
}
