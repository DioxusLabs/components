The Combobox component is an autocomplete input with a filterable popup list.

Filtering preserves the order defined by the rendered `ComboboxOption` elements and their `index`
props. If you want query-dependent ranking, control `query`, sort your item data in user code,
render the options in that sorted order, and assign indexes from the sorted list.

## Component Structure

```rust
let mut value = use_signal(|| None::<String>);
let mut query = use_signal(String::new);

Combobox::<String> {
    value,
    on_value_change: move |next: Option<String>| {
        value.set(next);
    },
    query: Some(query()),
    on_query_change: move |next| query.set(next),
    ComboboxInput { placeholder: "Select a framework..." }
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
```
