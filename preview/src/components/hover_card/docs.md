The HoverCard component can be used to display additional information when a user hovers over an element. It is useful for showing tooltips, additional details, or any other content that should be revealed on hover.

## Component Structure

```rust
// The HoverCard component wraps the trigger element and the content that will be displayed on hover.
HoverCard {
    // The HoverCardTrigger contains the elements that will trigger the hover card to display when hovered.
    HoverCardTrigger {
        // The elements that will trigger the hover card when hovered over.
        {children}
    }
    // The HoverCardContent contains the content that will be displayed when the user hovers over the trigger.
    HoverCardContent {
        // The side of the HoverCardTrigger where the content will be displayed. Can be one Top, Right, Bottom, or Left.
        side: HoverCardSide::Bottom,
        // The alignment of the HoverCardContent relative to the HoverCardTrigger. Can be one of Start, Center, or End.
        align: HoverCardAlign::Start,
        // The content of the hover card, which can include text, images, or any other elements.
        {children}
    }
}
```