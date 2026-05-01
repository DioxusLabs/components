use std::{collections::BTreeMap, rc::Rc};

use dioxus::prelude::*;

use crate::use_effect_with_cleanup;

pub(crate) fn use_focus_provider(roving_loop: ReadSignal<bool>) -> FocusState {
    use_context_provider(|| {
        let recent_focus = Signal::new(None);
        let current_focus = Signal::new(None);
        let items = Signal::new(BTreeMap::new());

        FocusState {
            recent_focus,
            current_focus,
            roving_loop,
            items,
        }
    })
}

pub(crate) fn use_focus_entry_disabled(
    mut ctx: FocusState,
    index: impl Readable<Target = usize> + Copy + 'static,
    disabled: impl Fn() -> bool + Copy + 'static,
) {
    use_effect_with_cleanup(move || {
        let idx = index.cloned();
        ctx.add_update_item(idx, disabled());
        move || {
            ctx.remove_item(idx);
        }
    });
}

pub(crate) fn use_focus_control(
    ctx: FocusState,
    index: impl Readable<Target = usize> + Copy + 'static,
) -> impl FnMut(MountedEvent) {
    let disabled = || false;
    use_focus_control_disabled(ctx, index, disabled)
}

pub(crate) fn use_focus_control_disabled(
    ctx: FocusState,
    index: impl Readable<Target = usize> + Copy + 'static,
    disabled: impl Fn() -> bool + Copy + 'static,
) -> impl FnMut(MountedEvent) {
    let mut controlled_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    use_effect(move || {
        if disabled() {
            return;
        }
        ctx.control_mount_focus(index.cloned(), controlled_ref);
    });

    move |data: Event<MountedData>| controlled_ref.set(Some(data.data()))
}

pub(crate) fn use_focus_controlled_item_disabled(
    index: impl Readable<Target = usize> + Copy + 'static,
    disabled: impl Fn() -> bool + Copy + 'static,
) -> impl FnMut(MountedEvent) {
    let ctx: FocusState = use_context();
    use_focus_entry_disabled(ctx, index, disabled);
    use_focus_control_disabled(ctx, index, disabled)
}

#[derive(Clone, Copy)]
pub(crate) struct FocusState {
    pub(crate) roving_loop: ReadSignal<bool>,
    pub(crate) recent_focus: Signal<Option<usize>>,
    pub(crate) current_focus: Signal<Option<usize>>,
    items: Signal<BTreeMap<usize, bool>>,
}

impl FocusState {
    pub(crate) fn set_focus(&mut self, index: Option<usize>) {
        if let Some(idx) = index {
            self.recent_focus.set(Some(idx));
        }
        self.current_focus.set(index);
    }

    fn enabled_index_after(&self, index: usize) -> Option<usize> {
        self.items
            .read()
            .range(index.saturating_add(1)..)
            .find_map(|(&idx, &disabled)| (!disabled).then_some(idx))
    }

    fn enabled_index_before(&self, index: usize) -> Option<usize> {
        self.items
            .read()
            .range(..index)
            .rev()
            .find_map(|(&idx, &disabled)| (!disabled).then_some(idx))
    }

    pub(crate) fn first_enabled_index(&self) -> Option<usize> {
        self.items
            .read()
            .iter()
            .find_map(|(&idx, &disabled)| (!disabled).then_some(idx))
    }

    pub(crate) fn last_enabled_index(&self) -> Option<usize> {
        self.items
            .read()
            .iter()
            .rev()
            .find_map(|(&idx, &disabled)| (!disabled).then_some(idx))
    }

    pub(crate) fn focus_next(&mut self) {
        let index = match self.recent_focus() {
            Some(current) => self.enabled_index_after(current).or_else(|| {
                (self.roving_loop)()
                    .then(|| self.first_enabled_index())
                    .flatten()
            }),
            None => self.first_enabled_index(),
        };
        if let Some(index) = index {
            self.set_focus(Some(index));
        }
    }

    pub(crate) fn focus_prev(&mut self) {
        let index = match self.recent_focus() {
            Some(current) => self.enabled_index_before(current).or_else(|| {
                (self.roving_loop)()
                    .then(|| self.last_enabled_index())
                    .flatten()
            }),
            None if (self.roving_loop)() => self.last_enabled_index(),
            None => self.first_enabled_index(),
        };
        if let Some(index) = index {
            self.set_focus(Some(index));
        }
    }

    pub(crate) fn focus_first(&mut self) {
        if let Some(index) = self.first_enabled_index() {
            self.set_focus(Some(index));
        }
    }

    pub(crate) fn focus_last(&mut self) {
        if let Some(index) = self.last_enabled_index() {
            self.set_focus(Some(index));
        }
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
        self.recent_focus()
            .filter(|index| self.items.peek().get(index) == Some(&false))
            .or_else(|| self.first_enabled_index())
            .unwrap_or_default()
    }

    pub(crate) fn add_update_item(&mut self, index: usize, disabled: bool) {
        if self.items.peek().get(&index) == Some(&disabled) {
            return;
        }
        self.items.write().insert(index, disabled);
        if disabled && self.current_focus() == Some(index) {
            self.blur();
        }
    }

    pub(crate) fn remove_item(&mut self, index: usize) {
        let removed = self.items.write().remove(&index).is_some();
        if removed && (self.current_focus)() == Some(index) {
            self.set_focus(None);
        }
    }

    pub(crate) fn control_mount_focus(
        &self,
        index: usize,
        controlled_ref: Signal<Option<Rc<MountedData>>>,
    ) {
        let is_focused = self.is_focused(index);
        let is_enabled = self.items.peek().get(&index) == Some(&false);
        if is_focused && is_enabled {
            if let Some(md) = controlled_ref() {
                spawn(async move {
                    let _ = md.set_focus(true).await;
                });
            }
        }
    }
}
