use dioxus_lib::prelude::*;

// TODO: add controlled version
// TODO: Aria compatibility
// TODO: Keybinds & horizontal accordion

#[derive(Clone, Copy, Default)]
struct AccordionContext {
    /// Used to track the next runtime-generated id.
    next_id: Signal<usize>,

    /// The runtime generated ids of the open items.
    open_items: Signal<Vec<usize>>,

    /// Whether multiple items can be open at once.
    allow_multiple_open: ReadOnlySignal<bool>,

    /// Whether the entire accordion is disabled.
    disabled: ReadOnlySignal<bool>,

    /// Whether all accordion items can be collapsed.
    collapsible: ReadOnlySignal<bool>,
}

impl AccordionContext {
    pub fn new(
        allow_multiple_open: ReadOnlySignal<bool>,
        disabled: ReadOnlySignal<bool>,
        collapsible: ReadOnlySignal<bool>,
    ) -> Self {
        Self {
            next_id: Signal::new(0),
            open_items: Signal::new(Vec::new()),
            allow_multiple_open,
            disabled,
            collapsible,
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

        // If the accordion is not collapsible, we can't close this one.
        if !*self.collapsible.peek() && open_items.len() == 1 {
            return;
        }

        *open_items = open_items
            .iter()
            .cloned()
            .filter(|item| *item != id)
            .collect();
    }

    pub fn is_open(&self, id: usize) -> bool {
        self.open_items.read().contains(&id)
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

    /// Set whether the accordion is disabled.
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    /// Whether the accordion can be fully collapsed.
    ///
    /// Setting this to true will allow all accordion items to close. Defaults to true.
    #[props(default)]
    collapsible: ReadOnlySignal<bool>,
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let _ctx = use_context_provider(|| {
        AccordionContext::new(props.allow_multiple_open, props.disabled, props.collapsible)
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

    /// Whether the accordion item is disabled.
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    /// Whether this accordion item should be opened by default.
    #[props(default)]
    default_open: bool,

    /// Callback for when the accordion's open/closed state changes.
    ///
    /// The new value is provided.
    #[props(default)]
    on_change: Callback<bool, ()>,

    /// Callback for when the trigger is clicked.
    #[props(default)]
    on_trigger_click: Callback,
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    let mut ctx: AccordionContext = use_context();
    let item = use_context_provider(|| Item {
        id: ctx.unique_id(),
        disabled: props.disabled,
        on_trigger_click: props.on_trigger_click,
    });

    // Open this item if we're set as default.
    use_hook(move || {
        if props.default_open {
            ctx.set_open(item.id);
        }
    });

    // Handle calling `on_change` callback.
    use_effect(move || {
        let open = ctx.is_open(item.id);
        props.on_change.call(open)
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
                item.on_trigger_click.call(());

                // If the item is not controlled, handle state.
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
    on_trigger_click: Callback,
}

impl Item {
    pub fn is_disabled(&self) -> bool {
        (self.disabled)()
    }
}
