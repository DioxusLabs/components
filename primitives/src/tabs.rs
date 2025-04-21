use crate::use_controlled;
use dioxus_lib::prelude::*;

#[derive(Clone, Copy)]
struct TabsContext {
    // State
    value: ReadOnlySignal<String>,
    set_value: Callback<String>,
    disabled: ReadOnlySignal<bool>,

    // Keyboard nav data
    item_count: Signal<usize>,
    recent_focus: Signal<usize>,
    current_focus: Signal<Option<usize>>,

    // Orientation
    horizontal: ReadOnlySignal<bool>,
    roving_focus: ReadOnlySignal<bool>,
    roving_loop: ReadOnlySignal<bool>,
}

impl TabsContext {
    fn set_focus(&mut self, id: Option<usize>) {
        self.current_focus.set(id);
        if let Some(id) = id {
            self.recent_focus.set(id);
        }
    }

    fn focus_next(&mut self) {
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
        if let Some(current_focus) = (self.current_focus)() {
            let mut new_focus = current_focus.saturating_sub(1);
            if current_focus == 0 && (self.roving_loop)() {
                new_focus = (self.item_count)().saturating_sub(1);
            }

            self.current_focus.set(Some(new_focus));
        }
    }

    fn focus_start(&mut self) {
        self.current_focus.set(Some(0));
    }

    fn focus_end(&mut self) {
        let new_focus = (self.item_count)().saturating_sub(1);
        self.current_focus.set(Some(new_focus));
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    value: Option<Signal<String>>,

    #[props(default)]
    default_value: String,

    #[props(default)]
    on_value_change: Callback<String>,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

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
pub fn Tabs(props: TabsProps) -> Element {
    let (value, set_value) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let mut ctx = use_context_provider(|| TabsContext {
        value: value.into(),
        set_value,
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
            role: "tablist",
            "data-orientation": if (props.horizontal)() { "horizontal" } else { "vertical" },
            "data-disabled": (props.disabled)(),

            onfocusout: move |_| ctx.set_focus(None),
            ..props.attributes,

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TabTriggerProps {
    value: String,
    index: ReadOnlySignal<usize>,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    id: Option<String>,
    class: Option<String>,

    children: Element,
}

#[component]
pub fn TabTrigger(props: TabTriggerProps) -> Element {
    let mut ctx: TabsContext = use_context();

    use_effect(move || {
        ctx.item_count += 1;
    });

    use_drop(move || {
        ctx.item_count -= 1;
        if (ctx.current_focus)() == Some((props.index)()) {
            ctx.set_focus(None);
        }
    });

    let value = props.value.clone();
    let selected = use_memo(move || (ctx.value)() == value);

    let tab_index = use_memo(move || {
        if !(ctx.roving_focus)() {
            return "0";
        }

        if selected() {
            return "0";
        }
        if (ctx.current_focus)() == Some((props.index)()) {
            return "0";
        }
        "-1"
    });

    rsx! {
        button {
            role: "tab",
            id: props.id,
            class: props.class,
            tabindex: tab_index,

            aria_selected: selected,
            "data-state": if selected() { "active" } else { "inactive" },
            "data-disabled": (ctx.disabled)() || (props.disabled)(),
            disabled: (ctx.disabled)() || (props.disabled)(),

            onclick: move |_| {
                let value = props.value.clone();
                if !selected() {
                    ctx.set_value.call(value);
                }
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

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TabContentProps {
    value: String,

    id: Option<String>,
    class: Option<String>,

    children: Element,
}

#[component]
pub fn TabContent(props: TabContentProps) -> Element {
    let ctx: TabsContext = use_context();
    let selected = use_memo(move || (ctx.value)() == props.value);

    rsx! {
        div {
            role: "tabpanel",
            id: props.id,
            class: props.class,

            tabindex: "0",
            "data-state": if selected() { "active" } else { "inactive" },
            hidden: !selected(),

            {props.children}
        }
    }
}
