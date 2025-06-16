use std::rc::Rc;

use dioxus::prelude::*;

use crate::use_effect_cleanup;

pub(crate) fn use_focus_provider(roving_loop: ReadOnlySignal<bool>) -> FocusState {
    use_context_provider(|| {
        let item_count = Signal::new(0);
        let recent_focus = Signal::new(0);
        let current_focus = Signal::new(None);

        FocusState {
            item_count,
            recent_focus,
            current_focus,
            roving_loop,
        }
    })
}

pub(crate) fn use_focus_entry(index: impl Readable<Target = usize> + Copy + 'static) {
    let mut ctx: FocusState = use_context();
    use_effect(move || {
        ctx.item_count += 1;
    });
    use_effect_cleanup(move || {
        ctx.item_count -= 1;
        if (ctx.current_focus)() == Some(index.cloned()) {
            ctx.set_focus(None);
        }
    });
}

pub(crate) fn use_focus_controlled_item(
    index: impl Readable<Target = usize> + Copy + 'static,
) -> impl FnMut(MountedEvent) {
    use_focus_entry(index);
    use_focus_control(index)
}

pub(crate) fn use_focus_control(
    index: impl Readable<Target = usize> + Copy + 'static,
) -> impl FnMut(MountedEvent) {
    let ctx: FocusState = use_context();
    let mut controlled_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    use_effect(move || {
        let is_focused = ctx.is_focused(index.cloned());
        if is_focused {
            if let Some(md) = controlled_ref() {
                spawn(async move {
                    let _ = md.set_focus(true).await;
                });
            }
        }
    });

    move |data: Event<MountedData>| controlled_ref.set(Some(data.data()))
}

#[derive(Clone, Copy)]
pub(crate) struct FocusState {
    pub(crate) roving_loop: ReadOnlySignal<bool>,
    pub(crate) item_count: Signal<usize>,
    pub(crate) recent_focus: Signal<usize>,
    pub(crate) current_focus: Signal<Option<usize>>,
}

impl FocusState {
    pub(crate) fn set_focus(&mut self, index: Option<usize>) {
        if let Some(idx) = index {
            self.recent_focus.set(idx);
        }
        self.current_focus.set(index);
    }

    pub(crate) fn focus_next(&mut self) {
        let current_focus = self.recent_focus();
        let mut new_focus = current_focus.saturating_add(1);

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
        let mut new_focus = current_focus.saturating_sub(1);
        if current_focus == 0 && (self.roving_loop)() {
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

    pub(crate) fn is_recent_focus(&self, id: usize) -> bool {
        let recent = (self.recent_focus)();
        recent == id
    }

    pub(crate) fn current_focus(&self) -> Option<usize> {
        (self.current_focus)()
    }

    pub(crate) fn recent_focus(&self) -> usize {
        (self.recent_focus)()
    }
}
