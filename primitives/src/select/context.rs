//! Context types and implementations for the select component.

use crate::focus::FocusState;
use dioxus::prelude::*;
use dioxus_core::Task;
use dioxus_time::sleep;

use std::time::Duration;

use super::text_search::{AdaptiveKeyboard, KeyboardLayout};

/// Represents the current cursor state in the select component
#[derive(Clone, PartialEq)]
pub(super) struct SelectCursor<V> {
    /// Typed value of the select
    pub value: V,
    /// Human-readable display of the select value
    pub text_value: String,
}

/// Main context for the select component containing all shared state
#[derive(Clone, Copy)]
pub(super) struct SelectContext<T: Clone + PartialEq + 'static> {
    /// The typeahead buffer for searching options
    pub typeahead_buffer: Signal<String>,
    /// If the select is open
    pub open: Signal<bool>,
    /// Current cursor position and value
    pub cursor: Memo<SelectCursor<T>>,
    /// Set the value callback
    pub set_value: Callback<Option<SelectCursor<T>>>,
    /// A list of options with their states
    pub options: Signal<Vec<OptionState<T>>>,
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
}

impl<T: Clone + PartialEq + 'static> SelectContext<T> {
    /// Select the currently focused item
    pub fn select_current_item(&mut self) {
        // If the select is open, select the focused item
        if self.open.cloned() {
            if let Some(focused_index) = self.focus_state.current_focus() {
                let options = self.options.read();
                if let Some(option) = options.iter().find(|opt| opt.tab_index == focused_index) {
                    self.set_value.call(Some(SelectCursor {
                        value: option.value.clone(),
                        text_value: option.text_value.clone(),
                    }));
                }
            }
        }
    }

    /// Learn from a keyboard event mapping physical key to logical character
    pub fn learn_from_keyboard_event(&mut self, physical_code: &str, logical_char: char) {
        let mut adaptive = self.adaptive_keyboard.write();
        adaptive.learn_from_event(physical_code, logical_char);
    }

    /// Record a user correction to improve future matching
    #[allow(dead_code)]
    pub fn record_user_correction(&mut self, typed: char, intended: char) {
        let mut adaptive = self.adaptive_keyboard.write();
        adaptive.record_correction(typed, intended);
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

        // Spawn a new task to clear the buffer after 1 second
        let new_task = spawn(async move {
            sleep(Duration::from_millis(1000)).await;

            // Clear the buffer
            typeahead_buffer_signal.write().clear();

            // Remove our own task handle to indicate no task is active
            typeahead_clear_task_signal.write().take();
        });

        // Store the new task handle
        self.typeahead_clear_task.write().replace(new_task);

        // Focus the best match using adaptive keyboard
        let options = self.options.read();
        let adaptive = self.adaptive_keyboard.read();
        let keyboard_layout = KeyboardLayout::Adaptive(adaptive.clone());

        if let Some(best_match_index) =
            super::text_search::best_match(&keyboard_layout, &typeahead, &options)
        {
            self.focus_state.set_focus(Some(best_match_index));
        }
    }
}

/// State for individual select options
pub(super) struct OptionState<T> {
    /// Tab index for focus management
    pub tab_index: usize,
    /// The value of the option
    pub value: T,
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

/// Context for select group components
#[derive(Clone, Copy)]
pub(super) struct SelectGroupContext {
    /// ID of the element that labels this group
    pub labeled_by: Signal<Option<String>>,
}
