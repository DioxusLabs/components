//! Context types and implementations for the select component.

use dioxus::prelude::*;
use dioxus_core::Task;
use dioxus_sdk_time::sleep;

use super::text_search::AdaptiveKeyboard;
use std::collections::BTreeMap;
use std::{any::Any, rc::Rc, time::Duration};

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

/// Main context for the select component containing all shared state
#[derive(Clone, Copy)]
pub(super) struct SelectContext {
    /// The typeahead buffer for searching options
    pub typeahead_buffer: Signal<String>,
    /// If the select is open
    pub open: Signal<bool>,
    /// Current value
    pub value: Memo<Option<RcPartialEqValue>>,
    /// Set the value callback
    pub set_value: Callback<Option<RcPartialEqValue>>,
    /// Adaptive keyboard system for multi-language support
    pub adaptive_keyboard: Signal<AdaptiveKeyboard>,
    /// The ID of the list for ARIA attributes
    pub list_id: Signal<Option<String>>,
    /// Whether the select is disabled
    pub disabled: ReadSignal<bool>,
    /// The placeholder text
    pub placeholder: ReadSignal<String>,
    /// Task handle for clearing typeahead buffer
    pub typeahead_clear_task: Signal<Option<Task>>,
    /// Timeout before clearing typeahead buffer
    pub typeahead_timeout: ReadSignal<Duration>,
    /// A list of options with their states
    pub(crate) options: Signal<BTreeMap<usize, OptionState>>,
    /// If focus should loop around
    pub roving_loop: ReadSignal<bool>,
    /// The currently selected option tab_index
    pub(crate) current_focus: Signal<Option<usize>>,
    /// The initial element to focus once the list is rendered<br>
    /// true: last element<br>
    /// false: first element
    pub initial_focus_last: Signal<Option<bool>>,
}

impl SelectContext {
    /// custom implementation for `FocusState::is_selected`
    pub(crate) fn is_focused(&self, id: usize) -> bool {
        (self.current_focus)() == Some(id)
    }

    pub(crate) fn any_focused(&self) -> bool {
        self.current_focus.read().is_some()
    }

    pub(crate) fn current_focus(&self) -> Option<usize> {
        (self.current_focus)()
    }

    pub(crate) fn current_focus_id(&self) -> Option<String> {
        let focus = (self.current_focus)()?;
        self.options.read().get(&focus).map(|s| s.id.clone())
    }

    pub(crate) fn blur(&mut self) {
        self.current_focus.write().take();
    }

    /// custom implementation for `FocusState::focus_next`
    pub(crate) fn focus_next(&mut self) {
        // select first if current is none
        let current_focus = match self.current_focus() {
            Some(k) => k,
            None => return self.focus_first(),
        };

        let options = self.options.read();

        // iterate until the end of the map
        for (index, state) in options.range((current_focus + 1)..) {
            // focus if not disabled
            if !state.disabled {
                self.current_focus.set(Some(*index));
                return;
            }
        }

        // stop if we dont allow rollover
        if !(self.roving_loop)() {
            return;
        }

        // iterate over the rest of the map starting from the beginning
        for (index, state) in options.range(..=current_focus) {
            // stop if we reached the current element
            if *index == current_focus {
                break;
            }

            // focus if not disabled
            if !state.disabled {
                self.current_focus.set(Some(*index));
                return;
            }
        }
    }

    /// custom implementation for `FocusState::focus_prev`
    pub(crate) fn focus_prev(&mut self) {
        // focus last if current is none
        let current_focus = match self.current_focus() {
            Some(k) => k,
            None => return self.focus_last(),
        };

        let options = self.options.read();

        // iterate until the start of the map (reversed)
        for (index, state) in options.range(..current_focus).rev() {
            // focus if not disabled
            if !state.disabled {
                self.current_focus.set(Some(*index));
                return;
            }
        }

        // stop if we dont allow rollover
        if !(self.roving_loop)() {
            return;
        }

        // iterate over the rest of the map starting from the end (reversed)
        for (index, state) in options.range(current_focus..).rev() {
            // stop if we reached the current element
            if *index == current_focus {
                break;
            }

            // focus if not disabled
            if !state.disabled {
                self.current_focus.set(Some(*index));
                return;
            }
        }
    }

    /// custom implementation for `FocusState::focus_first`
    pub(crate) fn focus_first(&mut self) {
        if let Some((index, _)) = self
            .options
            .read()
            .iter()
            .find(|(_, state)| !state.disabled)
        {
            self.current_focus.set(Some(*index));
        }
    }

    /// custom implementation for `FocusState::focus_last`
    pub(crate) fn focus_last(&mut self) {
        if let Some((index, _)) = self
            .options
            .read()
            .iter()
            .rev()
            .find(|(_, state)| !state.disabled)
        {
            self.current_focus.set(Some(*index));
        }
    }

    /// Select the currently focused item
    pub fn select_current_item(&mut self) {
        // If the select is open, select the focused item
        if self.open.cloned() {
            if let Some(focused_index) = self.current_focus() {
                let options = self.options.read();
                if let Some(state) = options.get(&focused_index) {
                    self.set_value.call(Some(state.value.clone()));
                    self.open.set(false);
                }
            }
        }
    }

    /// Learn from a keyboard event mapping physical key to logical character
    pub fn learn_from_keyboard_event(&mut self, physical_code: &str, logical_char: char) {
        let mut adaptive = self.adaptive_keyboard.write();
        let logical_char = logical_char.to_lowercase().next().unwrap_or(logical_char);
        adaptive.learn_from_event(physical_code, logical_char);
    }

    /// Add text to the typeahead buffer for searching
    pub fn add_to_typeahead_buffer(&mut self, text: &str) {
        // Cancel any existing clear task to prevent race conditions
        if let Some(existing_task) = self.typeahead_clear_task.write().take() {
            existing_task.cancel();
        }

        // Update the buffer and get the current content
        let typeahead = {
            let mut typeahead_buffer = self.typeahead_buffer.write();
            typeahead_buffer.push_str(text);
            typeahead_buffer.clone()
        };

        // Create references for the async closure
        let mut typeahead_buffer_signal = self.typeahead_buffer;
        let mut typeahead_clear_task_signal = self.typeahead_clear_task;

        // Spawn a new task to clear the buffer after the configured timeout
        let timeout = self.typeahead_timeout.cloned();
        let new_task = spawn(async move {
            sleep(timeout).await;

            // Clear the buffer
            typeahead_buffer_signal.write().clear();

            // Remove our own task handle to indicate no task is active
            typeahead_clear_task_signal.write().take();
        });

        // Store the new task handle
        self.typeahead_clear_task.write().replace(new_task);

        // Focus the best match using adaptive keyboard
        let options = self.options.read();
        let keyboard = self.adaptive_keyboard.read();

        if let Some(best_match_index) =
            super::text_search::best_match(&keyboard, &typeahead, options.values())
        {
            self.current_focus.set(Some(best_match_index));
        }
    }
}

/// State for individual select options
pub(super) struct OptionState {
    /// Tab index for focus management
    pub tab_index: usize,
    /// The value of the option
    pub value: RcPartialEqValue,
    /// Display text for the option
    pub text_value: String,
    /// Unique ID for the option
    pub id: String,
    /// Whether the option is disabled
    pub disabled: bool,
}

/// Context for select option components to know if they're selected
#[derive(Clone, Copy)]
pub(super) struct SelectOptionContext {
    /// Whether this option is currently selected
    pub selected: ReadSignal<bool>,
}

/// Context for children of select list components to know if they should render
#[derive(Clone, Copy)]
pub(super) struct SelectListContext {
    /// Whether to render in the dom (or just run logic)
    pub render: ReadSignal<bool>,
}

/// Context for select group components
#[derive(Clone, Copy)]
pub(super) struct SelectGroupContext {
    /// ID of the element that labels this group
    pub labeled_by: Signal<Option<String>>,
}
