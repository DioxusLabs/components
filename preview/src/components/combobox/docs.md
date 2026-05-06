The Combobox component is an autocomplete input with a filterable popup list.

## Component Structure

```rust
Combobox::<String> {
    value: "next",
    on_value_change: |value: Option<String>| {
        // Handle the change.
    },
    ComboboxInput { placeholder: "Select a framework..." }
    ComboboxContent {
        ComboboxList {
            ComboboxEmpty { "No framework found." }
            ComboboxOption::<String> {
                index: 0,
                value: "next",
                text_value: "Next.js",
                "Next.js"
                ComboboxItemIndicator { "✔" }
            }
        }
    }
}
```
