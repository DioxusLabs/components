use std::{collections::HashMap, rc::Rc};

use crate::use_controlled;
use dioxus_lib::prelude::*;

#[derive(Clone, Copy)]
struct RadioGroupCtx {
    // State
    disabled: ReadOnlySignal<bool>,
    value: ReadOnlySignal<String>,
    set_value: Callback<String>,

    // Keyboard nav data
    // A map of tabindex -> value in the enabled radio items
    values: Signal<HashMap<usize, String>>,
    recent_focus: Signal<usize>,
    current_focus: Signal<Option<usize>>,

    horizontal: ReadOnlySignal<bool>,
    roving_focus: ReadOnlySignal<bool>,
    roving_loop: ReadOnlySignal<bool>,
}

impl RadioGroupCtx {
    /// Set the currently focused radio item.
    ///
    /// This should be used by `focus`/`focusout` event only to start tracking focus.
    fn set_focus(&mut self, id: Option<usize>) {
        self.current_focus.set(id);
        if let Some(id) = id {
            self.recent_focus.set(id);
        }
    }

    /// Set the value of the radio group.
    fn set_value(&mut self, value: String) {
        let current_value = self.value.peek();
        if *current_value == value {
            return; // No change, do nothing
        }
        self.set_value.call(value);
    }

    fn focus_next(&mut self) {
        if let Some(current_focus) = (self.current_focus)() {
            let mut new_focus = current_focus.saturating_add(1);

            let item_count = self.item_count();
            if new_focus >= item_count {
                match (self.roving_loop)() {
                    true => new_focus = 0,
                    false => new_focus = item_count.saturating_sub(1),
                }
            }

            self.current_focus.set(Some(new_focus));
            self.select_focused_value();
        }
    }

    fn focus_prev(&mut self) {
        if let Some(current_focus) = (self.current_focus)() {
            let mut new_focus = current_focus.saturating_sub(1);
            if current_focus == 0 && (self.roving_loop)() {
                new_focus = self.item_count().saturating_sub(1);
            }

            self.current_focus.set(Some(new_focus));
            self.select_focused_value();
        }
    }

    fn select_focused_value(&mut self) {
        if let Some(current_focus) = (self.current_focus)() {
            let value = { self.values.read().get(&current_focus).cloned() };
            if let Some(value) = value {
                self.set_value(value.clone());
            }
        }
    }

    fn focus_start(&mut self) {
        self.current_focus.set(Some(0));
    }

    fn focus_end(&mut self) {
        let new_focus = self.item_count().saturating_sub(1);
        self.current_focus.set(Some(new_focus));
    }

    fn item_count(&self) -> usize {
        self.values.read().len()
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupProps {
    value: ReadOnlySignal<Option<String>>,

    #[props(default)]
    default_value: String,

    #[props(default)]
    on_value_change: Callback<String>,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(default)]
    required: ReadOnlySignal<bool>,

    #[props(default)]
    name: ReadOnlySignal<String>,

    #[props(default)]
    horizontal: ReadOnlySignal<bool>,

    #[props(default = ReadOnlySignal::new(Signal::new(true)))]
    roving_focus: ReadOnlySignal<bool>,

    #[props(default = ReadOnlySignal::new(Signal::new(true)))]
    roving_loop: ReadOnlySignal<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    let (value, set_value) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let mut ctx = use_context_provider(|| RadioGroupCtx {
        value: value.into(),
        set_value,
        disabled: props.disabled,

        values: Signal::new(Default::default()),
        recent_focus: Signal::new(0),
        current_focus: Signal::new(None),
        horizontal: props.horizontal,
        roving_focus: props.roving_focus,
        roving_loop: props.roving_loop,
    });

    rsx! {
        div {
            role: "radiogroup",
            "data-orientation": if (props.horizontal)() { "horizontal" } else { "vertical" },
            "data-disabled": (props.disabled)(),
            aria_required: props.required,

            onfocusout: move |_| ctx.set_focus(None),
            ..props.attributes,

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct RadioItemProps {
    value: ReadOnlySignal<String>,
    index: ReadOnlySignal<usize>,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    id: Option<String>,
    class: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn RadioItem(props: RadioItemProps) -> Element {
    let mut ctx: RadioGroupCtx = use_context();

    // Move registration logic into an effect
    use_effect(move || {
        if (props.disabled)() {
            return;
        }
        // Register on mount
        ctx.values.write().insert((props.index)(), (props.value)());
    });

    let value = (props.value)().clone();
    let checked = use_memo(move || (ctx.value)() == value);

    // Tab index for roving index
    let tab_index = use_memo(move || {
        if !(ctx.roving_focus)() {
            return "0";
        }

        if checked() {
            return "0";
        }
        let current_focus = (ctx.current_focus)();
        if let Some(current_focus) = current_focus {
            if current_focus == (props.index)() {
                return "0";
            }
        } else if (ctx.value)().is_empty() {
            return "0";
        }

        "-1"
    });

    let mut element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    use_effect(move || {
        let Some(element) = element() else {
            return;
        };
        if (ctx.current_focus)() == Some((props.index)()) {
            spawn(async move {
                _ = element.set_focus(true).await;
            });
        }
    });

    rsx! {
        button {
            role: "radio",
            id: props.id,
            class: props.class,
            tabindex: tab_index,

            aria_checked: checked,
            "data-state": if checked() { "checked" } else { "unchecked" },
            "data-disabled": (ctx.disabled)() || (props.disabled)(),
            disabled: (ctx.disabled)() || (props.disabled)(),

            onclick: move |_| {
                let value = (props.value)().clone();
                ctx.set_value(value);
            },

            onmounted: move |node| {
                element.set(Some(node.data()));
            },
            onfocus: move |_| ctx.set_focus(Some((props.index)())),

            onkeydown: move |event: Event<KeyboardData>| {
                let key = event.key();
                let horizontal = (ctx.horizontal)();
                let mut prevent_default = true;
                match key {
                    Key::ArrowUp if !horizontal => ctx.focus_prev(),
                    Key::ArrowDown if !horizontal => ctx.focus_next(),
                    Key::ArrowLeft if horizontal => ctx.focus_prev(),
                    Key::ArrowRight if horizontal => ctx.focus_next(),
                    Key::Home => ctx.focus_start(),
                    Key::End => ctx.focus_end(),
                    _ => prevent_default = false,
                };
                if prevent_default {
                    event.prevent_default();
                }
            },
            ..props.attributes,

            {props.children}
        }
    }
}
