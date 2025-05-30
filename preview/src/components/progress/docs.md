The Progress component is used to display the progress of a task or operation. It can be used to indicate loading states, file uploads, or any other process that takes time to complete.

## Component Structure

```rust
Progress {
    // The current progress value (0 to max
    value: 0.5,
    // The maximum value of the progress (default is 100.0)
    max: 1.0,
    // Elements that will be displayed inside the progress bar
    {children}
}
```