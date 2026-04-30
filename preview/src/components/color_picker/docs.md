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

    // Optional label on the trigger button.
    label: "Pick",
}
```

### Custom Dialog

`ColorPicker` renders the default trigger and `ColorPickerSelect` dialog. If you
want to fully replace the default dialog UI, compose the picker from
`ColorPickerRoot`, `ColorPickerTrigger`, and `ColorPickerDialog`:

```rust
ColorPickerRoot {
    color,
    on_color_change: move |c: Color| { /* ... */ },

    ColorPickerTrigger {
        label: "Pick",
    }

    ColorPickerDialog {
        div {
            "Custom color picker content"
        }
    }
}
```

If you only need to add content after the default controls, pass children to
`ColorPicker`; they will be appended inside the default dialog.
