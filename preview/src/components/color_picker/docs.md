Color picker used when the user needs to make a customized color selection.

## Component Structure

```rust
ColorPicker {
    // The currently selected color in the color picker.
    color,
    on_color_change: move |c: Color| {
        // This callback is triggered when a color is selected in the popover dialog.
        // The color parameter contains the selected color.
    },

    // The ColorPickerSelect (standard color picker container) contains the content that will be displayed when the user
    // clicks on the trigger, can be replaced with a custom implementation
    ColorPickerSelect { }
}
```