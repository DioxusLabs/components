//! Context types and implementations for the select component.

use crate::focus::FocusState;
use dioxus::prelude::*;
use dioxus_core::Task;
use dioxus_time::sleep;

use std::{any::Any, rc::Rc, time::Duration};

use super::text_search::AdaptiveKeyboard;

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
    /// A list of options with their states
    pub options: Signal<Vec<OptionState>>,
    /// Adaptive keyboard system for multi-language support
    pub adaptive_keyboard: Signal<AdaptiveKeyboard>,
    /// The ID of the list for ARIA attributes
    pub list_id: Signal<Option<String>>,
    /// The focus state for the select
    pub focus_state: FocusState,
    /// Whether the select is disabled
    pub disabled: ReadOnlySignal<bool>,
    /// The placeholder text
    pub placeholder: ReadOnlySignal<String>,
    /// Task handle for clearing typeahead buffer
    pub typeahead_clear_task: Signal<Option<Task>>,
    /// Timeout before clearing typeahead buffer
    pub typeahead_timeout: ReadOnlySignal<Duration>,
}

impl SelectContext {
    /// Select the currently focused item
    pub fn select_current_item(&mut self) {
        // If the select is open, select the focused item
        if self.open.cloned() {
            if let Some(focused_index) = self.focus_state.current_focus() {
                let options = self.options.read();
                if let Some(option) = options.iter().find(|opt| opt.tab_index == focused_index) {
                    self.set_value.call(Some(option.value.clone()));
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
            super::text_search::best_match(&keyboard, &typeahead, &options)
        {
            self.focus_state.set_focus(Some(best_match_index));
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
}

/// Context for select option components to know if they're selected
#[derive(Clone, Copy)]
pub(super) struct SelectOptionContext {
    /// Whether this option is currently selected
    pub selected: ReadOnlySignal<bool>,
}

/// Context for children of select list components to know if they should render
#[derive(Clone, Copy)]
pub(super) struct SelectListContext {
    /// Whether to render in the dom (or just run logic)
    pub render: ReadOnlySignal<bool>,
}

/// Context for select group components
#[derive(Clone, Copy)]
pub(super) struct SelectGroupContext {
    /// ID of the element that labels this group
    pub labeled_by: Signal<Option<String>>,
}
