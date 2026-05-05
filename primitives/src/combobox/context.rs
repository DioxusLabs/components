//! Context types for the combobox component.

use crate::focus::FocusState;
use dioxus::prelude::*;

use std::{any::Any, rc::Rc};

trait DynPartialEq: Any {
    fn eq(&self, other: &dyn Any) -> bool;
}

impl<T: PartialEq + 'static> DynPartialEq for T {
    fn eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<T>() == Some(self)
    }
}

#[derive(Clone)]
pub(crate) struct RcPartialEqValue {
    value: Rc<dyn DynPartialEq>,
}

impl RcPartialEqValue {
    pub fn new<T: PartialEq + 'static>(value: T) -> Self {
        Self {
            value: Rc::new(value),
        }
    }

    pub fn as_any(&self) -> &dyn Any {
        (&*self.value) as &dyn Any
    }

    pub fn as_ref<T: PartialEq + 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

impl PartialEq for RcPartialEqValue {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&*other.value)
    }
}

/// State for individual combobox options.
pub(super) struct OptionState {
    /// Order of the option as it was registered.
    pub tab_index: usize,
    /// The value of the option.
    pub value: RcPartialEqValue,
    /// Display text used for filtering and trigger value.
    pub text_value: String,
    /// Unique ID for the option.
    pub id: String,
    /// Whether the option is disabled.
    pub disabled: bool,
}

/// Default case-insensitive substring filter.
pub fn default_combobox_filter(query: &str, text: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return true;
    }
    text.to_lowercase().contains(&query)
}

/// Main context for the combobox component.
#[derive(Clone, Copy)]
pub(super) struct ComboboxContext {
    /// Whether the popup is open.
    pub open: Signal<bool>,
    /// Current search/filter query.
    pub query: Signal<String>,
    /// Current value.
    pub value: Memo<Option<RcPartialEqValue>>,
    /// Set the value callback.
    pub set_value: Callback<Option<RcPartialEqValue>>,
    /// All registered options.
    pub options: Signal<Vec<OptionState>>,
    /// Filter callback used to decide which options match the query.
    pub filter: Callback<(String, String), bool>,
    /// The id of the listbox for ARIA wiring.
    pub list_id: Signal<Option<String>>,
    /// The id of the search input for ARIA wiring.
    pub input_id: Signal<Option<String>>,
    /// Roving focus state for option keyboard navigation.
    pub focus_state: FocusState,
    /// Whether the combobox is disabled.
    pub disabled: ReadSignal<bool>,
    /// Placeholder text for an empty value.
    pub placeholder: ReadSignal<String>,
}

impl ComboboxContext {
    /// Returns whether the option matches the current query.
    pub fn option_matches(&self, opt: &OptionState) -> bool {
        let query = self.query.read().clone();
        self.filter.call((query, opt.text_value.clone()))
    }

    /// Sorted tab indices of options that pass the filter and aren't disabled.
    pub fn visible_indices(&self) -> Vec<usize> {
        let options = self.options.read();
        let mut visible: Vec<(usize, usize)> = options
            .iter()
            .filter(|o| !o.disabled && self.option_matches(o))
            .map(|o| (o.tab_index, o.tab_index))
            .collect();
        visible.sort_by_key(|(_, ti)| *ti);
        visible.into_iter().map(|(ti, _)| ti).collect()
    }

    /// Whether at least one option is visible.
    pub fn has_visible_options(&self) -> bool {
        let options = self.options.read();
        options
            .iter()
            .any(|o| !o.disabled && self.option_matches(o))
    }

    /// Move focus to the next visible option, wrapping if needed.
    pub fn focus_next_visible(&mut self) {
        let visible = self.visible_indices();
        if visible.is_empty() {
            self.focus_state.set_focus(None);
            return;
        }
        let current = self.focus_state.recent_focus();
        let next = match current {
            Some(curr) => visible
                .iter()
                .copied()
                .find(|&i| i > curr)
                .unwrap_or_else(|| visible[0]),
            None => visible[0],
        };
        self.focus_state.set_focus(Some(next));
    }

    /// Move focus to the previous visible option, wrapping if needed.
    pub fn focus_prev_visible(&mut self) {
        let visible = self.visible_indices();
        if visible.is_empty() {
            self.focus_state.set_focus(None);
            return;
        }
        let current = self.focus_state.recent_focus();
        let prev = match current {
            Some(curr) => visible
                .iter()
                .copied()
                .rev()
                .find(|&i| i < curr)
                .unwrap_or_else(|| *visible.last().unwrap()),
            None => *visible.last().unwrap(),
        };
        self.focus_state.set_focus(Some(prev));
    }

    /// Move focus to the first visible option.
    pub fn focus_first_visible(&mut self) {
        let visible = self.visible_indices();
        if let Some(first) = visible.first() {
            self.focus_state.set_focus(Some(*first));
        }
    }

    /// Move focus to the last visible option.
    pub fn focus_last_visible(&mut self) {
        let visible = self.visible_indices();
        if let Some(last) = visible.last() {
            self.focus_state.set_focus(Some(*last));
        }
    }

    /// Select the currently focused (visible) option, if any.
    pub fn select_focused(&mut self) {
        if !self.open.cloned() {
            return;
        }
        let Some(idx) = self.focus_state.current_focus() else {
            return;
        };
        let options = self.options.read();
        if let Some(opt) = options.iter().find(|o| o.tab_index == idx) {
            if !opt.disabled {
                self.set_value.call(Some(opt.value.clone()));
                drop(options);
                self.open.set(false);
                self.query.set(String::new());
            }
        }
    }
}

/// Context for individual options to know if they're selected.
#[derive(Clone, Copy)]
pub(super) struct ComboboxOptionContext {
    pub selected: ReadSignal<bool>,
}

/// Context for child components to know if they should render.
#[derive(Clone, Copy)]
pub(super) struct ComboboxContentContext {
    pub render: ReadSignal<bool>,
}

/// Context for combobox group components.
#[derive(Clone, Copy)]
pub(super) struct ComboboxGroupContext {
    pub labeled_by: Signal<Option<String>>,
}
