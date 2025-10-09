use dioxus::prelude::*;
use dioxus_primitives::avatar::{self, AvatarFallbackProps, AvatarImageProps, AvatarState};

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AvatarImageSize {
    #[default]
    Small,
    Medium,
    Large,
}

impl AvatarImageSize {
    fn to_class(self) -> &'static str {
        match self {
            AvatarImageSize::Small => "avatar-sm",
            AvatarImageSize::Medium => "avatar-md",
            AvatarImageSize::Large => "avatar-lg",
        }
    }
}

/// The props for the [`Avatar`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    /// Callback when image loads successfully
    #[props(default)]
    pub on_load: Option<EventHandler<()>>,

    /// Callback when image fails to load
    #[props(default)]
    pub on_error: Option<EventHandler<()>>,

    /// Callback when the avatar state changes
    #[props(default)]
    pub on_state_change: Option<EventHandler<AvatarState>>,

    #[props(default)]
    pub size: AvatarImageSize,

    /// Additional attributes for the avatar element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the Avatar component, which can include AvatarImage and AvatarFallback
    pub children: Element,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        avatar::Avatar {
            class: "avatar {props.size.to_class()}",
            on_load: props.on_load,
            on_error: props.on_error,
            on_state_change: props.on_state_change,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AvatarImage(props: AvatarImageProps) -> Element {
    rsx! {
        avatar::AvatarImage {
            class: "avatar-image",
            src: props.src,
            alt: props.alt,
            attributes: props.attributes,
        }
    }
}

#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
    rsx! {
        avatar::AvatarFallback { class: "avatar-fallback", attributes: props.attributes, {props.children} }
    }
}
