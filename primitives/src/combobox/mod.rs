//! Defines the [`Combobox`] component and its sub-components — an
//! autocomplete input with a popover list of filterable options. The
//! keyboard-and-typeahead cousin of [`Select`](crate::select::Select).
//!
//! Following WAI-ARIA 1.2's combobox pattern: a single text [`ComboboxInput`]
//! is the trigger and the search field — `role="combobox"` lives there, the
//! listbox sits in a separate popup, and DOM focus never leaves the input.
//!
//! ## Parts
//!
//! - [`Combobox`] — single-select root container, state owner.
//! - [`ComboboxMulti`] — multi-select variant. Toggling an option keeps the
//!   popup open; the listbox advertises `aria-multiselectable="true"`.
//! - [`ComboboxInput`] — the input that opens the popup and filters options.
//! - [`ComboboxContent`] — the popup container.
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
//!     ComboboxList, ComboboxOption,
//! };
//!
//! #[component]
//! fn Demo() -> Element {
//!     rsx! {
//!         Combobox::<String> {
//!             ComboboxInput { placeholder: "Select a framework..." }
//!             ComboboxContent {
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
    Combobox, ComboboxContent, ComboboxContentProps, ComboboxEmpty, ComboboxEmptyProps,
    ComboboxGroup, ComboboxGroupLabel, ComboboxGroupLabelProps, ComboboxGroupProps, ComboboxInput,
    ComboboxInputProps, ComboboxItemIndicator, ComboboxItemIndicatorProps, ComboboxList,
    ComboboxListProps, ComboboxMulti, ComboboxMultiProps, ComboboxOption, ComboboxOptionProps,
    ComboboxProps,
};

pub use context::default_combobox_filter;
