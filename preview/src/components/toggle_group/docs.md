The Toggle Group component is used to create a group of toggle buttons that allows the user to select one or more options from a set. It is useful for creating a set of options.

## Component Structure

```rust
// The ToggleGroup component wraps all toggle items in the group.
ToggleGroup {
    // The orientation of the toggle group, true for horizontal and false for vertical.
    horizontal: true,
    // The toggle item represents each individual toggle button in the group.
    ToggleItem {
        // The index of the toggle item, used to determine the order in which items are focused.
        index: 0,
        // The contents of the toggle item button
        {children}
    }
}
```
