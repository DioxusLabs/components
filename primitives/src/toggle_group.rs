use crate::{toggle::Toggle, use_controlled};
use dioxus_lib::prelude::*;
use std::collections::HashSet;

// Todo: docs, test controlled version, focus/keyboard management.

#[derive(Clone, Copy)]
struct ToggleGroupCtx {
    // State
    disabled: ReadOnlySignal<bool>,
    pressed: Memo<HashSet<usize>>,
    set_pressed: Callback<HashSet<usize>>,

    allow_multiple_pressed: ReadOnlySignal<bool>,

    // Keyboard nav data
    item_count: Signal<usize>,
    current_focus: Signal<Option<usize>>,
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

    fn set_pressed(&self, id: usize, pressed: bool) {
        let mut new_pressed = (self.pressed)();
        match pressed {
            false => new_pressed.remove(&id),
            true => {
                if !(self.allow_multiple_pressed)() {
                    new_pressed.clear();
                }
                new_pressed.insert(id)
            }
        };
        self.set_pressed.call(new_pressed);
    }

    fn horizontal(&self) -> bool {
        (self.horizontal)()
    }

    fn focus_next(&mut self) {
        if !(self.roving_focus)() {
            return;
        }

        if let Some(current_focus) = (self.current_focus)() {
            self.current_focus.set(Some(current_focus.saturating_add(1)));
        }
    }

    fn focus_prev(&mut self) {
        if !(self.roving_focus)() {
            return;
        }

        if let Some(current_focus) = (self.current_focus)() {
            self.current_focus.set(Some(current_focus.saturating_sub(1)));
        }
    }

    fn set_focus(&mut self, id: Option<usize>) {

    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupProps {
    #[props(default)]
    default_pressed: HashSet<usize>,

    pressed: Option<Signal<HashSet<usize>>>,

    #[props(default)]
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
    let (pressed, set_pressed) = use_controlled(
        props.pressed,
        props.default_pressed,
        props.on_pressed_change,
    );

    let ctx = use_context_provider(|| ToggleGroupCtx {
        pressed,
        set_pressed,
        allow_multiple_pressed: props.allow_multiple_pressed,
        disabled: props.disabled,

        item_indexes: Signal::new(HashSet::new()),
        current_focus: Signal::new(None),
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
    index: usize,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    // Extending props onto another component doesn't work so we need this.
    //#[props(extends = GlobalAttributes)]
    //attributes: Vec<Attribute>,
    id: Option<String>,
    class: Option<String>,

    children: Element,
}

#[component]
pub fn ToggleItem(props: ToggleItemProps) -> Element {
    let ctx: ToggleGroupCtx = use_context();

    let mut pressed = use_signal(|| ctx.is_pressed(props.index));
    use_effect(move || {
        let is_pressed = ctx.is_pressed(props.index);
        pressed.set(is_pressed);
    });

    let tab_index = use_memo(|| {});

    rsx! {
        Toggle {
            disabled: (ctx.disabled)() || (props.disabled)(),
            "data-orientation": ctx.orientation(),

            pressed,
            on_pressed_change: move |pressed| {
                ctx.set_pressed(props.index, pressed);
            },

            id: props.id,
            class: props.class,
            tabindex: "x",
            //..props.attributes,

            {props.children}
        }
    }
}
