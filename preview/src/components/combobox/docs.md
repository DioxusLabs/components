The Combobox component is an autocomplete input with a popover list. It lets users select a value from a filterable list of options — like a [Select](../select), but with a search input baked in.

## Component Structure

```rust
Combobox::<String> {
    // The currently selected value.
    value: "next",
    // Fired when the value changes.
    on_value_change: |value: Option<String>| {
        // Handle the change.
    },
    // The trigger button shows the selected value (or placeholder).
    ComboboxTrigger { ComboboxValue {} }
    // The popover wraps the search input and the option list.
    ComboboxContent {
        // The search input filters the list as the user types.
        ComboboxInput { placeholder: "Search frameworks..." }
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
