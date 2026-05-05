//! Defines the [`Combobox`] component and its sub-components — a button +
//! popover combination that lets the user pick a value from a filterable list
//! of options. It's the keyboard-and-typeahead cousin of
//! [`Select`](crate::select::Select).
//!
//! ## Parts
//!
//! - [`Combobox`] — root container, state owner.
//! - [`ComboboxTrigger`] — button that opens the popup.
//! - [`ComboboxValue`] — displays the selected option's text in the trigger.
//! - [`ComboboxContent`] — the popup container.
//! - [`ComboboxInput`] — the search input.
//! - [`ComboboxList`] — `role="listbox"` container for options.
//! - [`ComboboxOption`] — a single selectable option.
//! - [`ComboboxItemIndicator`] — visible only when its option is selected.
//! - [`ComboboxGroup`] / [`ComboboxGroupLabel`] — option grouping.
//! - [`ComboboxEmpty`] — fallback content shown when no options match.
//!
//! ## Example
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_primitives::combobox::{
//!     Combobox, ComboboxContent, ComboboxEmpty, ComboboxInput, ComboboxItemIndicator,
//!     ComboboxList, ComboboxOption, ComboboxTrigger, ComboboxValue,
//! };
//!
//! #[component]
//! fn Demo() -> Element {
//!     rsx! {
//!         Combobox::<String> {
//!             placeholder: "Select a framework...",
//!             ComboboxTrigger { ComboboxValue {} }
//!             ComboboxContent {
//!                 ComboboxInput { placeholder: "Search frameworks..." }
//!                 ComboboxList {
//!                     ComboboxEmpty { "No framework found." }
//!                     ComboboxOption::<String> {
//!                         index: 0usize,
//!                         value: "next",
//!                         "Next.js"
//!                         ComboboxItemIndicator { "✔" }
//!                     }
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```

mod components;
mod context;

pub use components::{
    use_combobox_content_visible, Combobox, ComboboxContent, ComboboxContentProps,
    ComboboxEmpty, ComboboxEmptyProps, ComboboxGroup, ComboboxGroupLabel,
    ComboboxGroupLabelProps, ComboboxGroupProps, ComboboxInput, ComboboxInputProps,
    ComboboxItemIndicator, ComboboxItemIndicatorProps, ComboboxList, ComboboxListProps,
    ComboboxOption, ComboboxOptionProps, ComboboxProps, ComboboxTrigger, ComboboxTriggerProps,
    ComboboxValue, ComboboxValueProps,
};

pub use context::default_combobox_filter;
