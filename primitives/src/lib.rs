use dioxus_lib::prelude::*;

pub mod accordion;
pub mod aspect_ratio;
pub mod checkbox;
pub mod separator;

/// Generate a runtime-unique id.
fn use_unique_id() -> Signal<String> {
    static NEXT_ID: GlobalSignal<usize> = Signal::global(|| 0);

    use_signal(|| {
        let id = *NEXT_ID.peek();
        *NEXT_ID.write() += 1;
        format!("dxc-{id}")
    })
}
