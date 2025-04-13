use crate::{use_controlled, use_id_or, use_unique_id};
use dioxus_lib::prelude::*;

// Context for the Select component
#[derive(Clone, Copy)]
struct SelectCtx {
    // State
    open: ReadOnlySignal<bool>,
    set_open: Callback<bool>,
    value: ReadOnlySignal<Option<String>>,
    set_value: Callback<Option<String>>,
    disabled: ReadOnlySignal<bool>,

    // ARIA attributes
    trigger_id: Signal<String>,
    content_id: Signal<String>,
    label_id: Signal<String>,

    // Keyboard navigation
    item_count: Signal<usize>,
    recent_focus: Signal<usize>,
    current_focus: Signal<Option<usize>>,
}

impl SelectCtx {
    fn set_focus(&mut self, index: Option<usize>) {
        if let Some(idx) = index {
            self.recent_focus.set(idx);
        }
        self.current_focus.set(index);
    }

    fn focus_next(&mut self) {
        let count = *self.item_count.read();
        if count == 0 {
            return;
        }

        let next = match *self.current_focus.read() {
            Some(current) => (current + 1) % count,
            None => 0,
        };
        self.set_focus(Some(next));
    }

    fn focus_prev(&mut self) {
        let count = *self.item_count.read();
        if count == 0 {
            return;
        }

        let prev = match *self.current_focus.read() {
            Some(current) => {
                if current == 0 {
                    count - 1
                } else {
                    current - 1
                }
            }
            None => count - 1,
        };
        self.set_focus(Some(prev));
    }

    fn focus_first(&mut self) {
        if *self.item_count.read() > 0 {
            self.set_focus(Some(0));
        }
    }

    fn focus_last(&mut self) {
        let count = *self.item_count.read();
        if count > 0 {
            self.set_focus(Some(count - 1));
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    /// The controlled value of the select
    #[props(default)]
    value: Option<Signal<Option<String>>>,

    /// The default value of the select
    #[props(default)]
    default_value: Option<String>,

    /// Callback when the value changes
    #[props(default)]
    on_value_change: Callback<Option<String>>,

    /// Whether the select is open
    open: Option<Signal<bool>>,

    /// Default open state
    #[props(default)]
    default_open: bool,

    /// Callback when open state changes
    #[props(default)]
    on_open_change: Callback<bool>,

    /// Whether the select is disabled
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    /// Whether the select is required
    #[props(default)]
    required: ReadOnlySignal<bool>,

    /// Name of the select for form submission
    #[props(default)]
    name: ReadOnlySignal<String>,

    /// Optional ID for the select
    #[props(default)]
    id: ReadOnlySignal<Option<String>>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    let (value, set_value) =
        use_controlled(props.value, props.default_value, props.on_value_change);
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    // Generate unique IDs for accessibility
    let trigger_id = use_unique_id();
    let content_id = use_unique_id();
    let label_id = use_unique_id();

    let mut ctx = use_context_provider(|| SelectCtx {
        open: open.into(),
        set_open,
        value: value.into(),
        set_value,
        disabled: props.disabled,

        trigger_id,
        content_id,
        label_id,

        item_count: Signal::new(0),
        recent_focus: Signal::new(0),
        current_focus: Signal::new(None),
    });

    // Generate or use provided ID
    let gen_id = use_unique_id();
    let id = use_id_or(gen_id, props.id);

    rsx! {
        div {
            id: id,
            class: "select",
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": (props.disabled)(),

            // Handle focus out to close the select
            onfocusout: move |_: Event<FocusData>| {
                // We'll use a simple approach - close the select on any focus out
                // In a real implementation, we would check if focus is still within the select
                ctx.set_open.call(false);
                ctx.set_focus(None);
            },

            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectTriggerProps {
    /// Optional placeholder text when no value is selected
    #[props(default = "Select an option")]
    placeholder: &'static str,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SelectTrigger(props: SelectTriggerProps) -> Element {
    let mut ctx: SelectCtx = use_context();

    // Handle click to toggle the select
    let handle_click = move |_| {
        if !(ctx.disabled)() {
            let new_open = !(ctx.open)();
            ctx.set_open.call(new_open);

            // If opening, reset focus
            if new_open {
                ctx.set_focus(None);
            }
        }
    };

    // Handle keyboard events
    let handle_keydown = move |event: Event<KeyboardData>| {
        if (ctx.disabled)() {
            return;
        }

        let mut prevent_default = true;
        match event.key() {
            Key::Enter => {
                let new_open = !(ctx.open)();
                ctx.set_open.call(new_open);

                // If opening, focus the first item
                if new_open {
                    ctx.focus_first();
                }
            }
            Key::ArrowDown => {
                if !(ctx.open)() {
                    ctx.set_open.call(true);
                    ctx.focus_first();
                } else {
                    ctx.focus_next();
                }
            }
            Key::ArrowUp => {
                if !(ctx.open)() {
                    ctx.set_open.call(true);
                    ctx.focus_last();
                } else {
                    ctx.focus_prev();
                }
            }
            Key::Escape => {
                if (ctx.open)() {
                    ctx.set_open.call(false);
                }
            }
            _ => prevent_default = false,
        }

        if prevent_default {
            event.prevent_default();
        }
    };

    rsx! {
        button {
            id: ctx.trigger_id.peek().clone(),
            class: "select-trigger",
            type: "button",
            role: "combobox",
            aria_expanded: ctx.open,
            aria_labelledby: ctx.label_id,
            aria_controls: ctx.content_id,
            aria_required: "false", // TODO: Add required prop
            "data-state": if (ctx.open)() { "open" } else { "closed" },
            "data-disabled": (ctx.disabled)(),
            "data-placeholder": if (ctx.value)().is_none() { "true" } else { "false" },
            disabled: (ctx.disabled)(),

            onclick: handle_click,
            onkeydown: handle_keydown,

            ..props.attributes,

            // Show either the selected value or the placeholder
            if let Some(value) = (ctx.value)() {
                span { class: "select-value", {value} }
            } else {
                span { class: "select-placeholder", {props.placeholder} }
            }

            // Render children (usually an icon)
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectValueProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SelectValue(props: SelectValueProps) -> Element {
    let ctx: SelectCtx = use_context();

    // Only render if a value is selected
    if (ctx.value)().is_none() {
        return rsx!({});
    }

    rsx! {
        span {
            class: "select-value",
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectContentProps {
    /// Optional position of the content
    #[props(default = "bottom")]
    position: &'static str,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SelectContent(props: SelectContentProps) -> Element {
    let mut ctx: SelectCtx = use_context();

    // Only render if the select is open
    let is_open = (ctx.open)();
    if !is_open {
        return rsx!({});
    }

    rsx! {
        div {
            id: ctx.content_id.peek().clone(),
            class: "select-content",
            role: "listbox",
            "data-state": if is_open { "open" } else { "closed" },
            "data-position": props.position,

            // Handle keyboard navigation
            onkeydown: move |event: Event<KeyboardData>| {
                let mut prevent_default = true;
                match event.key() {
                    Key::ArrowDown => ctx.focus_next(),
                    Key::ArrowUp => ctx.focus_prev(),
                    Key::Home => ctx.focus_first(),
                    Key::End => ctx.focus_last(),
                    Key::Escape => {
                        ctx.set_open.call(false);
                    }
                    Key::Enter => {
                        // Select the currently focused item
                        if let Some(_index) = (ctx.current_focus)() {
                            // This is a simplified approach - in a real implementation,
                            // we would need to get the value from the focused item
                            // For now, we'll just close the select
                            ctx.set_open.call(false);
                        }
                    }
                    _ => prevent_default = false,
                }

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
pub struct SelectItemProps {
    /// The value of the item
    value: String,

    /// The index of the item in the list
    index: usize,

    /// Whether the item is disabled
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SelectItem(props: SelectItemProps) -> Element {
    let mut ctx: SelectCtx = use_context();

    // Register this item with the select
    use_effect(move || {
        ctx.item_count += 1;
    });

    // Determine if this item is selected
    let value_for_selected = props.value.clone();
    let selected = use_memo(move || (ctx.value)() == Some(value_for_selected.clone()));

    // Determine if this item is currently focused
    let tab_index = use_memo(move || {
        if (ctx.current_focus)() == Some(props.index) {
            "0"
        } else {
            "-1"
        }
    });

    // Handle click to select this item
    let value_for_click = props.value.clone();
    let handle_click = move |_| {
        if !(ctx.disabled)() && !(props.disabled)() {
            // Set the value and close the select
            ctx.set_value.call(Some(value_for_click.clone()));
            ctx.set_open.call(false);

            // No logging, but we'll make sure the value is set correctly
        }
    };

    // Handle keyboard events
    let value_for_keydown = props.value.clone();
    let handle_keydown = move |event: Event<KeyboardData>| {
        if (ctx.disabled)() || (props.disabled)() {
            return;
        }

        let mut prevent_default = true;
        match event.key() {
            Key::Enter => {
                ctx.set_value.call(Some(value_for_keydown.clone()));
                ctx.set_open.call(false);
            }
            _ => prevent_default = false,
        }

        if prevent_default {
            event.prevent_default();
        }
    };

    rsx! {
        div {
            class: "select-item",
            role: "option",
            tabindex: tab_index,
            "data-state": if selected() { "selected" } else { "unselected" },
            "data-disabled": (ctx.disabled)() || (props.disabled)(),
            "data-highlighted": (ctx.current_focus)() == Some(props.index),
            aria_selected: selected,

            onclick: handle_click,
            onkeydown: handle_keydown,
            onfocus: move |_| ctx.set_focus(Some(props.index)),

            ..props.attributes,

            // Item content
            {props.children}

            // Selected indicator
            if selected() {
                span { class: "select-item-indicator", "✓" }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectLabelProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SelectLabel(props: SelectLabelProps) -> Element {
    let ctx: SelectCtx = use_context();

    rsx! {
        label {
            id: ctx.label_id.peek().clone(),
            class: "select-label",
            for: ctx.trigger_id.peek().clone(),
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectGroupProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SelectGroup(props: SelectGroupProps) -> Element {
    rsx! {
        div {
            class: "select-group",
            role: "group",
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectSeparatorProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn SelectSeparator(props: SelectSeparatorProps) -> Element {
    rsx! {
        div {
            class: "select-separator",
            role: "separator",
            ..props.attributes,
        }
    }
}
