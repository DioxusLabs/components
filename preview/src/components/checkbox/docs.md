# Checkbox

The Checkbox component is used to create an accessible checkbox input.

## Component Structure

```rust
// The checkbox component creates a checkbox button
Checkbox {
    // The checkbox indicator is a child component which will only be visible when the checkbox is checked.
    CheckboxIndicator {
        // The content of the checkbox indicator, typically an icon or image.
        {children}
    }
}
```
