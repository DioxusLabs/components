use dioxus_lib::prelude::*;

// TODO: Convert default and add controlled version from "open"/"default_open" props on AccordionItem
// TODO: Add onvaluechange callback
// TODO: collapsible? as in, allow all items to collapse.
// TODO: horizontal accordion?
// TODO: Aria compatibility
// TODO: Keybinds

#[derive(Clone, Copy, Default)]
struct AccordionContext {
    /// Used to track the next runtime-generated id.
    next_id: Signal<usize>,

    /// The runtime generated ids of the open items.
    open_items: Signal<Vec<usize>>,

    /// Whether multiple items can be open at once.
    allow_multiple_open: ReadOnlySignal<bool>,

    /// The name of the default open item.
    default_open: ReadOnlySignal<Option<String>>,

    /// Whether the entire accordion is disabled.
    disabled: ReadOnlySignal<bool>,
}

impl AccordionContext {
    pub fn new(
        allow_multiple_open: ReadOnlySignal<bool>,
        default_open: ReadOnlySignal<Option<String>>,
        disabled: ReadOnlySignal<bool>,
    ) -> Self {
        Self {
            next_id: Signal::new(0),
            open_items: Signal::new(Vec::new()),
            allow_multiple_open,
            default_open,
            disabled,
        }
    }

    pub fn unique_id(&mut self) -> usize {
        let mut next_id = self.next_id.write();
        let id = *next_id;
        *next_id += 1;
        id
    }

    pub fn set_open(&mut self, id: usize) {
        if !*self.allow_multiple_open.peek() {
            self.open_items.clear();
        }
        self.open_items.push(id);
    }

    pub fn set_closed(&mut self, id: usize) {
        let mut open_items = self.open_items.write();
        *open_items = open_items
            .iter()
            .cloned()
            .filter(|item| *item != id)
            .collect();
    }

    pub fn is_open(&self, id: usize) -> bool {
        self.open_items.read().contains(&id)
    }

    pub fn is_default(&self, name: &str) -> bool {
        let value = self.default_open.read();
        if let Some(value) = value.as_ref() {
            return *value == name;
        }

        false
    }

    pub fn is_disabled(&self) -> bool {
        (self.disabled)()
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
    id: Option<String>,
    class: Option<String>,
    style: Option<String>,
    children: Element,

    /// Whether multiple accordion items are allowed to be open at once.
    ///
    /// Defaults to false.
    #[props(default)]
    allow_multiple_open: ReadOnlySignal<bool>,

    /// The accordion item opened by default.
    default_item: ReadOnlySignal<Option<String>>,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let _ctx = use_context_provider(|| {
        AccordionContext::new(
            props.allow_multiple_open,
            props.default_item,
            props.disabled,
        )
    });

    rsx! {
        div {
            id: props.id,
            class: props.class,
            style: props.style,
            "data-disabled": (props.disabled)(),

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AccordionItemProps {
    id: Option<String>,
    class: Option<String>,
    style: Option<String>,
    children: Element,

    /// Set the name of the accordion item.
    name: Option<String>,

    /// Set whether the accordion item is disabled.
    #[props(default)]
    disabled: ReadOnlySignal<bool>,
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    let mut ctx: AccordionContext = use_context();
    let item = use_context_provider(|| Item {
        id: ctx.unique_id(),
        disabled: props.disabled,
    });

    // Check if we're the default item.
    use_effect(move || {
        if let Some(name) = &props.name {
            let is_default = ctx.is_default(name);
            if is_default {
                ctx.set_open(item.id);
            }
        }
    });

    rsx! {
        div {
            id: props.id,
            class: props.class,
            style: props.style,
            "data-open": ctx.is_open(item.id),
            "data-disabled": ctx.is_disabled() || item.is_disabled(),

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AccordionContentProps {
    id: Option<String>,
    class: Option<String>,
    style: Option<String>,
    children: Element,
}

#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
    rsx! {
        div {
            id: props.id,
            class: props.class,
            style: props.style,

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AccordionTriggerProps {
    id: Option<String>,
    class: Option<String>,
    style: Option<String>,
    children: Element,
}

#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
    let mut ctx: AccordionContext = use_context();
    let item: Item = use_context();

    let is_disabled = ctx.is_disabled() || item.is_disabled();

    rsx! {
        button {
            id: props.id,
            class: props.class,
            style: props.style,
            disabled: is_disabled,

            onclick: move |_| {
                if is_disabled {
                    return;
                }

                match ctx.is_open(item.id) {
                    true => ctx.set_closed(item.id),
                    false => ctx.set_open(item.id),
                }
            },

            {props.children}
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Item {
    id: usize,
    disabled: ReadOnlySignal<bool>,
}

impl Item {
    pub fn is_disabled(&self) -> bool {
        (self.disabled)()
    }
}
