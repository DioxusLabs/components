The `Collapsible` component allows users to create expandable sections that can be toggled open or closed. This is useful for displaying content in a compact manner, such as FAQs, menus, or any content that benefits from being hidden until needed.

## Component Structure

```rust
// The collapsible component must wrap all collapsible items.
Collapsible {
    // The trigger is used to expand or collapse the item.
    CollapsibleTrigger {}
    // The content that is shown when the item is expanded.
    CollapsibleContent {}
}
```
