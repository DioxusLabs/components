//! Shared state for the combobox component.

pub(crate) use crate::selectable::RcPartialEqValue;
use crate::selectable::SelectableContext;
use dioxus::prelude::*;

/// The default case-insensitive substring filter.
pub fn default_combobox_filter(query: &str, text: &str) -> bool {
    let query = query.trim().to_lowercase();
    query.is_empty() || text.to_lowercase().contains(&query)
}

fn combobox_match_rank(query: &str, text: &str) -> Option<(usize, usize, usize)> {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return Some((0, 0, 0));
    }

    let text = text.to_lowercase();
    if text == query {
        return Some((0, 0, text.len()));
    }

    if text.starts_with(&query) {
        return Some((1, 0, text.len()));
    }

    let word_prefix = text.match_indices(&query).find_map(|(index, _)| {
        text[..index]
            .chars()
            .last()
            .is_some_and(|c| !c.is_alphanumeric())
            .then_some(index)
    });
    if let Some(index) = word_prefix {
        return Some((2, index, text.len()));
    }

    text.find(&query).map(|index| (3, index, text.len()))
}

#[derive(Clone, Copy)]
pub(super) struct ComboboxContext {
    pub selectable: SelectableContext,
    pub query: Signal<String>,
    pub filter: Callback<(String, String), bool>,
}

impl ComboboxContext {
    pub fn set_open(&mut self, open: bool) {
        self.selectable.set_open(open);
    }

    pub fn selected_text(&self) -> Option<String> {
        self.selectable.selected_text()
    }

    pub fn is_selected(&self, value: &RcPartialEqValue) -> bool {
        self.selectable.is_selected(value)
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

    fn visible_enabled_indices(&self) -> Vec<usize> {
        let query = self.query.cloned();
        let mut matches: Vec<_> = self
            .selectable
            .options
            .read()
            .iter()
            .filter(|option| {
                !option.disabled && self.filter.call((query.clone(), option.text_value.clone()))
            })
            .map(|option| {
                (
                    combobox_match_rank(&query, &option.text_value).unwrap_or((usize::MAX, 0, 0)),
                    option.tab_index,
                )
            })
            .collect();
        matches.sort_by_key(|(rank, tab_index)| (*rank, *tab_index));
        matches
            .into_iter()
            .map(|(_, tab_index)| tab_index)
            .collect()
    }

    pub fn visible_option_order(&self, tab_index: usize) -> Option<usize> {
        self.visible_enabled_indices()
            .into_iter()
            .position(|index| index == tab_index)
    }

    pub fn focus_next_visible(&mut self) {
        let indices = self.visible_enabled_indices();
        let Some(next) = next_index(
            &indices,
            self.selectable.focus_state.current_focus(),
            (self.selectable.focus_state.roving_loop)(),
        ) else {
            self.selectable.focus_state.set_focus(None);
            return;
        };
        self.selectable.focus_state.set_focus(Some(next));
    }

    pub fn focus_prev_visible(&mut self) {
        let indices = self.visible_enabled_indices();
        let Some(next) = prev_index(
            &indices,
            self.selectable.focus_state.current_focus(),
            (self.selectable.focus_state.roving_loop)(),
        ) else {
            self.selectable.focus_state.set_focus(None);
            return;
        };
        self.selectable.focus_state.set_focus(Some(next));
    }

    pub fn focus_first_visible(&mut self) {
        let first = self.visible_enabled_indices().into_iter().next();
        self.selectable.focus_state.set_focus(first);
    }

    pub fn focus_last_visible(&mut self) {
        let last = self.visible_enabled_indices().into_iter().next_back();
        self.selectable.focus_state.set_focus(last);
    }

    pub fn select_focused(&mut self) {
        let query = self.query.cloned();
        let filter = self.filter;
        self.selectable
            .select_focused_where(|option| filter.call((query.clone(), option.text_value.clone())));
    }

    pub fn select_value(&mut self, value: RcPartialEqValue) {
        self.selectable.select_value(value);
    }
}

fn next_index(indices: &[usize], current: Option<usize>, roving_loop: bool) -> Option<usize> {
    match current {
        Some(current) => {
            let current_position = indices.iter().position(|&index| index == current)?;
            indices
                .get(current_position + 1)
                .copied()
                .or_else(|| roving_loop.then(|| indices.first().copied()).flatten())
        }
        None => indices.first().copied(),
    }
}

fn prev_index(indices: &[usize], current: Option<usize>, roving_loop: bool) -> Option<usize> {
    match current {
        Some(current) => {
            let current_position = indices.iter().position(|&index| index == current)?;
            current_position
                .checked_sub(1)
                .and_then(|position| indices.get(position).copied())
                .or_else(|| roving_loop.then(|| indices.last().copied()).flatten())
        }
        None if roving_loop => indices.last().copied(),
        None => indices.first().copied(),
    }
}
