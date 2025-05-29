The Select component is used to create a dropdown menu that allows users to select one or more options from the select groups.

## Component Structure

```rust
// The Select component wraps all select items in the dropdown.
Select {
    // The currently selected value(s) in the dropdown.
    value: "option1",
    // Callback function triggered when the selected value changes.
    on_value_change: |value: String| {
        // Handle the change in selected value.
    },
    // An group within the select dropdown which may contain multiple items.
    SelectGroup {
        // The label for the group, which is displayed as a header in the dropdown.
        label: "Group 1",
        // Each select option represents an individual option in the dropdown.
        SelectOption {
            // The value of the item, which will be passed to the on_value_change callback when selected.
            value: "option1",
            // The content of the select option
            {children}
        }
    }
}
```