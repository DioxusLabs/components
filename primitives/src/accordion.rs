use crate::use_unique_id;
use dioxus_lib::prelude::*;
use std::rc::Rc;

// TODO: controlled version
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

    /// Whether the accordion is horizontal.
    horizontal: ReadOnlySignal<bool>,

    /// The focused accordion item, if any.
    focused_id: Signal<Option<usize>>,
}

impl AccordionContext {
    pub fn new(
        allow_multiple_open: ReadOnlySignal<bool>,
        disabled: ReadOnlySignal<bool>,
        collapsible: ReadOnlySignal<bool>,
        horizontal: ReadOnlySignal<bool>,
    ) -> Self {
        Self {
            next_id: Signal::new(0),
            open_items: Signal::new(Vec::new()),
            allow_multiple_open,
            disabled,
            collapsible,
            horizontal,
            focused_id: Signal::new(None),
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

    pub fn is_focused(&self, id: usize) -> bool {
        if let Some(current_id) = *self.focused_id.read() {
            return current_id == id;
        }

        false
    }

    /// Set the currently focused accordion item.
    ///
    /// This should be used by `focus`/`focusout` event only to start tracking focus.
    pub fn set_focus(&mut self, id: Option<usize>) {
        self.focused_id.set(id);
    }

    /// Focus the next accordion item.
    pub fn next_focus(&mut self) {
        let Some(id) = *self.focused_id.read() else {
            return;
        };

        let count = (self.next_id)() - 1;
        let mut next_id = id.saturating_add(1);
        if id == count {
            next_id = 0;
        }
        self.focused_id.set(Some(next_id));
    }

    /// Focus the previous accordion item.
    pub fn previous_focus(&mut self) {
        let Some(id) = *self.focused_id.read() else {
            return;
        };

        let count = (self.next_id)() - 1;
        let mut next_id = id.saturating_sub(1);
        if id == 0 {
            next_id = count;
        }
        self.focused_id.set(Some(next_id));
    }

    pub fn is_horizontal(&self) -> bool {
        (self.horizontal)()
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
    #[props(default = ReadOnlySignal::new(Signal::new(true)))]
    collapsible: ReadOnlySignal<bool>,

    /// Whether the accordion is horizontal.
    ///
    /// Settings this to true will use left/right keybinds for navigation instead of up/down. Defaults to false.
    #[props(default)]
    horizontal: ReadOnlySignal<bool>,
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let mut ctx = use_context_provider(|| {
        AccordionContext::new(
            props.allow_multiple_open,
            props.disabled,
            props.collapsible,
            props.horizontal,
        )
    });

    rsx! {
        div {
            id: props.id,
            class: props.class,
            style: props.style,
            "data-disabled": (props.disabled)(),

            onfocusout: move |_| {
                ctx.set_focus(None);
            },

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
    let aria_id = use_unique_id();

    let item = use_context_provider(|| Item {
        id: ctx.unique_id(),
        aria_id,
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
    id: ReadOnlySignal<Option<String>>,
    class: Option<String>,
    style: Option<String>,
    children: Element,
}

#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
    let mut item: Item = use_context();

    // Elements can only have one id so if the user provides their own, we must use it as the aria id.
    let id = use_memo(move || {
        let user_id = (props.id)();
        match user_id {
            Some(id) => {
                item.aria_id.set(id.clone());
                id
            }
            None => item.aria_id.peek().clone(),
        }
    });

    rsx! {
        div {
            id: id,
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

    let mut btn_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    use_effect(move || {
        let is_focused = ctx.is_focused(item.id);
        if is_focused {
            if let Some(md) = btn_ref() {
                spawn(async move {
                    let _ = md.set_focus(true).await;
                });
            }
        }
    });

    rsx! {
        button {
            id: props.id,
            class: props.class,
            style: props.style,
            disabled: is_disabled,
            tabindex: "0",
            "aria-controls": item.aria_id(),
            "aria-expanded": ctx.is_open(item.id),

            onmounted: move |data| btn_ref.set(Some(data.data())),
            onfocus: move |_| {
                ctx.set_focus(Some(item.id));
            },
            onkeydown: move |event| {
                let key = event.key();
                let horizontal = ctx.is_horizontal();

                match key {
                    Key::ArrowUp if !horizontal => ctx.previous_focus(),
                    Key::ArrowDown if !horizontal => ctx.next_focus(),
                    Key::ArrowLeft if horizontal => ctx.previous_focus(),
                    Key::ArrowRight if horizontal => ctx.next_focus(),
                    _ => {},
                };
            },

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
    aria_id: Signal<String>,
    disabled: ReadOnlySignal<bool>,
    on_trigger_click: Callback,
}

impl Item {
    pub fn is_disabled(&self) -> bool {
        (self.disabled)()
    }

    pub fn aria_id(&self) -> String {
        (self.aria_id)()
    }
}
