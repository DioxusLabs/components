# Accordion

The accordion component is used to display collapsible content panels for presenting information in a limited amount of space. It allows users to expand and collapse sections to view more or less content.

## Component Structure

```rust
// The accordion component must wrap all accordion items.
Accordion {
    // Each accordion item contains both a trigger the expanded contents of the item.
    AccordionItem {
        // Each item must have an index starting from 0 to control where the item is placed.
        index: 0,
        // The trigger is used to expand or collapse the item.
        AccordionTrigger {}
        // The content that is shown when the item is expanded.
        AccordionContent {}
    }
}
```