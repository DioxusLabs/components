use crate::use_effect_cleanup;
use dioxus::prelude::*;
use indexmap::IndexMap;
use std::ops::ControlFlow;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) fn use_focus_provider(roving_loop: ReadSignal<bool>) -> FocusState {
    use_context_provider(|| {
        let recent_focus = Signal::new(None);
        let current_focus = Signal::new(None);
        let items = Signal::new(IndexMap::new());

        FocusState {
            recent_focus,
            current_focus,
            roving_loop,
            items,
        }
    })
}

/// If you don't already have a unique id, use this hook to generate one.
pub(crate) fn use_focus_unique_id() -> usize {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

    #[allow(unused_mut)]
    let mut initial_value = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));

    fullstack! {
        let server_id = dioxus::prelude::use_server_cached(move || {
            initial_value
        });
        initial_value = server_id;
    }
    initial_value
}

pub(crate) fn use_focus_entry(
    ctx: FocusState,
    id: usize,
    tab_index: impl Readable<Target = usize> + Copy + 'static,
) {
    let disabled = use_signal(|| false);
    use_focus_entry_disabled(ctx, id, tab_index, disabled);
}

pub(crate) fn use_focus_entry_disabled(
    mut ctx: FocusState,
    id: usize,
    tab_index: impl Readable<Target = usize> + Copy + 'static,
    disabled: impl Readable<Target = bool> + Copy + 'static,
) {
    use_effect(move || {
        ctx.add_update_item(id, tab_index, disabled);
    });
    use_effect_cleanup(move || {
        ctx.remove_item(id);
    })
}

pub(crate) fn use_focus_control(ctx: FocusState, id: usize) -> impl FnMut(MountedEvent) {
    let disabled = use_signal(|| false);
    use_focus_control_disabled(ctx, id, disabled)
}

pub(crate) fn use_focus_control_disabled(
    ctx: FocusState,
    id: usize,
    disabled: impl Readable<Target = bool> + 'static,
) -> impl FnMut(MountedEvent) {
    let mut controlled_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    use_effect(move || {
        if disabled.cloned() {
            return;
        }
        ctx.control_mount_focus(id, controlled_ref);
    });

    move |data: Event<MountedData>| controlled_ref.set(Some(data.data()))
}

pub(crate) fn use_focus_controlled_item(
    id: usize,
    index: impl Readable<Target = usize> + Copy + 'static,
) -> impl FnMut(MountedEvent) {
    let disabled = use_signal(|| false);
    use_focus_controlled_item_disabled(id, index, disabled)
}

pub(crate) fn use_focus_controlled_item_disabled(
    id: usize,
    index: impl Readable<Target = usize> + Copy + 'static,
    disabled: impl Readable<Target = bool> + Copy + 'static,
) -> impl FnMut(MountedEvent) {
    let ctx: FocusState = use_context();
    use_focus_entry_disabled(ctx, id, index, disabled);
    use_focus_control_disabled(ctx, id, disabled)
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct FocusStateElem {
    pub(crate) tab_index: usize,
    pub(crate) disabled: bool,
}

#[derive(Clone, Copy)]
pub(crate) struct FocusState {
    pub(crate) roving_loop: ReadSignal<bool>,
    /// Recent focus is only None if this State is created.
    /// It will normally hold the last focused item, even when no item is currently focused.
    /// Similar to current_focus this holds the key into the map.
    pub(crate) recent_focus: Signal<Option<usize>>,
    /// Key into the map that is currently focused.
    pub(crate) current_focus: Signal<Option<usize>>,
    pub(crate) items: Signal<IndexMap<usize, FocusStateElem>>,
}

impl FocusState {
    pub(crate) fn set_focus(&mut self, index: Option<usize>) {
        if let Some(idx) = index {
            self.recent_focus.set(Some(idx));
        }
        self.current_focus.set(index);
    }

    fn current_index(&self) -> Option<usize> {
        let current_focus = self.current_focus()?;

        self.items.read().get_index_of(&current_focus)
    }

    fn focus_enabled(&mut self, index: usize, reverse: bool) {
        let Self {
            items,
            roving_loop,
            recent_focus,
            current_focus,
        } = self;
        let items = items.read();

        let (range1, range2) = if !reverse {
            (index + 1..items.len(), 0..index)
        } else {
            (0..index, index + 1..items.len())
        };

        let iter = items.get_range(range1).into_iter().flat_map(|x| x.iter());

        let iter2 = roving_loop()
            .then(|| items.get_range(range2).into_iter().flat_map(|x| x.iter()))
            .into_iter()
            .flatten();

        let it = |(&key, value): (&usize, &FocusStateElem)| {
            if !value.disabled {
                recent_focus.set(Some(key));
                current_focus.set(Some(key));
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(())
        };

        if !reverse {
            let _ = iter.chain(iter2).try_for_each(it);
        } else {
            let _ = iter2.chain(iter).rev().try_for_each(it);
        }
    }

    pub(crate) fn focus_next(&mut self) {
        let index = match self.current_index() {
            Some(x) => x,
            None => return self.focus_first(),
        };

        self.focus_enabled(index, false);
    }

    pub(crate) fn focus_prev(&mut self) {
        let index = match self.current_index() {
            Some(x) => x,
            None => return self.focus_last(),
        };

        self.focus_enabled(index, true);
    }

    pub(crate) fn focus_first(&mut self) {
        let key = {
            self.items
                .read()
                .iter()
                .filter(|(_, value)| !value.disabled)
                .map(|(&key, _)| key)
                .next()
        };
        if let Some(key) = key {
            self.set_focus(Some(key));
        }
    }

    pub(crate) fn focus_last(&mut self) {
        let key = {
            self.items
                .read()
                .iter()
                .rev()
                .filter(|(_, value)| !value.disabled)
                .map(|(&key, _)| key)
                .next()
        };
        if let Some(key) = key {
            self.set_focus(Some(key));
        }
    }

    // pub(crate) fn focus_recent_or_first(&mut self) {
    //     if let Some(id) = self.recent_focus() {
    //         self.current_focus.set(Some(id));
    //     } else {
    //         self.focus_first();
    //     }
    // }

    pub(crate) fn focus_recent_or_first(&mut self) {
        if let Some(id) = self.recent_focus() {
            if self.items.peek().contains_key(&id) {
                self.current_focus.set(Some(id));
                return;
            }
        }
        self.focus_first();
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

    /// `id`: The unique ID of the item to be added. I suggest using props.id here.
    /// This code also runs when the tab_index or disabled changes
    pub(crate) fn add_update_item(
        &mut self,
        id: usize,
        tab_index: impl Readable<Target = usize> + Copy + 'static,
        disabled: impl Readable<Target = bool> + Copy + 'static,
    ) {
        let tab_index = tab_index.cloned();
        let disabled = disabled.cloned();

        {
            // update item if it already exists
            let mut items = self.items.write();
            let item = items.get_mut(&id);
            if let Some(item) = item {
                item.disabled = disabled;
                let changed = item.tab_index != tab_index;
                item.tab_index = tab_index;

                // if the tab_index didn't change, we don't need to refresh its order.
                if !changed {
                    return;
                }
            }
        }

        let index = self
            .items
            .peek()
            .partition_point(|_, value| value.tab_index <= tab_index);

        self.items.write().insert_before(
            index,
            id,
            FocusStateElem {
                tab_index,
                disabled,
            },
        );
    }

    pub(crate) fn item_count(&self) -> usize {
        self.items.read().len()
    }

    pub(crate) fn remove_item(&mut self, id: usize) {
        let elem = self.items.write().shift_remove_full(&id);
        if let Some((_, key, _)) = elem {
            if (self.current_focus)() == Some(key) {
                self.set_focus(None);
            }
        }
    }

    pub(crate) fn control_mount_focus(
        &self,
        id: usize,
        controlled_ref: Signal<Option<Rc<MountedData>>>,
    ) {
        let is_focused = self.is_focused(id);
        if is_focused {
            if let Some(md) = controlled_ref() {
                spawn(async move {
                    let _ = md.set_focus(true).await;
                });
            }
        }
    }
}
