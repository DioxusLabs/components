//! Shared state for the combobox component.

use crate::selectable::{OptionState, SelectableContext};
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

    fn predicate_for(&self, query: String) -> impl Fn(&OptionState) -> bool {
        let filter = self.filter;
        move |option| filter.call((query.clone(), option.text_value.clone()))
    }

    fn predicate(&self) -> impl Fn(&OptionState) -> bool {
        self.predicate_for(self.query.cloned())
    }

    pub fn is_visible(&self, tab_index: usize) -> bool {
        let predicate = self.predicate();
        self.selectable
            .options
            .read()
            .iter()
            .find(|option| option.tab_index == tab_index)
            .is_some_and(predicate)
    }

    pub fn has_visible_options(&self) -> bool {
        self.selectable.options.read().iter().any(self.predicate())
    }

    pub fn first_visible_enabled_index_for_query(&self, query: String) -> Option<usize> {
        self.selectable
            .first_matching_enabled_index(self.predicate_for(query))
    }

    pub fn last_visible_enabled_index_for_query(&self, query: String) -> Option<usize> {
        self.selectable
            .last_matching_enabled_index(self.predicate_for(query))
    }

    pub fn focused_visible_option_id(&self) -> Option<String> {
        self.selectable.focused_option_id_where(self.predicate())
    }

    pub fn focus_next_visible(&mut self) {
        self.selectable.focus_next_where(self.predicate());
    }

    pub fn focus_prev_visible(&mut self) {
        self.selectable.focus_prev_where(self.predicate());
    }

    pub fn focus_first_visible(&mut self) {
        self.selectable.focus_first_where(self.predicate());
    }

    pub fn focus_last_visible(&mut self) {
        self.selectable.focus_last_where(self.predicate());
    }

    pub fn select_focused(&mut self) {
        self.selectable.select_focused_where(self.predicate());
    }
}
