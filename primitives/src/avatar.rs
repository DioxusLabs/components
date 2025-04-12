use dioxus_lib::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    /// The image source URL
    src: Option<String>,

    /// Alt text for the image
    alt: Option<String>,

    /// Fallback text to display when image fails to load or no src provided
    fallback: Option<Element>,

    /// Whether the image has loaded
    #[props(default = Signal::new(false))]
    loaded: Signal<bool>,

    /// Callback when image loads successfully
    #[props(default)]
    on_load: Option<EventHandler<()>>,

    /// Callback when image fails to load
    #[props(default)]
    on_error: Option<EventHandler<()>>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let mut loaded = props.loaded;

    rsx! {
        span {
            role: "img",
            "data-state": if loaded() { "loaded" } else { "loading" },
            ..props.attributes,

            // Show image if src is provided
            if let Some(src) = &props.src {
                img {
                    src: src.clone(),
                    alt: props.alt.clone().unwrap_or_default(),
                    onload: move |_| {
                        loaded.set(true);
                        if let Some(handler) = &props.on_load {
                            handler.call(());
                        }
                    },
                    onerror: move |_| {
                        loaded.set(false);
                        if let Some(handler) = &props.on_error {
                            handler.call(());
                        }
                    },
                    style: "width: 100%; height: 100%; object-fit: cover;",
                }
            }

            // Show fallback if no image or loading failed
            if !loaded() {
                if let Some(fallback) = &props.fallback {
                    {fallback}
                } else {
                    // Default fallback showing first two letters
                    span { style: "display: flex; align-items: center; justify-content: center; width: 100%; height: 100%;",
                        if let Some(alt) = &props.alt {
                            {alt.chars().take(2).collect::<String>().to_uppercase()}
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AvatarFallbackProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
    rsx! {
        span { ..props.attributes,{props.children} }
    }
}
