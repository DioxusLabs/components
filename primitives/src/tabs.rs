use crate::{
    focus::{FocusState, use_focus_controlled_item, use_focus_provider},
    use_controlled, use_id_or, use_unique_id,
};
use dioxus_lib::prelude::*;

#[derive(Clone, Copy)]
struct TabsContext {
    // State
    value: ReadOnlySignal<String>,
    set_value: Callback<String>,
    disabled: ReadOnlySignal<bool>,

    // Focus state
    focus: FocusState,

    // Orientation
    horizontal: ReadOnlySignal<bool>,
    roving_focus: ReadOnlySignal<bool>,

    // ARIA attributes
    tab_content_ids: Signal<Vec<String>>,
}

#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    value: ReadOnlySignal<Option<String>>,

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

    let focus = use_focus_provider(props.roving_loop);
    let mut ctx = use_context_provider(|| TabsContext {
        value: value.into(),
        set_value,
        disabled: props.disabled,

        focus,

        horizontal: props.horizontal,
        roving_focus: props.roving_focus,
        tab_content_ids: Signal::new(Vec::new()),
    });

    rsx! {
        div {
            "data-orientation": if (props.horizontal)() { "horizontal" } else { "vertical" },
            "data-disabled": (props.disabled)(),

            onfocusout: move |_| ctx.focus.blur(),
            ..props.attributes,

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TabListProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn TabList(props: TabListProps) -> Element {
    rsx! {
        div {
            role: "tablist",
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

    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn TabTrigger(props: TabTriggerProps) -> Element {
    let mut ctx: TabsContext = use_context();

    let value = props.value.clone();
    let selected = use_memo(move || (ctx.value)() == value);

    let tab_index = use_memo(move || {
        if !(ctx.roving_focus)() {
            return "0";
        }

        if selected() {
            return "0";
        }
        if ctx.focus.is_focused(props.index.cloned()) {
            return "0";
        }
        "-1"
    });

    let onmounted = use_focus_controlled_item(props.index.clone());

    rsx! {
        button {
            role: "tab",
            id: props.id,
            class: props.class,
            tabindex: tab_index,

            aria_selected: selected,
            aria_controls: (ctx.tab_content_ids)().get((props.index)()).cloned(),
            "data-state": if selected() { "active" } else { "inactive" },
            "data-disabled": (ctx.disabled)() || (props.disabled)(),
            disabled: (ctx.disabled)() || (props.disabled)(),

            onmounted,
            onclick: move |_| {
                let value = props.value.clone();
                if !selected() {
                    ctx.set_value.call(value);
                }
            },

            onfocus: move |_| ctx.focus.set_focus(Some((props.index)())),

            onkeydown: move |event: Event<KeyboardData>| {
                let key = event.key();
                let horizontal = (ctx.horizontal)();
                let mut prevent_default = true;
                match key {
                    Key::ArrowUp if !horizontal => ctx.focus.focus_prev(),
                    Key::ArrowDown if !horizontal => ctx.focus.focus_next(),
                    Key::ArrowLeft if horizontal => ctx.focus.focus_prev(),
                    Key::ArrowRight if horizontal => ctx.focus.focus_next(),
                    Key::Home => ctx.focus.focus_first(),
                    Key::End => ctx.focus.focus_last(),
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

#[derive(Props, Clone, PartialEq)]
pub struct TabContentProps {
    value: String,

    id: ReadOnlySignal<Option<String>>,
    class: Option<String>,

    index: ReadOnlySignal<usize>,

    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn TabContent(props: TabContentProps) -> Element {
    let mut ctx: TabsContext = use_context();
    let selected = use_memo(move || (ctx.value)() == props.value);
    let uuid = use_unique_id();
    let id = use_id_or(uuid, props.id);

    use_effect(move || {
        let mut tab_ids = ctx.tab_content_ids.write();
        let index = (props.index)();
        while tab_ids.len() <= index {
            tab_ids.push(String::new());
        }
        tab_ids[index] = id();
    });

    rsx! {
        div {
            role: "tabpanel",
            id,
            class: props.class,

            tabindex: "0",
            "data-state": if selected() { "active" } else { "inactive" },
            hidden: !selected(),
            ..props.attributes,

            {props.children}
        }
    }
}
