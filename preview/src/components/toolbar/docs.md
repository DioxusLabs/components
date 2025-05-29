# Toolbar

The toolbar component is a flexible and customizable component that can be used to create a variety of toolbars. It can be used to create a simple toolbar with a title, or a more complex toolbar with multiple buttons and actions.

## Component Structure

```rust
// The Toolbar component wraps all toolbar items.
Toolbar {
    // The aria_label of the toolbar, used for accessibility purposes.
    aria_label: "Toolbar Title",
    // The ToolbarButton component represents each individual button in the toolbar.
    ToolbarButton {
        // The index of the toolbar button, used to determine the order in which buttons are focused.
        index: 0,
        on_click: |_: ()| {
            // This callback is triggered when the button is clicked.
        },
        // The contents of the toolbar button
        {children}
    }
    // The ToolbarSeparator component represents a separator line in the toolbar.
    ToolbarSeparator {
        // The orientation of the separator, true for horizontal and false for vertical.
        horizontal: true,
        // The decorative property controls if the separator is decorative and should not be visible to screen readers.
        decorative: false,
    }
}
```