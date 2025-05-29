# Context Menu

The context menu component can be used to define a context menu that is displayed when the user right-clicks on an element. It can contain various menu items that the user can interact with.

## Component Structure

```rust
// The context menu component must wrap all context menu items.
ContextMenu {
    // The context menu trigger is the element that will display the context menu when right-clicked.
    ContextMenuTrigger {
        // The content of the trigger
        {children}
    }
    // The context menu content contains all the items that will be displayed in the context menu.
    ContextMenuContent {
        // Each context menu item represents an individual action in the context menu. Items are displayed in order based on the order of the index property.
        ContextMenuItem {
            // The index of the item, used to determine the order in which items are displayed.
            index: 0,
            // The value of the item which will be passed to the on_select callback when the item is selected.
            value: "",
            on_select: |value: String| {
                // This callback is triggered when the item is selected.
                // The value parameter contains the value of the selected item.
            },
        }
    }
}
```