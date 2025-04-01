use std::collections::HashSet;

use dioxus_lib::prelude::*;

use crate::{toggle::Toggle, use_controlled};

#[derive(Clone, Copy)]
struct ToggleGroupCtx {
    // State
    disabled: ReadOnlySignal<bool>,
    pressed: Memo<HashSet<usize>>,
    set_pressed_items: Callback<HashSet<usize>>,

    // Keyboard nav data
    horizontal: ReadOnlySignal<bool>,
    roving_focus: ReadOnlySignal<bool>,
    roving_loop: ReadOnlySignal<bool>,
}

impl ToggleGroupCtx {
    fn orientation(&self) -> &'static str {
        match (self.horizontal)() {
            true => "horizontal",
            false => "vertical",
        }
    }

    fn is_pressed(&self, id: usize) -> bool {
        let pressed = (self.pressed)();
        pressed.contains(&id)
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupProps {
    default_pressed: HashSet<usize>,
    pressed: Option<Signal<HashSet<usize>>>,
    on_pressed_change: Callback<HashSet<usize>>,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(default)]
    allow_multiple_pressed: ReadOnlySignal<bool>,

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
pub fn ToggleGroup(props: ToggleGroupProps) -> Element {
    let (pressed, set_pressed_items) = use_controlled(
        props.pressed,
        props.default_pressed,
        props.on_pressed_change,
    );

    let ctx = use_context_provider(|| ToggleGroupCtx {
        pressed,
        set_pressed_items,
        disabled: props.disabled,
        horizontal: props.horizontal,
        roving_focus: props.roving_focus,
        roving_loop: props.roving_loop,
    });

    rsx! {
        div {
            "data-orientation": ctx.orientation(),
            ..props.attributes,

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ToggleItemProps {
    index: ReadOnlySignal<usize>,

    disabled: ReadOnlySignal<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn ToggleItem(props: ToggleItemProps) -> Element {
    let ctx: ToggleGroupCtx = use_context();

    let pressed = ctx.is_pressed((props.index)());

    rsx! {
        Toggle {
            pressed: Signal::new(pressed),
            disabled: (ctx.disabled)() || (props.disabled)(),
            "data-orientation": ctx.orientation(),

            //..props.attributes,
            {props.children}
        }
    }
}
