//! Defines the [`Select`] component and its sub-components, which provide a searchable select input with keyboard navigation.
//!
//! The Select component consists of several parts that work together:
//! - [`Select`] - The root container component
//! - [`SelectTrigger`] - The button that opens/closes the dropdown
//! - [`SelectList`] - The dropdown container for options
//! - [`SelectOption`] - Individual selectable options
//! - [`SelectItemIndicator`] - Visual indicator for selected items
//! - [`SelectGroup`] - Groups related options together
//! - [`SelectGroupLabel`] - Labels for option groups
//!
//! ## Features
//!
//! - **Keyboard Navigation**: Full keyboard support with arrow keys, home/end, enter, and escape
//! - **Typeahead Search**: Smart text search that adapts to different keyboard layouts
//! - **Accessibility**: ARIA compliant with proper roles and attributes
//! - **Customizable**: Flexible styling through data attributes and CSS
//! - **Focus Management**: Automatic focus handling and restoration
//!
//! ## Example
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_primitives::select::{
//!     Select, SelectGroup, SelectGroupLabel, SelectItemIndicator,
//!     SelectList, SelectOption, SelectTrigger,
//! };
//!
//! #[component]
//! fn Demo() -> Element {
//!     rsx! {
//!         Select::<String> {
//!             placeholder: "Select a fruit...",
//!             SelectTrigger::<String> {
//!                 aria_label: "Select Trigger",
//!                 width: "12rem",
//!             }
//!             SelectList::<String> {
//!                 aria_label: "Select Demo",
//!                 SelectGroup::<String> {
//!                     SelectGroupLabel { "Fruits" }
//!                     SelectOption::<String> {
//!                         index: 0usize,
//!                         value: "apple".to_string(),
//!                         display: "Apple".to_string(),
//!                         "Apple"
//!                         SelectItemIndicator { "✔️" }
//!                     }
//!                     SelectOption::<String> {
//!                         index: 1usize,
//!                         value: "banana".to_string(),
//!                         display: "Banana".to_string(),
//!                         "Banana"
//!                         SelectItemIndicator { "✔️" }
//!                     }
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```

// Internal modules
mod context;
mod text_search;

// Public components module
pub mod components;

// Re-export all public components and types
pub use components::{
    Select, SelectGroup, SelectGroupLabel, SelectGroupLabelProps, SelectGroupProps,
    SelectItemIndicator, SelectItemIndicatorProps, SelectList, SelectListProps, SelectOption,
    SelectOptionProps, SelectProps, SelectTrigger, SelectTriggerProps,
};
