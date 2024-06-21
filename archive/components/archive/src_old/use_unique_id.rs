use dioxus::prelude::*;

static CURRENT_ID: GlobalSignal<usize> = Signal::global(|| 0);
const ID_PREFIX: &str = "dxc-uniq-";

/// Generate a unique dxc-uniq-x id.
pub(crate) fn use_unique_id() -> Signal<Option<String>> {
    let mut id = use_signal(|| None);

    if id().is_none() {
        let current_id = CURRENT_ID();
        CURRENT_ID.with_mut(|x| *x += 1);

        let new_id = format!("{}{}", ID_PREFIX, current_id);
        id.set(Some(new_id));
    }

    id
}
