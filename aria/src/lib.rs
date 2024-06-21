use dioxus::{
    dioxus_core::use_hook,
    signals::{GlobalSignal, Signal},
};

mod button;
pub use button::*;

mod alert;
pub use alert::*;

mod accordion;
pub use accordion::*;

#[derive(Clone, PartialEq)]
pub struct Icon {
    pub src: String,
    pub height: u32,
    pub width: u32,
}

static ARIA_ID_COUNT: GlobalSignal<u32> = Signal::global(|| 0);
pub(crate) fn use_aria_id() -> String {
    use_hook(|| {
        let id = ARIA_ID_COUNT();
        *ARIA_ID_COUNT.write() += 1;
        format!("dxa-aria-{}", id)
    })
}
