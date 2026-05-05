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
    /// ID of the group that owns this option, if any.
    pub group_id: Option<String>,
    /// Stable callback that returns the option's rendered `<div role="option">`.
    /// `ComboboxList` calls this in relevance-ranked order so that DOM order
    /// matches visual order — keeping screen-reader exploration consistent
    /// with what sighted users see.
    pub render: Callback<(), Element>,
}

impl PartialEq for OptionState {
    // `render` is a stable Callback handle whose inner closure swaps every
    // render — comparing it would never report equality. The other fields
    // fully describe registration identity.
    fn eq(&self, other: &Self) -> bool {
        self.tab_index == other.tab_index
            && self.value == other.value
            && self.text_value == other.text_value
            && self.id == other.id
            && self.disabled == other.disabled
            && self.group_id == other.group_id
    }
}

/// A filtered option ready to render.
#[derive(Clone)]
pub(super) struct VisibleOptionState {
    /// Order of the option as it was registered.
    pub tab_index: usize,
    /// Whether the option is disabled.
    pub disabled: bool,
    /// ID of the group that owns this option, if any.
    pub group_id: Option<String>,
    /// Stable callback that returns the option's rendered `<div role="option">`.
    pub render: Callback<(), Element>,
}

impl PartialEq for VisibleOptionState {
    fn eq(&self, other: &Self) -> bool {
        self.tab_index == other.tab_index
            && self.disabled == other.disabled
            && self.group_id == other.group_id
    }
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
    /// Currently selected values. Single-select stores 0 or 1; multi-select
    /// stores any number, in user-toggle order.
    pub values: Memo<Vec<RcPartialEqValue>>,
    /// Toggle/replace a value. In single mode replaces the selection (or
    /// clears it when called with `None`); in multi mode toggles a value's
    /// membership in `values` (`None` is a no-op).
    pub set_value: Callback<Option<RcPartialEqValue>>,
    /// Whether this is a multi-select combobox.
    pub multi: bool,
    /// All registered options.
    pub options: Signal<Vec<OptionState>>,
    /// The id of the listbox for ARIA wiring.
    pub list_id: Signal<Option<String>>,
    /// Roving focus state for option keyboard navigation.
    pub focus_state: FocusState,
    /// Whether the combobox is disabled.
    pub disabled: ReadSignal<bool>,
    /// Visible options in relevance-ranked order. The root `Combobox`
    /// component computes this once per `(options, query)` change so the
    /// list, empty placeholder, and keyboard navigation share a single scan.
    pub visible: Memo<Vec<VisibleOptionState>>,
}

impl ComboboxContext {
    /// `text_value`(s) for the currently selected option(s). Single mode
    /// returns the lone selection's text; multi mode returns the joined
    /// `text_value`s in selection order. `None` when nothing is selected.
    pub fn selected_text(&self) -> Option<String> {
        let values = self.values.read();
        if values.is_empty() {
            return None;
        }
        let options = self.options.read();
        let parts: Vec<String> = values
            .iter()
            .filter_map(|v| {
                options
                    .iter()
                    .find(|opt| &opt.value == v)
                    .map(|opt| opt.text_value.clone())
            })
            .collect();
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(", "))
        }
    }

    /// Whether the given value is currently selected.
    pub fn is_selected(&self, value: &RcPartialEqValue) -> bool {
        self.values.read().iter().any(|v| v == value)
    }

    /// Tab indices of visible options in ranked order.
    pub fn visible_indices(&self) -> Vec<usize> {
        self.visible
            .read()
            .iter()
            .filter_map(|option| (!option.disabled).then_some(option.tab_index))
            .collect()
    }

    /// Render callbacks of visible root options in ranked order.
    pub fn root_visible_renders(&self) -> Vec<Callback<(), Element>> {
        self.visible
            .read()
            .iter()
            .filter(|option| option.group_id.is_none())
            .map(|option| option.render)
            .collect()
    }

    /// Render callbacks of visible options in a group in ranked order.
    pub fn group_visible_renders(&self, group_id: &str) -> Vec<Callback<(), Element>> {
        self.visible
            .read()
            .iter()
            .filter(|option| option.group_id.as_deref() == Some(group_id))
            .map(|option| option.render)
            .collect()
    }

    /// Whether a group has any visible options.
    pub fn group_has_visible_options(&self, group_id: &str) -> bool {
        self.visible
            .read()
            .iter()
            .any(|option| option.group_id.as_deref() == Some(group_id))
    }

    /// Whether at least one option is visible.
    pub fn has_visible_options(&self) -> bool {
        !self.visible.read().is_empty()
    }

    /// Whether an enabled option is still in the visible set.
    pub fn is_visible_focusable(&self, tab_index: usize) -> bool {
        self.visible
            .read()
            .iter()
            .any(|option| option.tab_index == tab_index && !option.disabled)
    }

    /// ID of the focused option, only when it is currently visible and enabled.
    pub fn focused_visible_option_id(&self) -> Option<String> {
        let idx = self.focus_state.current_focus()?;
        if !self.is_visible_focusable(idx) {
            return None;
        }
        self.options
            .read()
            .iter()
            .find(|opt| opt.tab_index == idx)
            .map(|opt| opt.id.clone())
    }

    /// Move focus to the next visible enabled option in ranked order.
    pub fn focus_next_visible(&mut self) {
        let indices = self.visible_indices();
        if indices.is_empty() {
            self.focus_state.set_focus(None);
            return;
        }
        let roving_loop = self.focus_state.roving_loop.cloned();
        let next = match self.focus_state.recent_focus() {
            Some(curr) => match indices.iter().position(|&i| i == curr) {
                Some(pos) => indices
                    .get(pos + 1)
                    .copied()
                    .or_else(|| roving_loop.then_some(indices[0]))
                    .unwrap_or(curr),
                None => indices[0],
            },
            None => indices[0],
        };
        self.focus_state.set_focus(Some(next));
    }

    /// Move focus to the previous visible enabled option in ranked order.
    pub fn focus_prev_visible(&mut self) {
        let indices = self.visible_indices();
        if indices.is_empty() {
            self.focus_state.set_focus(None);
            return;
        }
        let last = *indices.last().unwrap();
        let roving_loop = self.focus_state.roving_loop.cloned();
        let prev = match self.focus_state.recent_focus() {
            Some(curr) => match indices.iter().position(|&i| i == curr) {
                Some(0) => {
                    if roving_loop {
                        last
                    } else {
                        curr
                    }
                }
                Some(pos) => indices[pos - 1],
                None => {
                    if roving_loop {
                        last
                    } else {
                        indices[0]
                    }
                }
            },
            None => {
                if roving_loop {
                    last
                } else {
                    indices[0]
                }
            }
        };
        self.focus_state.set_focus(Some(prev));
    }

    /// Move focus to the first visible enabled option.
    pub fn focus_first_visible(&mut self) {
        self.focus_state
            .set_focus(self.visible_indices().first().copied());
    }

    /// Move focus to the last visible enabled option.
    pub fn focus_last_visible(&mut self) {
        self.focus_state
            .set_focus(self.visible_indices().last().copied());
    }

    /// Select the currently focused (visible) option, if any.
    pub fn select_focused(&mut self) {
        if !self.open.cloned() {
            return;
        }
        let Some(idx) = self.focus_state.current_focus() else {
            return;
        };
        if !self.is_visible_focusable(idx) {
            return;
        }
        let value = {
            let options = self.options.read();
            options
                .iter()
                .find(|o| o.tab_index == idx && !o.disabled)
                .map(|o| o.value.clone())
        };
        if let Some(value) = value {
            self.select_value(value);
        }
    }

    /// Select or toggle a value. In single mode this commits the value, closes
    /// the popup, and clears the query. In multi mode it toggles the value's
    /// membership in the selection while leaving the popup open and the query
    /// intact, so the user can keep selecting from the filtered list.
    pub fn select_value(&mut self, value: RcPartialEqValue) {
        self.set_value.call(Some(value));
        if !self.multi {
            self.open.set(false);
            self.query.set(String::new());
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
    pub id: Memo<String>,
    pub labeled_by: Signal<Option<String>>,
    pub visible: Memo<bool>,
}
