//! Unstyled Checkbox primitive for Dioxus
//! Usage: style externally via class or CSS selectors.

use dioxus_lib::prelude::*;

use crate::use_unique_id;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckboxState {
    Checked,
    Unchecked,
    Indeterminate,
}

impl CheckboxState {
    pub fn to_aria_checked(&self) -> &'static str {
        match self {
            CheckboxState::Checked => "true",
            CheckboxState::Indeterminate => "mixed",
            CheckboxState::Unchecked => "false",
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
struct CheckboxCtx {
    checked: Signal<CheckboxState>,
    set_checked: Callback<CheckboxState>,
    disabled: bool,
    id: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct CheckboxRootProps {
    #[props(default = CheckboxState::Unchecked)]
    default_checked: CheckboxState,
    #[props(default)]
    disabled: bool,
    #[props(default)]
    id: Option<String>,
    #[props(default)]
    on_checked_change: Callback<CheckboxState>,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn CheckboxRoot(props: CheckboxRootProps) -> Element {
    let id = props
        .id
        .clone()
        .unwrap_or_else(|| use_unique_id().to_string());
    let mut checked = use_signal(|| props.default_checked);

    let set_checked = use_callback(move |state: CheckboxState| {
        checked.set(state);
        props.on_checked_change.call(state);
    });

    use_context_provider(|| CheckboxCtx {
        checked,
        set_checked,
        disabled: props.disabled,
        id: id.clone(),
    });

    let on_click = use_callback(move |_| {
        if !props.disabled {
            let next = match checked() {
                CheckboxState::Checked => CheckboxState::Unchecked,
                CheckboxState::Unchecked => CheckboxState::Checked,
                CheckboxState::Indeterminate => CheckboxState::Checked,
            };
            set_checked.call(next);
        }
    });

    rsx! {
        button {
            r#type: "button",
            id: id.clone(),
            role: "checkbox",
            aria_checked: checked().to_aria_checked(),
            aria_disabled: props.disabled,
            disabled: props.disabled,
            tabindex: "0",
            "data-state": match checked() {
                CheckboxState::Checked => "checked",
                CheckboxState::Unchecked => "unchecked",
                CheckboxState::Indeterminate => "indeterminate",
            },
            "data-disabled": props.disabled,
            onclick: on_click,
            ..props.attributes,
            // onkeydown: move |e| {
            //     if e.key() == " " || e.key() == "Space" {
            //         e.prevent_default();
            //         on_click(());
            //     }
            // },
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CheckboxIndicatorProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn CheckboxIndicator(props: CheckboxIndicatorProps) -> Element {
    let ctx: CheckboxCtx = use_context();
    let checked = (ctx.checked)();

    if checked == CheckboxState::Checked || checked == CheckboxState::Indeterminate {
        rsx! {
            span { "aria-hidden": "true", ..props.attributes, {props.children} }
        }
    } else {
        VNode::empty()
    }
}
