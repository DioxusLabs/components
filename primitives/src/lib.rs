use dioxus_lib::prelude::*;

pub mod accordion;
pub mod aspect_ratio;
pub mod checkbox;
pub mod collapsible;
pub mod dialog;
pub mod label;
mod portal;
pub mod progress;
pub mod separator;
pub mod slider;
pub mod switch;
pub mod toast;
pub mod toggle;
pub mod toggle_group;

/// Generate a runtime-unique id.
fn use_unique_id() -> Signal<String> {
    static NEXT_ID: GlobalSignal<usize> = Signal::global(|| 0);

    use_signal(|| {
        let id = *NEXT_ID.peek();
        *NEXT_ID.write() += 1;
        format!("dxc-{id}")
    })
}

// Elements can only have one id so if the user provides their own, we must use it as the aria id.
fn use_id_or(mut gen_id: Signal<String>, user_id: ReadOnlySignal<Option<String>>) -> Memo<String> {
    use_memo(move || match user_id() {
        Some(id) => {
            gen_id.set(id.clone());
            id
        }
        None => gen_id.peek().clone(),
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
