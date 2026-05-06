The Combobox component is an autocomplete input with a filterable popup list.

## Component Structure

```rust
let mut value = use_signal(|| None::<String>);

Combobox::<String> {
    value,
    on_value_change: move |next: Option<String>| {
        value.set(next);
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
