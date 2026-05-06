//! Shared state and behavior for select-like listbox components.

use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;

use crate::{
    focus::{use_focus_provider, FocusState},
    selection, use_controlled,
};

pub(crate) use crate::selection::{OptionState, RcPartialEqValue};

/// Whether selecting an option should replace the current value or toggle it.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum SelectionMode {
    /// A single value is selected and the popup closes after selection.
    Single,
    /// Multiple values can be selected and the popup stays open after selection.
    Multiple,
}

impl SelectionMode {
    pub(crate) fn is_multiple(self) -> bool {
        matches!(self, Self::Multiple)
    }

    fn closes_on_select(self) -> bool {
        matches!(self, Self::Single)
    }
}

/// Shared context for components built around a selectable listbox.
#[derive(Clone, Copy)]
pub(crate) struct SelectableContext {
    pub(crate) open: Memo<bool>,
    pub(crate) set_open: Callback<bool>,
    pub(crate) values: Memo<Vec<RcPartialEqValue>>,
    pub(crate) set_value: Callback<Option<RcPartialEqValue>>,
    pub(crate) selection_mode: SelectionMode,
    pub(crate) options: Signal<Vec<OptionState>>,
    pub(crate) list_id: Signal<Option<String>>,
    pub(crate) focus_state: FocusState,
    pub(crate) disabled: ReadSignal<bool>,
}

impl SelectableContext {
    pub(crate) fn set_open(&mut self, open: bool) {
        self.set_open.call(open);
    }

    pub(crate) fn toggle_open(&mut self) {
        self.set_open(!self.open.cloned());
    }

    pub(crate) fn selected_text(&self) -> Option<String> {
        let values = self.values.read();
        let options = self.options.read();
        selection::selected_text(values.iter(), &options)
    }

    pub(crate) fn is_selected(&self, value: &RcPartialEqValue) -> bool {
        self.values.read().iter().any(|selected| selected == value)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.values.read().is_empty()
    }

    pub(crate) fn focused_option_id_where(
        &self,
        predicate: impl Fn(&OptionState) -> bool,
    ) -> Option<String> {
        let index = self.focus_state.current_focus()?;
        self.options
            .read()
            .iter()
            .find(|option| option.tab_index == index && !option.disabled && predicate(option))
            .map(|option| option.id.clone())
    }

    pub(crate) fn select_focused_where(&mut self, predicate: impl Fn(&OptionState) -> bool) {
        if !self.open.cloned() {
            return;
        }
        let Some(index) = self.focus_state.current_focus() else {
            return;
        };
        let value = self
            .options
            .read()
            .iter()
            .find(|option| option.tab_index == index && !option.disabled && predicate(option))
            .map(|option| option.value.clone());
        if let Some(value) = value {
            self.select_value(value);
        }
    }

    pub(crate) fn select_focused(&mut self) {
        self.select_focused_where(|_| true);
    }

    pub(crate) fn select_value(&mut self, value: RcPartialEqValue) {
        self.set_value.call(Some(value));
        if self.selection_mode.closes_on_select() {
            self.set_open(false);
        }
    }
}

pub(crate) fn use_selectable_root(
    values: Memo<Vec<RcPartialEqValue>>,
    set_value: Callback<Option<RcPartialEqValue>>,
    selection_mode: SelectionMode,
    disabled: ReadSignal<bool>,
    roving_loop: ReadSignal<bool>,
    open: ReadSignal<Option<bool>>,
    default_open: bool,
    on_open_change: Callback<bool>,
) -> SelectableContext {
    let (open, set_open) = use_controlled(open, default_open, on_open_change);
    let options: Signal<Vec<OptionState>> = use_signal(Vec::default);
    let list_id = use_signal(|| None);
    let focus_state = use_focus_provider(roving_loop);

    SelectableContext {
        open,
        set_open,
        values,
        set_value,
        selection_mode,
        options,
        list_id,
        focus_state,
        disabled,
    }
}

pub(crate) fn pointer_select_start(
    event: &Event<PointerData>,
    disabled: bool,
    mut down_pos: Signal<Option<(f64, f64)>>,
) {
    if disabled || event.trigger_button() != Some(MouseButton::Primary) {
        return;
    }
    event.prevent_default();
    let p = event.client_coordinates();
    down_pos.set(Some((p.x, p.y)));
}

pub(crate) fn pointer_select_commit(
    event: &Event<PointerData>,
    disabled: bool,
    mut down_pos: Signal<Option<(f64, f64)>>,
) -> bool {
    if disabled || event.trigger_button() != Some(MouseButton::Primary) {
        return false;
    }
    let Some((x0, y0)) = down_pos.take() else {
        return false;
    };
    if event.pointer_type() == "touch" {
        let p = event.client_coordinates();
        let dx = p.x - x0;
        let dy = p.y - y0;
        if dx * dx + dy * dy > 25.0 {
            return false;
        }
    }
    true
}

pub(crate) fn pointer_select_cancel(mut down_pos: Signal<Option<(f64, f64)>>) {
    down_pos.set(None);
}
