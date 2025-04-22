use crate::{toggle::Toggle, use_controlled};
use dioxus_lib::prelude::*;
use std::{collections::HashSet, rc::Rc};

// Todo: docs, test controlled version

#[derive(Clone, Copy)]
struct ToggleGroupCtx {
    // State
    disabled: ReadOnlySignal<bool>,
    pressed: Memo<HashSet<usize>>,
    set_pressed: Callback<HashSet<usize>>,

    allow_multiple_pressed: ReadOnlySignal<bool>,

    // Keyboard nav data
    item_count: Signal<usize>,
    // For tracking tabindex
    recent_focus: Signal<usize>,
    // For tracking who should set focus
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

    fn register_item(&mut self) {
        self.item_count += 1;
    }

    fn unregister_item(&mut self) {
        self.item_count -= 1;
    }

    fn is_horizontal(&self) -> bool {
        (self.horizontal)()
    }

    fn focus_next(&mut self) {
        if !(self.roving_focus)() {
            return;
        }

        if let Some(current_focus) = (self.current_focus)() {
            let mut new_focus = current_focus.saturating_add(1);

            let item_count = (self.item_count)();
            if new_focus >= item_count {
                match (self.roving_loop)() {
                    true => new_focus = 0,
                    false => new_focus = item_count.saturating_sub(1),
                }
            }

            self.current_focus.set(Some(new_focus));
        }
    }

    fn focus_prev(&mut self) {
        if !(self.roving_focus)() {
            return;
        }

        if let Some(current_focus) = (self.current_focus)() {
            let mut new_focus = current_focus.saturating_sub(1);
            if current_focus == 0 && (self.roving_loop)() {
                new_focus = (self.item_count)().saturating_sub(1);
            }

            self.current_focus.set(Some(new_focus));
        }
    }

    /// Set the currently focused item.
    ///
    /// This should be used by `focus`/`focusout` event only to start tracking focus.
    fn set_focus(&mut self, id: Option<usize>) {
        self.current_focus.set(id);
        if let Some(id) = id {
            self.recent_focus.set(id);
        }
    }

    pub fn focus_start(&mut self) {
        self.current_focus.set(Some(0));
    }

    pub fn focus_end(&mut self) {
        let new_focus = self.item_count.write().saturating_sub(1);
        self.current_focus.set(Some(new_focus));
    }

    fn is_focused(&self, id: usize) -> bool {
        (self.current_focus)().map(|x| x == id).unwrap_or(false)
    }

    fn is_recent_focus(&self, id: usize) -> bool {
        let recent = (self.recent_focus)();
        recent == id
    }

    fn is_roving_focus(&self) -> bool {
        (self.roving_focus)()
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

    let mut ctx = use_context_provider(|| ToggleGroupCtx {
        pressed,
        set_pressed,
        allow_multiple_pressed: props.allow_multiple_pressed,
        disabled: props.disabled,

        item_count: Signal::new(0),
        recent_focus: Signal::new(0),
        current_focus: Signal::new(None),
        horizontal: props.horizontal,
        roving_focus: props.roving_focus,
        roving_loop: props.roving_loop,
    });

    rsx! {
        div {
            onfocusout: move |_| ctx.set_focus(None),

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
    let mut ctx: ToggleGroupCtx = use_context();

    // un/register item with ctx
    use_hook(move || ctx.register_item());
    use_drop(move || ctx.unregister_item());

    // We need a kept-alive signal to control the toggle.
    let mut pressed = use_signal(|| ctx.is_pressed(props.index));
    use_effect(move || {
        let is_pressed = ctx.is_pressed(props.index);
        pressed.set(is_pressed);
    });

    // Tab index for roving index
    let tab_index = use_memo(move || {
        if !ctx.is_roving_focus() {
            return "0";
        }

        match ctx.is_recent_focus(props.index) {
            true => "0",
            false => "-1",
        }
    });

    // Handle settings focus
    let mut toggle_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    use_effect(move || {
        let is_focused = ctx.is_focused(props.index);
        if is_focused {
            if let Some(md) = toggle_ref() {
                spawn(async move {
                    let _ = md.set_focus(true).await;
                });
            }
        }
    });

    rsx! {
        Toggle {
            onmounted: move |data: Event<MountedData>| toggle_ref.set(Some(data.data())),
            onfocus: move |_| ctx.set_focus(Some(props.index)),
            onkeydown: move |event: Event<KeyboardData>| {
                let key = event.key();
                let horizontal = ctx.is_horizontal();
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

            tabindex: tab_index,
            disabled: (ctx.disabled)() || (props.disabled)(),
            "data-orientation": ctx.orientation(),

            pressed,
            on_pressed_change: move |pressed| {
                ctx.set_pressed(props.index, pressed);
            },

            id: props.id,
            class: props.class,
            //..props.attributes,

            {props.children}
        }
    }
}
