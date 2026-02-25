The RecycleList component virtualizes large lists by rendering only visible rows plus a buffer. It is useful for long, dynamic-height datasets where rendering every row at once would be expensive.

## Component Structure

```rust
{RecycleList(RecycleListProps {
    // Full data source.
    items: rows.as_slice(),
    // Render buffer (approximate row count) above and below viewport.
    buffer: 8,
    // Row renderer callback receives (item, absolute_index).
    render_item: move |row, idx| rsx! {
        article { key: "{idx}", "{row.title}" }
    },
})}
```
