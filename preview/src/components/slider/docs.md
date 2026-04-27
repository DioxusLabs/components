The slider component allows users to select a value from a range by sliding a handle along a track.

## Component Structure

```rust
// The slider component wraps all slider-related elements.
Slider {
    // The current value of the slider, which should be updated as the user interacts with the slider.
    value: 0.0,
    // The orientation of the slider, true for horizontal and false for vertical.
    horizontal: true,
    // Callback function triggered when the slider value changes.
    on_value_change: |value: f64| {
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
            // The content of the thumb button
            {children}
        }
    }
}
```

For a two-thumb range selector, use `RangeSlider` with two `SliderThumb`s, one at `index: 0` and one at `index: 1`:

```rust
use dioxus_primitives::slider::SliderRangeValue;

RangeSlider {
    default_value: SliderRangeValue::new(20.0, 80.0),
    on_value_change: |value: SliderRangeValue| {
        // value.start() and value.end() give the two endpoints
    },
    SliderTrack {
        SliderRange {}
        SliderThumb { index: 0 }
        SliderThumb { index: 1 }
    }
}
```