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
    /// Stable callback that returns the option's rendered `<div role="option">`.
    /// `ComboboxList` calls this in relevance-ranked order so that DOM order
    /// matches visual order — keeping screen-reader exploration consistent
    /// with what sighted users see.
    pub render: Callback<(), Element>,
}

/// Default fuzzy filter: empty query matches everything; otherwise an option
/// is visible if its `text_value` contains the query as a case-insensitive
/// substring **or** as an in-order subsequence of characters.
///
/// The subsequence pass is what makes "svk" match "SvelteKit" and "nxt"
/// match "Next.js" — close to cmdk's default scoring behavior.
pub fn default_combobox_filter(query: &str, text: &str) -> bool {
    match_score(query, text).is_some()
}

/// Score how well `text` matches `query`. Lower is better; `None` means no
/// match. Matches are tiered: prefix (best) < substring < subsequence (worst).
/// Within a tier, shorter text and earlier match position are preferred.
pub(super) fn match_score(query: &str, text: &str) -> Option<u32> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return Some(0);
    }
    let t = text.to_lowercase();

    // Tier 1: prefix match. Score by text length so shorter prefixes win.
    if t.starts_with(&q) {
        return Some(t.chars().count() as u32);
    }
    // Tier 2: contiguous substring. Earlier match position is better, then
    // shorter text. Score is offset above tier 1.
    if let Some(byte_pos) = t.find(&q) {
        let char_pos = t[..byte_pos].chars().count() as u32;
        return Some(1_000 + char_pos * 10 + t.chars().count() as u32);
    }
    // Tier 3: subsequence match. Score by total characters skipped between
    // matched positions.
    let mut t_chars = t.chars();
    let mut skipped: u32 = 0;
    'q: for c in q.chars() {
        for tc in t_chars.by_ref() {
            if tc == c {
                continue 'q;
            }
            skipped += 1;
        }
        return None;
    }
    Some(10_000 + skipped + t.chars().count() as u32)
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

    /// Tab indices of options that pass the filter and aren't disabled, ordered
    /// by relevance: best match first when there's a query, otherwise declared
    /// order. Ties fall back to declared order so the result is stable.
    pub fn visible_indices(&self) -> Vec<usize> {
        let options = self.options.read();
        let query = self.query.read().clone();
        let q_trim = query.trim().to_string();

        let mut visible: Vec<(Option<u32>, usize)> = options
            .iter()
            .filter(|o| !o.disabled && self.option_matches(o))
            .map(|o| {
                let score = if q_trim.is_empty() {
                    None
                } else {
                    match_score(&q_trim, &o.text_value)
                };
                (score, o.tab_index)
            })
            .collect();

        visible.sort_by(|(s1, t1), (s2, t2)| match (s1, s2) {
            (Some(a), Some(b)) => a.cmp(b).then_with(|| t1.cmp(t2)),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => t1.cmp(t2),
        });

        visible.into_iter().map(|(_, ti)| ti).collect()
    }

    /// Whether at least one option is visible.
    pub fn has_visible_options(&self) -> bool {
        let options = self.options.read();
        options
            .iter()
            .any(|o| !o.disabled && self.option_matches(o))
    }

    /// Move focus to the next visible option (in ranked order), wrapping.
    pub fn focus_next_visible(&mut self) {
        let visible = self.visible_indices();
        if visible.is_empty() {
            self.focus_state.set_focus(None);
            return;
        }
        let next = match self.focus_state.recent_focus() {
            Some(curr) => match visible.iter().position(|&i| i == curr) {
                Some(pos) => visible.get(pos + 1).copied().unwrap_or(visible[0]),
                None => visible[0],
            },
            None => visible[0],
        };
        self.focus_state.set_focus(Some(next));
    }

    /// Move focus to the previous visible option (in ranked order), wrapping.
    pub fn focus_prev_visible(&mut self) {
        let visible = self.visible_indices();
        if visible.is_empty() {
            self.focus_state.set_focus(None);
            return;
        }
        let last = *visible.last().unwrap();
        let prev = match self.focus_state.recent_focus() {
            Some(curr) => match visible.iter().position(|&i| i == curr) {
                Some(0) => last,
                Some(pos) => visible[pos - 1],
                None => last,
            },
            None => last,
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
