use dioxus_lib::prelude::*;

pub mod accordion;
pub mod aspect_ratio;
pub mod checkbox;
pub mod collapsible;
pub mod label;
pub mod separator;
pub mod toggle;

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
fn use_aria_or(
    mut gen_id: Signal<String>,
    user_id: ReadOnlySignal<Option<String>>,
) -> Memo<String> {
    use_memo(move || match user_id() {
        Some(id) => {
            gen_id.set(id.clone());
            id
        }
        None => gen_id.peek().clone(),
    })
}
