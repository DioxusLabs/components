//! Shared listbox popup hooks.

use dioxus::prelude::*;

use crate::{
    selection::{option_text_value, remove_option, sync_option, OptionState, RcPartialEqValue},
    use_animated_open, use_effect, use_effect_cleanup, use_id_or, use_unique_id,
};

#[derive(Clone, Copy)]
pub(crate) struct ListboxContext {
    pub(crate) render: ReadSignal<bool>,
}

#[derive(Clone, Copy)]
pub(crate) struct ListboxOptionContext {
    pub(crate) selected: ReadSignal<bool>,
}

pub(crate) fn use_listbox_id(
    id: ReadSignal<Option<String>>,
    mut list_id: Signal<Option<String>>,
) -> Memo<String> {
    let generated_id = use_unique_id();
    let id = use_id_or(generated_id, id);

    use_effect(move || {
        list_id.set(Some(id()));
    });

    id
}

pub(crate) fn use_listbox_render(
    id: impl Readable<Target = String> + Copy + 'static,
    open: impl Readable<Target = bool> + Copy + 'static,
) -> Memo<bool> {
    let render = use_animated_open(id, open);
    use_memo(render)
}

pub(crate) fn use_listbox_option<T: Clone + PartialEq + 'static>(
    id: ReadSignal<Option<String>>,
    index: ReadSignal<usize>,
    value: ReadSignal<T>,
    text_value: ReadSignal<Option<String>>,
    options: Signal<Vec<OptionState>>,
    disabled: impl Fn() -> bool + Copy + 'static,
    component_name: &'static str,
) -> Memo<String> {
    let generated_id = use_unique_id();
    let id = use_id_or(generated_id, id);
    let mut previous_id: Signal<Option<String>> = use_signal(|| None);
    let text_value =
        use_memo(move || option_text_value(&*value.read(), text_value(), component_name));

    use_effect(move || {
        let option_id = id();
        let stale_id = previous_id
            .peek()
            .as_ref()
            .filter(|stale_id| *stale_id != &option_id)
            .cloned();
        if let Some(stale_id) = stale_id {
            remove_option(options, &stale_id);
        }
        let registered_id = option_id.clone();
        sync_option(
            options,
            OptionState {
                tab_index: index(),
                value: RcPartialEqValue::new(value.cloned()),
                text_value: text_value.cloned(),
                id: registered_id,
                disabled: disabled(),
            },
        );
        previous_id.set(Some(option_id));
    });

    use_effect_cleanup(move || {
        if let Some(option_id) = previous_id.peek().as_ref() {
            remove_option(options, option_id);
        }
    });

    id
}
