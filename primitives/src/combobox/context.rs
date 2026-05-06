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
        let mut indices: Vec<_> = self
            .selectable
            .options
            .read()
            .iter()
            .filter(|option| {
                !option.disabled && self.filter.call((query.clone(), option.text_value.clone()))
            })
            .map(|option| option.tab_index)
            .collect();
        indices.sort_unstable();
        indices
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
            let Some(current_position) = indices.iter().position(|&index| index == current) else {
                let next_position = indices.partition_point(|&index| index <= current);
                return indices
                    .get(next_position)
                    .copied()
                    .or_else(|| roving_loop.then(|| indices.first().copied()).flatten());
            };
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
            let Some(current_position) = indices.iter().position(|&index| index == current) else {
                let prev_position = indices.partition_point(|&index| index < current);
                return prev_position
                    .checked_sub(1)
                    .and_then(|position| indices.get(position).copied())
                    .or_else(|| roving_loop.then(|| indices.last().copied()).flatten());
            };
            current_position
                .checked_sub(1)
                .and_then(|position| indices.get(position).copied())
                .or_else(|| roving_loop.then(|| indices.last().copied()).flatten())
        }
        None if roving_loop => indices.last().copied(),
        None => indices.first().copied(),
    }
}

#[cfg(test)]
mod tests {
    use super::{next_index, prev_index};

    #[test]
    fn next_index_recovers_when_current_is_not_visible() {
        assert_eq!(next_index(&[2, 4, 6], Some(1), true), Some(2));
        assert_eq!(next_index(&[2, 4, 6], Some(1), false), Some(2));
        assert_eq!(next_index(&[1, 3, 6], Some(4), true), Some(6));
        assert_eq!(next_index(&[1, 3, 6], Some(4), false), Some(6));
        assert_eq!(next_index(&[1, 3, 6], Some(7), true), Some(1));
        assert_eq!(next_index(&[1, 3, 6], Some(7), false), None);
    }

    #[test]
    fn prev_index_recovers_when_current_is_not_visible() {
        assert_eq!(prev_index(&[2, 4, 6], Some(1), true), Some(6));
        assert_eq!(prev_index(&[2, 4, 6], Some(1), false), None);
        assert_eq!(prev_index(&[1, 3, 6], Some(4), true), Some(3));
        assert_eq!(prev_index(&[1, 3, 6], Some(4), false), Some(3));
        assert_eq!(prev_index(&[1, 3, 6], Some(0), true), Some(6));
        assert_eq!(prev_index(&[1, 3, 6], Some(0), false), None);
    }

    #[test]
    fn visible_index_navigation_handles_empty_lists() {
        assert_eq!(next_index(&[], Some(1), true), None);
        assert_eq!(prev_index(&[], Some(1), true), None);
    }
}
