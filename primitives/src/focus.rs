use std::rc::Rc;

use dioxus::prelude::*;

use crate::use_effect_cleanup;

pub(crate) fn use_focus_provider(roving_loop: ReadOnlySignal<bool>) -> FocusState {
    use_context_provider(|| {
        let item_count = Signal::new(0);
        let recent_focus = Signal::new(None);
        let current_focus = Signal::new(None);

        FocusState {
            item_count,
            recent_focus,
            current_focus,
            roving_loop,
        }
    })
}

pub(crate) fn use_focus_entry(
    ctx: FocusState,
    index: impl Readable<Target = usize> + Copy + 'static,
) {
    let disabled = use_signal(|| false);
    use_focus_entry_disabled(ctx, index, disabled);
}

pub(crate) fn use_focus_entry_disabled(
    mut ctx: FocusState,
    index: impl Readable<Target = usize> + Copy + 'static,
    disabled: impl Readable<Target = bool> + 'static,
) {
    let mut item = use_hook(|| CopyValue::new(false));
    use_effect(move || {
        if disabled.cloned() {
            if item.cloned() {
                ctx.remove_item(index.cloned());
                item.set(false);
            }
        } else {
            ctx.add_item();
            item.set(true);
        }
    });
    use_effect_cleanup(move || {
        if item.cloned() {
            ctx.remove_item(index.cloned());
        }
    });
}

pub(crate) fn use_focus_control(
    ctx: FocusState,
    index: impl Readable<Target = usize> + Copy + 'static,
) -> impl FnMut(MountedEvent) {
    let disabled = use_signal(|| false);
    use_focus_control_disabled(ctx, index, disabled)
}

pub(crate) fn use_focus_control_disabled(
    ctx: FocusState,
    index: impl Readable<Target = usize> + Copy + 'static,
    disabled: impl Readable<Target = bool> + 'static,
) -> impl FnMut(MountedEvent) {
    let mut controlled_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    use_effect(move || {
        if disabled.cloned() {
            return;
        }
        ctx.control_mount_focus(index.cloned(), controlled_ref);
    });

    move |data: Event<MountedData>| controlled_ref.set(Some(data.data()))
}

pub(crate) fn use_focus_controlled_item(
    index: impl Readable<Target = usize> + Copy + 'static,
) -> impl FnMut(MountedEvent) {
    let disabled = use_signal(|| false);
    use_focus_controlled_item_disabled(index, disabled)
}

pub(crate) fn use_focus_controlled_item_disabled(
    index: impl Readable<Target = usize> + Copy + 'static,
    disabled: impl Readable<Target = bool> + Copy + 'static,
) -> impl FnMut(MountedEvent) {
    let ctx: FocusState = use_context();
    use_focus_entry_disabled(ctx, index, disabled);
    use_focus_control_disabled(ctx, index, disabled)
}

#[derive(Clone, Copy)]
pub(crate) struct FocusState {
    pub(crate) roving_loop: ReadOnlySignal<bool>,
    pub(crate) item_count: Signal<usize>,
    pub(crate) recent_focus: Signal<Option<usize>>,
    pub(crate) current_focus: Signal<Option<usize>>,
}

impl FocusState {
    pub(crate) fn set_focus(&mut self, index: Option<usize>) {
        if let Some(idx) = index {
            self.recent_focus.set(Some(idx));
        }
        self.current_focus.set(index);
    }

    pub(crate) fn focus_next(&mut self) {
        let current_focus = self.recent_focus();
        let mut new_focus = current_focus
            .map(|x| x.saturating_add(1))
            .unwrap_or_default();

        let item_count = (self.item_count)();
        if new_focus >= item_count {
            match (self.roving_loop)() {
                true => new_focus = 0,
                false => new_focus = item_count.saturating_sub(1),
            }
        }

        self.set_focus(Some(new_focus));
    }

    pub(crate) fn focus_prev(&mut self) {
        let current_focus = self.recent_focus();
        let mut new_focus = current_focus
            .map(|x| x.saturating_sub(1))
            .unwrap_or_default();
        if current_focus.unwrap_or_default() == 0 && (self.roving_loop)() {
            new_focus = (self.item_count)().saturating_sub(1);
        }

        self.set_focus(Some(new_focus));
    }

    pub(crate) fn focus_first(&mut self) {
        self.set_focus(Some(0));
    }

    pub(crate) fn focus_last(&mut self) {
        let last_index = self.item_count.cloned() - 1;
        self.set_focus(Some(last_index));
    }

    pub(crate) fn blur(&mut self) {
        self.set_focus(None);
    }

    pub(crate) fn any_focused(&self) -> bool {
        self.current_focus.read().is_some()
    }

    pub(crate) fn is_focused(&self, id: usize) -> bool {
        (self.current_focus)().map(|x| x == id).unwrap_or(false)
    }

    pub(crate) fn current_focus(&self) -> Option<usize> {
        (self.current_focus)()
    }

    pub(crate) fn recent_focus(&self) -> Option<usize> {
        (self.recent_focus)()
    }

    pub(crate) fn recent_focus_or_default(&self) -> usize {
        (self.recent_focus)().unwrap_or_default()
    }

    pub(crate) fn add_item(&mut self) {
        self.item_count += 1;
    }

    pub(crate) fn remove_item(&mut self, index: usize) {
        self.item_count -= 1;
        if (self.current_focus)() == Some(index) {
            self.set_focus(None);
        }
    }

    pub(crate) fn control_mount_focus(
        &self,
        index: usize,
        controlled_ref: Signal<Option<Rc<MountedData>>>,
    ) {
        let is_focused = self.is_focused(index);
        if is_focused {
            if let Some(md) = controlled_ref() {
                spawn(async move {
                    let _ = md.set_focus(true).await;
                });
            }
        }
    }
}
