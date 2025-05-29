The ScrollArea component is used to create a scrollable container for its children. It can be used to enable scrolling for content that overflows the available space.

## Component Structure

```rust
// The ScrollArea component wraps all scrollable content.
ScrollArea {
    // The direction in which the scroll area can scroll. Can be one of Horizontal, Vertical, or Both.
    scroll_direction: ScrollDirection::Vertical,
    // The content of the scrollable area
    {children}
}
```