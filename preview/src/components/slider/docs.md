The slider component allows users to select a value from a range by sliding a handle along a track.

## Component Structure

```rust
// The slider component wraps all slider-related elements.
Slider {
    // The current value of the slider, which should be updated as the user interacts with the slider.
    value: SliderValue::Single(0.0),
    // The orientation of the slider, true for horizontal and false for vertical.
    horizontal: true,
    // Callback function triggered when the slider value changes.
    on_value_change: |value: u32| {
        // Handle the change in slider value.
    },
    // The track represents the visual track along which the handle moves.
    SliderTrack {
        // The slider range represents the filled portion of the track
        SliderRange {
            // The content of the range
            {children}
        }
        // The slider thumb represents the draggable handle that the user moves along the track.
        SliderThumb {
            // An optional index which can be either 0 or 1 to indicate if this is the first or second thumb in a range slider.
            index: 0,
            // The content of the thumb button
            {children}
        }
    }
}
```