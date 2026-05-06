//! Shared state for the combobox component.

use crate::focus::FocusState;
pub(crate) use crate::selection::{OptionState, RcPartialEqValue};
use dioxus::prelude::*;

/// The default case-insensitive substring filter.
pub fn default_combobox_filter(query: &str, text: &str) -> bool {
    let query = query.trim().to_lowercase();
    query.is_empty() || text.to_lowercase().contains(&query)
}

#[derive(Clone, Copy)]
pub(super) struct ComboboxContext {
    pub open: Signal<bool>,
    pub query: Signal<String>,
    pub value: Memo<Option<RcPartialEqValue>>,
    pub set_value: Callback<Option<RcPartialEqValue>>,
    pub options: Signal<Vec<OptionState>>,
    pub list_id: Signal<Option<String>>,
    pub focus_state: FocusState,
    pub disabled: ReadSignal<bool>,
    pub filter: Callback<(String, String), bool>,
}

impl ComboboxContext {
    pub fn selected_text(&self) -> Option<String> {
        let value = self.value.read();
        let value = value.as_ref()?;
        self.options
            .read()
            .iter()
            .find(|option| &option.value == value)
            .map(|option| option.text_value.clone())
    }

    pub fn is_selected(&self, value: &RcPartialEqValue) -> bool {
        self.value.read().as_ref() == Some(value)
    }

    pub fn is_visible(&self, tab_index: usize) -> bool {
        let query = self.query.cloned();
        self.options
            .read()
            .iter()
            .find(|option| option.tab_index == tab_index)
            .is_some_and(|option| self.filter.call((query.clone(), option.text_value.clone())))
    }

    pub fn has_visible_options(&self) -> bool {
        let query = self.query.cloned();
        self.options
            .read()
            .iter()
            .any(|option| self.filter.call((query.clone(), option.text_value.clone())))
    }

    pub fn focused_visible_option_id(&self) -> Option<String> {
        let index = self.focus_state.current_focus()?;
        self.options
            .read()
            .iter()
            .find(|option| option.tab_index == index)
            .map(|option| option.id.clone())
    }

    pub fn focus_next_visible(&mut self) {
        self.focus_state.focus_next();
    }

    pub fn focus_prev_visible(&mut self) {
        self.focus_state.focus_prev();
    }

    pub fn focus_first_visible(&mut self) {
        self.focus_state.focus_first();
    }

    pub fn focus_last_visible(&mut self) {
        self.focus_state.focus_last();
    }

    pub fn select_focused(&mut self) {
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
            .find(|option| option.tab_index == index && !option.disabled && self.is_visible(index))
            .map(|option| option.value.clone());
        if let Some(value) = value {
            self.select_value(value);
        }
    }

    pub fn select_value(&mut self, value: RcPartialEqValue) {
        self.set_value.call(Some(value));
        self.open.set(false);
        self.query.set(String::new());
    }
}

#[derive(Clone, Copy)]
pub(super) struct ComboboxOptionContext {
    pub selected: ReadSignal<bool>,
}

#[derive(Clone, Copy)]
pub(super) struct ComboboxContentContext {
    pub render: ReadSignal<bool>,
}
