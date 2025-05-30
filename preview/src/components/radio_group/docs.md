The RadioGroup component is used to create a group of radio buttons that allows the user to select one option from a set.

## Component Structure

```rust
// The RadioGroup component wraps all radio items in the group.
RadioGroup {
    // The value property represents the currently selected radio button in the group.
    value: "option1",
    on_value_change: |value: String| {
        // This callback is triggered when the selected radio button changes.
        // The value parameter contains the value of the newly selected radio button.
    },
    // The RadioItem component represents each individual radio button in the group.
    RadioItem {
        // The index of the radio item, used to determine the order in which items are displayed.
        index: 0,
        // The value of the radio button, which is used to identify the selected option and will be passed to the on_value_change callback when selected.
        value: "option1",
        // The contents of the radio item button
        {children}
    }
}
```