use crate::{
    portal::{PortalIn, PortalOut, use_portal},
    use_unique_id,
};
use dioxus_lib::prelude::*;
use dioxus_time::use_timeout;
use std::collections::VecDeque;
use std::time::Duration;

// Toast types for different visual styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastType {
    Success,
    Error,
    Warning,
    Info,
}

impl ToastType {
    fn as_str(&self) -> &'static str {
        match self {
            ToastType::Success => "success",
            ToastType::Error => "error",
            ToastType::Warning => "warning",
            ToastType::Info => "info",
        }
    }
}

// A single toast item
#[derive(Debug, Clone, PartialEq)]
pub struct ToastItem {
    id: usize,
    title: String,
    description: Option<String>,
    toast_type: ToastType,
    duration: Option<Duration>,
    permanent: bool,
}

// Type alias for the complex callback type
type AddToastCallback = Callback<(String, Option<String>, ToastType, Option<Duration>, bool)>;

// Context for managing toasts
#[derive(Clone)]
struct ToastCtx {
    #[allow(dead_code)]
    toasts: Signal<VecDeque<ToastItem>>,
    add_toast: AddToastCallback,
    remove_toast: Callback<usize>,
}

// Toast provider props
#[derive(Props, Clone, PartialEq)]
pub struct ToastProviderProps {
    #[props(default = ReadOnlySignal::new(Signal::new(Some(Duration::from_secs(5)))))]
    default_duration: ReadOnlySignal<Option<Duration>>,

    #[props(default = ReadOnlySignal::new(Signal::new(10)))]
    max_toasts: ReadOnlySignal<usize>,

    children: Element,
}

// Toast provider component
#[component]
pub fn ToastProvider(props: ToastProviderProps) -> Element {
    let mut toasts = use_signal(VecDeque::new);
    let portal = use_portal();

    // Create context first so we can reference it in the callbacks
    let ctx = ToastCtx {
        toasts,
        add_toast: Callback::new(|_| {}),    // Temporary placeholder
        remove_toast: Callback::new(|_| {}), // Temporary placeholder
    };

    // Remove toast callback
    let remove_toast = Callback::new(move |id: usize| {
        let mut toasts_vec = toasts.write();
        if let Some(pos) = toasts_vec.iter().position(|t| t.id == id) {
            toasts_vec.remove(pos);
        }
    });

    // Add toast callback
    let add_toast = Callback::new(
        move |(title, description, toast_type, duration, permanent): (
            String,
            Option<String>,
            ToastType,
            Option<Duration>,
            bool,
        )| {
            // Generate a unique ID for the toast
            // Use a static atomic counter to ensure unique IDs
            use std::sync::atomic::{AtomicUsize, Ordering};
            static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

            // Get the current ID and increment it atomically
            let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);

            // Only use default duration for non-permanent toasts
            let duration = if permanent {
                None
            } else {
                duration.or_else(|| (props.default_duration)())
            };

            let toast = ToastItem {
                id,
                title,
                description,
                toast_type,
                duration,
                permanent,
            };

            // Add the toast directly to the queue
            // This is safe because we're in an event handler, not during render
            let mut toasts_vec = toasts.write();
            toasts_vec.push_back(toast.clone());

            // Limit the number of toasts, but prioritize keeping permanent toasts
            let max = (props.max_toasts)();
            while toasts_vec.len() > max {
                // Try to find a non-permanent toast to remove first
                if let Some(pos) = toasts_vec.iter().position(|t| !t.permanent) {
                    toasts_vec.remove(pos);
                } else {
                    // If all toasts are permanent, remove the oldest one
                    toasts_vec.pop_front();
                }
            }

            // We'll handle auto-dismissal in the Toast component
        },
    );

    // Update the context with the real callbacks
    let mut ctx = ctx;
    ctx.add_toast = add_toast;
    ctx.remove_toast = remove_toast;

    // Provide the context
    let ctx = use_context_provider(|| ctx);

    // Create a stable list of toasts for rendering outside of RSX
    let toast_list = use_memo(move || {
        let toasts_vec = toasts.read();
        toasts_vec.iter().cloned().collect::<Vec<_>>()
    });

    rsx! {
        // Render children
        {props.children}

        // Render toast container using portal
        PortalIn { portal,
            div {
                role: "region",
                aria_live: "polite",
                aria_label: "Notifications",
                class: "toast-container",

                // Render all toasts
                for toast in toast_list().iter() {
                    Toast {
                        key: format!("{}", toast.id),
                        id: toast.id,
                        title: toast.title.clone(),
                        description: toast.description.clone(),
                        toast_type: toast.toast_type,
                        permanent: toast.permanent,
                        on_close: {
                            let toast_id = toast.id;
                            let remove_toast = ctx.remove_toast;
                            move |_| {
                                remove_toast.call(toast_id);
                            }
                        },

                        // Only pass duration to non-permanent toasts
                        duration: if toast.permanent { None } else { toast.duration },
                    }
                }
            }
        }

        // Portal output at the end of the document
        PortalOut { portal }
    }
}

// Toast props
#[derive(Props, Clone, PartialEq)]
pub struct ToastProps {
    id: usize,
    title: String,
    description: Option<String>,
    toast_type: ToastType,
    on_close: Callback<MouseEvent>,
    #[props(default = false)]
    permanent: bool,

    duration: Option<Duration>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

// Toast component
#[component]
pub fn Toast(props: ToastProps) -> Element {
    let toast_id = use_unique_id();
    let id = use_memo(move || format!("toast-{}", toast_id()));

    // Get the context at the top level of the component
    let ctx = use_context::<ToastCtx>();

    // Handle auto-dismissal for non-permanent toasts with a duration
    // Double-check that the toast is not permanent and has a duration
    if !props.permanent && props.duration.is_some() {
        let duration = props.duration.unwrap();
        let toast_id = props.id;
        let remove_toast = ctx.remove_toast;

        // Create a timeout using dioxus-time
        let timeout = use_timeout(duration, move |()| {
            // Call the remove_toast function directly with the toast ID
            remove_toast.call(toast_id);
        });

        // Start the timeout when the component mounts
        use_effect(move || {
            timeout.action(());
        });
    }

    rsx! {
        div {
            id,
            role: "alert",
            class: "toast",
            "data-type": props.toast_type.as_str(),
            "data-permanent": props.permanent.to_string(),
            ..props.attributes,

            div { class: "toast-content",

                div { class: "toast-title", {props.title.clone()} }

                if let Some(description) = &props.description {
                    div { class: "toast-description", {description.clone()} }
                }
            }

            button {
                class: "toast-close",
                aria_label: "Close",
                onclick: move |e| props.on_close.call(e),
                "×"
            }
        }
    }
}

// Toast options struct for easier API
#[derive(Clone, Default)]
pub struct ToastOptions {
    pub description: Option<String>,
    pub duration: Option<Duration>,
    pub permanent: bool,
}

// Type alias for the Toasts struct
type AddToastFn = AddToastCallback;

// Simplified toast API
#[derive(Clone, Copy)]
pub struct Toasts {
    add_toast: AddToastFn,
    // We keep remove_toast for potential future use
    #[allow(dead_code)]
    remove_toast: Callback<usize>,
}

impl Toasts {
    // Show a toast with the given type and options
    pub fn show(&self, title: String, toast_type: ToastType, options: ToastOptions) {
        self.add_toast.call((
            title,
            options.description,
            toast_type,
            // If permanent, force duration to None
            if options.permanent {
                None
            } else {
                options.duration
            },
            options.permanent,
        ));
    }

    // Convenience methods for different toast types
    pub fn success(&self, title: String, options: Option<ToastOptions>) {
        self.show(title, ToastType::Success, options.unwrap_or_default());
    }

    pub fn error(&self, title: String, options: Option<ToastOptions>) {
        self.show(title, ToastType::Error, options.unwrap_or_default());
    }

    pub fn warning(&self, title: String, options: Option<ToastOptions>) {
        self.show(title, ToastType::Warning, options.unwrap_or_default());
    }

    pub fn info(&self, title: String, options: Option<ToastOptions>) {
        self.show(title, ToastType::Info, options.unwrap_or_default());
    }
}

// Hook to use the toast API
pub fn use_toast() -> Toasts {
    let ctx = use_context::<ToastCtx>();
    let add_toast = ctx.add_toast;
    let remove_toast = ctx.remove_toast;

    Toasts {
        add_toast,
        remove_toast,
    }
}
