The Combobox component is an autocomplete input with a popover list. It lets users select a value from a filterable list of options — like a [Select](../select), but with a search input baked in.

Following WAI-ARIA 1.2's combobox pattern, the input itself is the trigger: typing, clicking, or pressing arrow keys opens the popup, and DOM focus stays on the input the whole time.

## Component Structure

```rust
Combobox::<String> {
    // The currently selected value.
    value: "next",
    // Fired when the value changes.
    on_value_change: |value: Option<String>| {
        // Handle the change.
    },
    // The input doubles as the trigger and the search field. When closed it
    // shows the selected option's text; when open it shows the typed query.
    ComboboxInput { placeholder: "Select a framework..." }
    // The popover wraps just the option list.
    ComboboxContent {
        ComboboxList {
            // Shown only when nothing matches the current query.
            ComboboxEmpty { "No framework found." }
            // Each option's text_value (or value, if it's a string) is what
            // the input filters against.
            ComboboxOption::<String> {
                index: 0,
                value: "next",
                "Next.js"
                ComboboxItemIndicator { "✔" }
            }
        }
    }
}
```
