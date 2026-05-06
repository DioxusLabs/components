//! Autocomplete input with a filterable popup list.
//!
//! `ComboboxInput` is the text input and trigger. `ComboboxContent` contains a
//! `ComboboxList` with `ComboboxOption` children.

mod components;
mod context;

pub use components::{
    Combobox, ComboboxContent, ComboboxContentProps, ComboboxEmpty, ComboboxEmptyProps,
    ComboboxInput, ComboboxInputProps, ComboboxItemIndicator, ComboboxItemIndicatorProps,
    ComboboxList, ComboboxListProps, ComboboxOption, ComboboxOptionProps, ComboboxProps,
};

pub use context::default_combobox_filter;
