The Select component is used to create a dropdown menu that allows users to select one or more options from the select groups.

## Component Structure

```rust
// The Select component wraps all select items in the dropdown.
Select::<String> {
    // The currently selected value(s) in the dropdown.
    value: "option1",
    // Callback function triggered when the selected value changes.
    on_value_change: |value: String| {
        // Handle the change in selected value.
    },
    // The select trigger is the button that opens the dropdown.
    SelectTrigger {
        // The (optional) select value displays the currently selected text value.
        SelectValue {}
    }
    // All groups must be wrapped in the select list.
    SelectList {
        // An group within the select dropdown which may contain multiple items.
        SelectGroup {
            // The label for the group
            SelectGroupLabel {
                "Other"
            }
            // Each select option represents an individual option in the dropdown. The type must match the type of the select.
            SelectOption::<String> {
                // The value of the item, which will be passed to the on_value_change callback when selected.
                value: "option1",
                // Select item indicator is only rendered if the item is selected.
                SelectItemIndicator {
                    "✔️"
                }
            }
        }
    }
}
```