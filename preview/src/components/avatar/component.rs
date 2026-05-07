use dioxus::prelude::*;
use dioxus_primitives::avatar::{self, AvatarFallbackProps, AvatarImageProps, AvatarState};

#[css_module("/src/components/avatar/style.css")]
struct Styles;

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
            AvatarImageSize::Small => Styles::dx_avatar_sm,
            AvatarImageSize::Medium => Styles::dx_avatar_md,
            AvatarImageSize::Large => Styles::dx_avatar_lg,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AvatarShape {
    #[default]
    Circle,
    Rounded,
}

impl AvatarShape {
    fn to_class(self) -> &'static str {
        match self {
            AvatarShape::Circle => Styles::dx_avatar_circle,
            AvatarShape::Rounded => Styles::dx_avatar_rounded,
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

    #[props(default)]
    pub shape: AvatarShape,

    /// Additional attributes for the avatar element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the Avatar component, which can include AvatarImage and AvatarFallback
    pub children: Element,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    rsx! {
        avatar::Avatar {
            class: Styles::dx_avatar,
            class: props.size.to_class(),
            class: props.shape.to_class(),
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
            class: Styles::dx_avatar_image,
            src: props.src,
            alt: props.alt,
            attributes: props.attributes,
        }
    }
}

#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
    rsx! {
        avatar::AvatarFallback { class: Styles::dx_avatar_fallback, attributes: props.attributes, {props.children} }
    }
}
