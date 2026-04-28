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

    // Enable built-in dialog content (ColorPickerSelect).
    // This is the default.
    use_default_dialog: true,
}
```

### Custom Dialog

If you want to fully replace the default dialog UI, disable the built-in content
and pass your custom UI via `children` to `ColorPicker`:

```rust
ColorPicker {
    color,
    on_color_change: move |c: Color| { /* ... */ },

    use_default_dialog: false,

    // Your custom dialog UI:
    div {
        "Custom color picker content"
    }
}
```

Note: if `use_default_dialog: true`, do not add `ColorPickerSelect` to `children`,
otherwise it will render twice.