use dioxus_lib::prelude::*;

pub mod accordion;
pub mod aspect_ratio;
pub mod avatar;
pub mod calendar;
pub mod checkbox;
pub mod collapsible;
pub mod context_menu;
pub mod dialog;
pub mod dropdown_menu;
pub mod hover_card;
pub mod label;
pub mod menubar;
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

/// Generate a runtime-unique id.
fn use_unique_id() -> Signal<String> {
    static NEXT_ID: GlobalSignal<usize> = Signal::global(|| 0);

    let id = *NEXT_ID.peek();
    let id_str = format!("dxc-{id}");

    // Update the ID counter in an effect to avoid signal writes during rendering
    use_effect(move || {
        *NEXT_ID.write() += 1;
    });

    use_signal(|| id_str)
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
    prop: Option<Signal<T>>,
    default: T,
    on_change: Callback<T>,
) -> (Memo<T>, Callback<T>) {
    let mut internal_value = use_signal(|| prop.map(|x| x()).unwrap_or(default));
    let value = use_memo(move || prop.unwrap_or(internal_value)());

    let set_value = Callback::new(move |x: T| {
        internal_value.set(x.clone());
        on_change.call(x);
    });

    (value, set_value)
}
