use dioxus_lib::prelude::*;
use std::rc::Rc;

#[derive(Clone, Copy)]
struct ToolbarCtx {
    // State
    disabled: ReadOnlySignal<bool>,

    // Focus management
    focused_index: Signal<Option<usize>>,

    // Orientation
    horizontal: ReadOnlySignal<bool>,
}

impl ToolbarCtx {
    fn set_focus(&mut self, index: Option<usize>) {
        self.focused_index.set(index);
    }

    fn is_focused(&self, index: usize) -> bool {
        (self.focused_index)() == Some(index)
    }

    fn orientation(&self) -> &'static str {
        if (self.horizontal)() {
            "horizontal"
        } else {
            "vertical"
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ToolbarProps {
    /// Whether the toolbar is disabled
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    /// Whether the toolbar is horizontal (true) or vertical (false)
    #[props(default = ReadOnlySignal::new(Signal::new(true)))]
    horizontal: ReadOnlySignal<bool>,

    /// ARIA label for the toolbar
    #[props(default)]
    aria_label: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Toolbar(props: ToolbarProps) -> Element {
    let mut ctx = use_context_provider(|| ToolbarCtx {
        disabled: props.disabled,
        focused_index: Signal::new(None),
        horizontal: props.horizontal,
    });

    rsx! {
        div {
            role: "toolbar",
            "data-orientation": ctx.orientation(),
            "data-disabled": (props.disabled)(),
            aria_label: props.aria_label,

            onfocusout: move |_| ctx.set_focus(None),
            ..props.attributes,

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ToolbarButtonProps {
    /// Index of the button in the toolbar
    index: ReadOnlySignal<usize>,

    /// Whether the button is disabled
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    /// Callback when the button is clicked
    #[props(default)]
    on_click: Callback<()>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn ToolbarButton(props: ToolbarButtonProps) -> Element {
    let mut ctx: ToolbarCtx = use_context();

    // Handle button ref for focus management
    let mut button_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    // Check if this button is focused
    let is_focused = use_memo(move || ctx.is_focused((props.index)()));

    // Set focus when needed
    use_effect(move || {
        if is_focused() {
            if let Some(md) = button_ref() {
                spawn(async move {
                    let _ = md.set_focus(true).await;
                });
            }
        }
    });

    rsx! {
        button {
            r#type: "button",
            tabindex: "0",
            disabled: (ctx.disabled)() || (props.disabled)(),
            "data-disabled": (ctx.disabled)() || (props.disabled)(),

            onmounted: move |data: Event<MountedData>| button_ref.set(Some(data.data())),
            onfocus: move |_| ctx.set_focus(Some((props.index)())),

            onclick: move |_| {
                if !(ctx.disabled)() && !(props.disabled)() {
                    props.on_click.call(());
                }
            },

            onkeydown: move |event: Event<KeyboardData>| {
                let key = event.key();
                let horizontal = (ctx.horizontal)();
                let mut prevent_default = true;
                match key {
                    Key::ArrowUp if !horizontal => {
                        let index = (props.index)();
                        if index > 0 {
                            ctx.set_focus(Some(index - 1));
                        }
                    }
                    Key::ArrowDown if !horizontal => {
                        let index = (props.index)();
                        ctx.set_focus(Some(index + 1));
                    }
                    Key::ArrowLeft if horizontal => {
                        let index = (props.index)();
                        if index > 0 {
                            ctx.set_focus(Some(index - 1));
                        }
                    }
                    Key::ArrowRight if horizontal => {
                        let index = (props.index)();
                        ctx.set_focus(Some(index + 1));
                    }
                    Key::Home => {
                        ctx.set_focus(Some(0));
                    }
                    Key::End => {
                        ctx.set_focus(Some(100));
                    }
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
pub struct ToolbarSeparatorProps {
    /// Whether the separator is horizontal (true) or vertical (false)
    #[props(default)]
    horizontal: Option<bool>,

    /// If the separator is decorative and should not be classified
    /// as a separator to the ARIA standard.
    #[props(default = false)]
    decorative: bool,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn ToolbarSeparator(props: ToolbarSeparatorProps) -> Element {
    let ctx: ToolbarCtx = use_context();

    // If horizontal is explicitly set, use that, otherwise invert the toolbar orientation
    let horizontal = props.horizontal.unwrap_or(!(ctx.horizontal)());

    let orientation = match horizontal {
        true => "horizontal",
        false => "vertical",
    };

    rsx! {
        div {
            role: if !props.decorative { "separator" } else { "none" },
            aria_orientation: if !props.decorative { orientation },
            "data-orientation": orientation,
            ..props.attributes,
        }
    }
}
