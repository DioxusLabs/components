//! Handle themes for Dioxus Components.

use dioxus::prelude::*;

/// The default theme.
pub(crate) const DEFAULT_THEME: &str = "theme-default";

/// A handle for retrieving the theme from the context hooks.
#[derive(Clone)]
pub struct Theme(pub String);

/// Provide a theme for this component and it's children.
/// A theme is a [`String`] that is appended to every Dioxus Component's class allowing
/// you to style your app. The signal that is returned by this hook can be mutated to change the theme.
///
/// You can use this to have sub-themes as it's value is not global.
pub fn use_theme_provider<S: ToString>(theme: S) -> Signal<Theme> {
    let theme = theme.to_string();

    let mut theme_ctx = use_context_provider(|| Signal::new(Theme(theme.clone())));
    let theme_changed = theme_ctx.read().0 != theme;
    if theme_changed  {
        theme_ctx.set(Theme(theme));
    }

    theme_ctx
}

/// Retrieve a signal for reading and writing to the theme.
pub fn use_theme() -> Signal<Theme> {
    try_use_context::<Signal<Theme>>().unwrap_or(Signal::new(Theme(DEFAULT_THEME.to_string())))
}
