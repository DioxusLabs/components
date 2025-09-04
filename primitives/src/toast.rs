//! Defines the [`Toast`] component and its sub-components, which provide a notification system for displaying temporary messages to users.

use crate::{
    portal::{use_portal, PortalIn, PortalOut},
    use_unique_id,
};
use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use dioxus_time::use_timeout;
use std::collections::VecDeque;
use std::time::Duration;

/// Toast types for different visual styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastType {
    /// A success toast
    Success,
    /// An error toast
    Error,
    /// A warning toast
    Warning,
    /// An info toast
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
struct ToastItem {
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
    focus_region: Callback,
}

// Toast provider props
/// The props for the [`ToastProvider`] component
#[derive(Props, Clone, PartialEq)]
pub struct ToastProviderProps {
    /// The default duration for non-permanent toasts. Defaults to 5 seconds
    #[props(default = ReadOnlySignal::new(Signal::new(Some(Duration::from_secs(5)))))]
    pub default_duration: ReadOnlySignal<Option<Duration>>,

    /// The maximum number of toasts to display at once. Defaults to 10.
    #[props(default = ReadOnlySignal::new(Signal::new(10)))]
    pub max_toasts: ReadOnlySignal<usize>,

    /// The callback to render a toast. Defaults to rendering the [`Toast`] component.
    #[props(default = Callback::new(|props: ToastPropsWithOwner| rsx! { {DynamicNode::Component(props.into_vcomponent(Toast))} }))]
    pub render_toast: Callback<ToastPropsWithOwner, Element>,

    /// The children of the toast provider component.
    children: Element,
}

/// # ToastProvider
///
/// The provider component manages rendering any toasts sent by child components. This component should wrap all components that need access to the [`use_toast`] hook.
///
/// It provides a global `f6` shortcut to focus the toast region, allowing users to quickly access the most recent toast notifications.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::toast::{ToastOptions, ToastProvider, use_toast};
/// use std::time::Duration;
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         ToastProvider { ToastButton {} }
///     }
/// }
///
/// #[component]
/// fn ToastButton() -> Element {
///     let toast_api = use_toast();
///
///     rsx! {
///         button {
///             onclick: move |_| {
///                 toast_api
///                     .info(
///                         "Custom Toast".to_string(),
///                         ToastOptions::new()
///                             .description("Some info you need")
///                             .duration(Duration::from_secs(60))
///                             .permanent(false),
///                     );
///             },
///             "Info (60s)"
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`ToastProvider`] component renders toasts with the following css variables you can use to control styling:
/// - `--data-toast-count`: The number of toasts currently displayed.
#[component]
pub fn ToastProvider(props: ToastProviderProps) -> Element {
    let mut toasts = use_signal(VecDeque::new);
    let portal = use_portal();

    // Remove toast callback
    let remove_toast = use_callback(move |id: usize| {
        let mut toasts_vec = toasts.write();
        if let Some(pos) = toasts_vec.iter().position(|t: &ToastItem| t.id == id) {
            toasts_vec.remove(pos);
        }
    });

    // Add toast callback
    let add_toast = use_callback(
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

    // Create a stable list of toasts for rendering outside of RSX
    let toast_list = use_memo(move || {
        let toasts_vec = toasts.read();
        toasts_vec.iter().cloned().collect::<Vec<_>>()
    });
    let length = toast_list.len();

    let mut region_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);

    let focus_region = use_callback(move |_| {
        let Some(region_ref) = region_ref() else {
            return;
        };
        spawn(async move {
            _ = region_ref.set_focus(true).await;
        });
    });

    // Mount the first toast when the user presses f6
    use_effect(move || {
        let mut eval = dioxus::document::eval(
            "document.addEventListener('keydown', (event) => { if (event.key === 'F6') { dioxus.send(true) } });",
        );
        spawn(async move {
            while let Ok(true) = eval.recv().await {
                // Focus the first toast when F6 is pressed
                focus_region(())
            }
        });
    });

    // Provide the context
    let ctx = use_context_provider(|| ToastCtx {
        toasts,
        add_toast,
        remove_toast,
        focus_region,
    });

    rsx! {
        // Render children
        {props.children}

        // Render toast container using portal
        PortalIn { portal,
            div {
                role: "region",
                aria_label: "{length} notifications",
                tabindex: "-1",
                class: "toast-container",
                style: "--toast-count: {length}",
                onmounted: move |e| {
                    region_ref.set(Some(e.data()));
                },

                ol {
                    class: "toast-list",
                    // Render all toasts
                    for (index, toast) in toast_list.read().iter().rev().enumerate() {
                        li {
                            key: "{toast.id}",
                            class: "toast-item",
                            {
                                props.render_toast.call(ToastProps::builder().id(toast.id)
                                    .index(index)
                                    .title(toast.title.clone())
                                    .description(toast.description.clone())
                                    .toast_type(toast.toast_type)
                                    .permanent(toast.permanent)
                                    .on_close({
                                        let toast_id = toast.id;
                                        let remove_toast = ctx.remove_toast;
                                        move |_| {
                                            remove_toast.call(toast_id);
                                        }
                                    })
                                    // Only pass duration to non-permanent toasts
                                    .duration(if toast.permanent { None } else { toast.duration })
                                    .attributes(vec![])
                                    .build()
                                )
                            }
                        }
                    }
                }
            }
        }

        // Portal output at the end of the document
        PortalOut { portal }
    }
}

/// The props for the [`Toast`] component
#[derive(Props, Clone, PartialEq)]
pub struct ToastProps {
    /// The unique identifier for the toast.
    pub id: usize,
    /// The index of the toast in the list.
    pub index: usize,
    /// The title of the toast.
    pub title: String,
    /// An optional description for the toast.
    pub description: Option<String>,
    /// The type of toast.
    pub toast_type: ToastType,
    /// Callback to handle the close action of the toast.
    pub on_close: Callback<MouseEvent>,
    /// Whether the toast is permanent (not auto-dismissed).
    #[props(default = false)]
    pub permanent: bool,

    /// The duration for which the toast is displayed.
    pub duration: Option<Duration>,

    /// Additional attributes to apply to the toast element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

/// # Toast
///
/// An individual toast notification with a message for the user. This is called automatically by the [`ToastProvider`] when a toast is added if you leave
/// the default `render_toast` callback.
///
/// If you call this component manually, it must be used inside a [`ToastProvider`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::toast::{ToastOptions, ToastProvider, use_toast};
/// use std::time::Duration;
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         ToastProvider { ToastButton {} }
///     }
/// }
///
/// #[component]
/// fn ToastButton() -> Element {
///     let toast_api = use_toast();
///
///     rsx! {
///         button {
///             onclick: move |_| {
///                 toast_api
///                     .info(
///                         "Custom Toast".to_string(),
///                         ToastOptions::new()
///                             .description("Some info you need")
///                             .duration(Duration::from_secs(60))
///                     );
///             },
///             "Info (60s)"
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Toast`] component defines the following data attributes you can use to control styling:
/// - `data-type`: The type of toast. Values are `success`, `error`, `warning`, or `info`.
/// - `data-permanent`: Indicates if the toast is permanent. Values are `true` or `false`.
/// - `data-toast-even`: Present on even-indexed toasts for alternating styles.
/// - `data-toast-odd`: Present on odd-indexed toasts for alternating styles.
/// - `data-top`: Present on the topmost toast.
///
/// The [`Toast`] component renders toasts with the following css variables you can use to control styling:
/// - `--toast-index`: The index of the toast in the list, used for z-indexing and positioning.
#[component]
pub fn Toast(props: ToastProps) -> Element {
    let toast_id = use_unique_id();
    let id = use_memo(move || format!("toast-{toast_id}"));
    let label_id = format!("{id}-label");
    let description_id = props
        .description
        .as_ref()
        .map(|_| format!("{id}-description"));

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
            role: "alertdialog",
            aria_labelledby: "{label_id}",
            aria_describedby: description_id,
            aria_modal: "false",
            tabindex: "0",

            class: "toast",
            "data-type": props.toast_type.as_str(),
            "data-permanent": props.permanent,
            "data-toast-even": (props.index % 2 == 0).then_some("true"),
            "data-toast-odd": (props.index % 2 == 1).then_some("true"),
            "data-top": (props.index == 0).then_some("true"),
            style: "--toast-index: {props.index}",
            ..props.attributes,

            div { class: "toast-content",
                role: "alert",
                aria_atomic: "true",

                div {
                    id: label_id,
                    class: "toast-title",
                    {props.title.clone()}
                }

                if let Some(description) = &props.description {
                    div {
                        id: description_id.clone(),
                        class: "toast-description",
                        {description.clone()}
                    }
                }
            }

            button {
                class: "toast-close",
                aria_label: "close",
                type: "button",
                onclick: move |e| {
                    // Focus the region again after closing
                    ctx.focus_region.call(());
                    props.on_close.call(e);
                },
                "×"
            }
        }
    }
}

/// Options for customizing the behavior of toasts dispatched from the [`Toasts`] context.
#[derive(Clone, Default)]
pub struct ToastOptions {
    description: Option<String>,
    duration: Option<Duration>,
    permanent: bool,
}

impl ToastOptions {
    /// Create a new `ToastOptions` with an empty description, no duration, that is not permanent.
    pub fn new() -> Self {
        Self {
            description: None,
            duration: None,
            permanent: false,
        }
    }

    /// Set the description for the toast.
    pub fn description(mut self, description: impl ToString) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Set the duration for the toast.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set whether the toast is permanent.
    pub fn permanent(mut self, permanent: bool) -> Self {
        self.permanent = permanent;
        self
    }
}

/// The toast context provided by the [`use_toast`] hook.
#[derive(Clone, Copy)]
pub struct Toasts {
    add_toast: AddToastCallback,
    // We keep remove_toast for potential future use
    #[allow(dead_code)]
    remove_toast: Callback<usize>,
}

impl Toasts {
    /// Send a toast to the associated [`ToastProvider`] with the given title, type, and options.
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

    /// Create a new success toast with the given title and options.
    pub fn success(&self, title: String, options: ToastOptions) {
        self.show(title, ToastType::Success, options);
    }

    /// Create a new error toast with the given title and options.
    pub fn error(&self, title: String, options: ToastOptions) {
        self.show(title, ToastType::Error, options);
    }

    /// Create a new warning toast with the given title and options.
    pub fn warning(&self, title: String, options: ToastOptions) {
        self.show(title, ToastType::Warning, options);
    }

    /// Create a new info toast with the given title and options.
    pub fn info(&self, title: String, options: ToastOptions) {
        self.show(title, ToastType::Info, options);
    }
}

/// # use_toast
///
/// The `use_toast` hook provides access to the [`Toast`] api from the nearest [`ToastProvider`] which lets you
/// dispatch toasts from anywhere in your component tree.
///
/// This must be called under a [`ToastProvider`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::toast::{ToastOptions, ToastProvider, use_toast};
/// use std::time::Duration;
///
/// #[component]
/// fn ToastButton() -> Element {
///     let toast_api = use_toast();
///
///     rsx! {
///         button {
///             onclick: move |_| {
///                 toast_api
///                     .info(
///                         "Custom Toast".to_string(),
///                         ToastOptions::new()
///                             .description("Some info you need")
///                             .duration(Duration::from_secs(60))
///                             .permanent(false),
///                     );
///             },
///             "Info (60s)"
///         }
///     }
/// }
/// ```
pub fn use_toast() -> Toasts {
    use_hook(consume_toast)
}

/// Consume the toast context from the context
///
/// This must be called under a [`ToastProvider`] component.
pub fn consume_toast() -> Toasts {
    let ctx = consume_context::<ToastCtx>();
    let add_toast = ctx.add_toast;
    let remove_toast = ctx.remove_toast;

    Toasts {
        add_toast,
        remove_toast,
    }
}
