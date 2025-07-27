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
//! ## Typeahead Buffer Behavior
//!
//! The Select component implements an intelligent typeahead search system with race condition prevention:
//!
//! ### How it Works
//!
//! When users type characters while the dropdown is open:
//! 1. Each character is added to a typeahead buffer
//! 2. The buffer is used to find and focus the best matching option
//! 3. The buffer automatically clears after 1 second of inactivity
//!
//! ### Race Condition Prevention
//!
//! The component uses task cancellation to prevent a common race condition:
//! - **Problem**: Without cancellation, rapid typing (e.g., "apple") would spawn multiple clear timers,
//!   causing the first timer to clear the entire buffer after 1 second, losing later keystrokes.
//! - **Solution**: Each new keystroke cancels any existing clear timer before starting a new one.
//!   This ensures only the most recent timer remains active.
//!
//! ### Example Scenario
//!
//! ```text
//! User types "app" quickly:
//! - 0ms: Types 'a' → starts 1-second timer
//! - 100ms: Types 'p' → cancels first timer, starts new 1-second timer
//! - 200ms: Types 'p' → cancels second timer, starts new 1-second timer
//! - 1200ms: Buffer clears (only the final timer executes)
//! ```
//!
//! This behavior ensures the typeahead buffer remains intact during rapid typing while still
//! clearing after a period of inactivity, providing a smooth and predictable user experience.
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
//!                         text_value: "Apple".to_string(),
//!                         "Apple"
//!                         SelectItemIndicator { "✔️" }
//!                     }
//!                     SelectOption::<String> {
//!                         index: 1usize,
//!                         value: "banana".to_string(),
//!                         text_value: "Banana".to_string(),
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
