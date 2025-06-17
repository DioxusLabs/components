use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::{Asset, asset, manganis};
use dioxus_lib::prelude::*;

pub mod accordion;
pub mod alert_dialog;
pub mod aspect_ratio;
pub mod avatar;
pub mod calendar;
pub mod checkbox;
pub mod collapsible;
pub mod context_menu;
pub mod dialog;
pub mod dropdown_menu;
mod focus;
pub mod hover_card;
pub mod label;
pub mod menubar;
#[cfg(feature = "router")]
pub mod navbar;
pub mod popover;
mod portal;
pub mod progress;
pub mod radio_group;
pub mod scroll_area;
pub mod select;
pub mod separator;
pub mod slider;
pub mod switch;
pub mod tabs;
pub mod toast;
pub mod toggle;
pub mod toggle_group;
pub mod toolbar;
pub mod tooltip;

pub(crate) const FOCUS_TRAP_JS: Asset = asset!("/src/js/focus-trap.js");

/// Generate a runtime-unique id.
fn use_unique_id() -> Signal<String> {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

    #[allow(unused_mut)]
    let mut initial_value = use_hook(|| {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let id_str = format!("dxc-{id}");
        id_str
    });

    fullstack! {
        let server_id = dioxus::prelude::use_server_cached(move || {
            initial_value.clone()
        });
        initial_value = server_id;
    }
    use_signal(|| initial_value)
}

// Elements can only have one id so if the user provides their own, we must use it as the aria id.
fn use_id_or(mut gen_id: Signal<String>, user_id: ReadOnlySignal<Option<String>>) -> Memo<String> {
    // First, check if we have a user-provided ID
    let has_user_id = use_memo(move || user_id().is_some());

    // If we have a user ID, update the gen_id in an effect
    use_effect(move || {
        if let Some(id) = user_id() {
            gen_id.set(id);
        }
    });

    // Return the appropriate ID
    use_memo(move || {
        if has_user_id() {
            user_id().unwrap()
        } else {
            gen_id.peek().clone()
        }
    })
}

/// Allows some state to be either controlled or uncontrolled.
fn use_controlled<T: Clone + PartialEq>(
    prop: ReadOnlySignal<Option<T>>,
    default: T,
    on_change: Callback<T>,
) -> (Memo<T>, Callback<T>) {
    let mut internal_value = use_signal(|| prop.cloned().unwrap_or(default));
    let value = use_memo(move || prop.cloned().unwrap_or_else(&*internal_value));

    let set_value = use_callback(move |x: T| {
        internal_value.set(x.clone());
        on_change.call(x);
    });

    (value, set_value)
}

/// Run some cleanup code when the component is unmounted if the effect was run.
fn use_effect_cleanup<F: FnOnce() + 'static>(#[allow(unused)] cleanup: F) {
    web!(use_drop(cleanup))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContentSide {
    Top,
    Right,
    Bottom,
    Left,
}

impl ContentSide {
    fn as_str(self) -> &'static str {
        match self {
            ContentSide::Top => "top",
            ContentSide::Right => "right",
            ContentSide::Bottom => "bottom",
            ContentSide::Left => "left",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContentAlign {
    Start,
    Center,
    End,
}

impl ContentAlign {
    fn as_str(self) -> &'static str {
        match self {
            ContentAlign::Start => "start",
            ContentAlign::Center => "center",
            ContentAlign::End => "end",
        }
    }
}
