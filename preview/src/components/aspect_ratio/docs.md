The AspectRatio component is used to maintain a specific aspect ratio for its children. This is particularly useful for responsive designs where you want to ensure that an element retains its proportions regardless of the screen size.

## Component Structure

```rust
AspectRatio {
    // The aspect ratio to maintain (width / height)
    ratio: 4. / 3.,
    // The children of the AspectRatio component will be rendered within it.
    {children}
}
```
