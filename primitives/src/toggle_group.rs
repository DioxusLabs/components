use crate::{
    focus::{use_focus_controlled_item, use_focus_provider, FocusState},
    toggle::Toggle,
    use_controlled,
};
use dioxus_lib::prelude::*;
use std::collections::HashSet;

// Todo: docs, test controlled version

#[derive(Clone, Copy)]
struct ToggleGroupCtx {
    // State
    disabled: ReadOnlySignal<bool>,
    pressed: Memo<HashSet<usize>>,
    set_pressed: Callback<HashSet<usize>>,

    allow_multiple_pressed: ReadOnlySignal<bool>,

    // Focus state
    focus: FocusState,

    horizontal: ReadOnlySignal<bool>,
    roving_focus: ReadOnlySignal<bool>,
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

    fn is_horizontal(&self) -> bool {
        (self.horizontal)()
    }

    fn focus_next(&mut self) {
        if !(self.roving_focus)() {
            return;
        }

        self.focus.focus_next();
    }

    fn focus_prev(&mut self) {
        if !(self.roving_focus)() {
            return;
        }

        self.focus.focus_prev();
    }

    fn is_roving_focus(&self) -> bool {
        (self.roving_focus)()
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupProps {
    #[props(default)]
    default_pressed: HashSet<usize>,

    pressed: ReadOnlySignal<Option<HashSet<usize>>>,

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

    let focus = use_focus_provider(props.roving_loop);
    let mut ctx = use_context_provider(|| ToggleGroupCtx {
        pressed,
        set_pressed,
        allow_multiple_pressed: props.allow_multiple_pressed,
        disabled: props.disabled,

        focus,
        horizontal: props.horizontal,
        roving_focus: props.roving_focus,
    });

    rsx! {
        div {
            onfocusout: move |_| ctx.focus.set_focus(None),

            "data-orientation": ctx.orientation(),
            "data-allow-multiple-pressed": ctx.allow_multiple_pressed,
            ..props.attributes,

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ToggleItemProps {
    index: ReadOnlySignal<usize>,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn ToggleItem(props: ToggleItemProps) -> Element {
    let mut ctx: ToggleGroupCtx = use_context();

    // We need a kept-alive signal to control the toggle.
    let mut pressed = use_signal(|| ctx.is_pressed(props.index.cloned()));
    use_effect(move || {
        let is_pressed = ctx.is_pressed(props.index.cloned());
        pressed.set(is_pressed);
    });

    // Tab index for roving index
    let tab_index = use_memo(move || {
        if !ctx.is_roving_focus() {
            return "0";
        }

        match ctx.focus.is_recent_focus(props.index.cloned()) {
            true => "0",
            false => "-1",
        }
    });

    // Handle settings focus
    let onmounted = use_focus_controlled_item(props.index);

    rsx! {
        Toggle {
            onmounted,
            onfocus: move |_| ctx.focus.set_focus(Some(props.index.cloned())),
            onkeydown: move |event: Event<KeyboardData>| {
                let key = event.key();
                let horizontal = ctx.is_horizontal();
                let mut prevent_default = true;

                match key {
                    Key::ArrowUp if !horizontal => ctx.focus_prev(),
                    Key::ArrowDown if !horizontal => ctx.focus_next(),
                    Key::ArrowLeft if horizontal => ctx.focus_prev(),
                    Key::ArrowRight if horizontal => ctx.focus_next(),
                    Key::Home => ctx.focus.focus_first(),
                    Key::End => ctx.focus.focus_last(),
                    _ => prevent_default = false,
                };

                if prevent_default {
                    event.prevent_default();
                }
            },

            tabindex: tab_index,
            disabled: (ctx.disabled)() || (props.disabled)(),
            "data-orientation": ctx.orientation(),

            pressed: pressed(),
            on_pressed_change: move |pressed| {
                ctx.set_pressed(props.index.cloned(), pressed);
            },

            attributes: props.attributes.clone(),

            {props.children}
        }
    }
}
