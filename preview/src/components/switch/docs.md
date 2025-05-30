The Switch component allows users to toggle between two states, such as on and off.

## Component Structure

```rust
// The Switch component wraps the switch thumb
Switch {
    // The current state of the switch, true for on and false for off.
    checked: true,
    // Callback function triggered when the switch state changes.
    on_checked_change: |checked: bool| {
        // Handle the change in switch state.
    },
    // The switch thumb represents the draggable handle that the user moves to toggle the switch.
    SwitchThumb {
        // The content of the thumb
        {children}
    }
}
```