//! Shared state for the combobox component.

pub(crate) use crate::selectable::RcPartialEqValue;
use crate::selectable::SelectableContext;
use dioxus::prelude::*;

/// The default case-insensitive substring filter.
pub fn default_combobox_filter(query: &str, text: &str) -> bool {
    let query = query.trim().to_lowercase();
    query.is_empty() || text.to_lowercase().contains(&query)
}

#[derive(Clone, Copy)]
pub(super) struct ComboboxContext {
    pub selectable: SelectableContext,
    pub query: Memo<String>,
    pub set_query: Callback<String>,
    pub filter: Callback<(String, String), bool>,
}

impl ComboboxContext {
    pub fn set_open(&mut self, open: bool) {
        if open {
            self.selectable.focus_state.set_focus(None);
        }
        self.selectable.set_open(open);
    }

    pub fn is_visible(&self, tab_index: usize) -> bool {
        let query = self.query.cloned();
        self.selectable
            .options
            .read()
            .iter()
            .find(|option| option.tab_index == tab_index)
            .is_some_and(|option| self.filter.call((query.clone(), option.text_value.clone())))
    }

    pub fn has_visible_options(&self) -> bool {
        let query = self.query.cloned();
        self.selectable
            .options
            .read()
            .iter()
            .any(|option| self.filter.call((query.clone(), option.text_value.clone())))
    }

    pub fn focused_visible_option_id(&self) -> Option<String> {
        self.selectable
            .focused_option_id_where(|option| self.is_visible(option.tab_index))
    }

    pub fn focus_next_visible(&mut self) {
        let query = self.query.cloned();
        let filter = self.filter;
        self.selectable
            .focus_next_where(|option| filter.call((query.clone(), option.text_value.clone())));
    }

    pub fn focus_prev_visible(&mut self) {
        let query = self.query.cloned();
        let filter = self.filter;
        self.selectable
            .focus_prev_where(|option| filter.call((query.clone(), option.text_value.clone())));
    }

    pub fn focus_first_visible(&mut self) {
        let query = self.query.cloned();
        let filter = self.filter;
        self.selectable
            .focus_first_where(|option| filter.call((query.clone(), option.text_value.clone())));
    }

    pub fn focus_last_visible(&mut self) {
        let query = self.query.cloned();
        let filter = self.filter;
        self.selectable
            .focus_last_where(|option| filter.call((query.clone(), option.text_value.clone())));
    }

    pub fn select_focused(&mut self) {
        let query = self.query.cloned();
        let filter = self.filter;
        self.selectable
            .select_focused_where(|option| filter.call((query.clone(), option.text_value.clone())));
    }
}
