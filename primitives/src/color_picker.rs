//! Defines the [`ColorPicker`] component and its sub-components.

use crate::dioxus_elements::geometry::{ClientPoint, Pixels};
use crate::dioxus_elements::input_data::MouseButton;
use crate::pointer;
use dioxus::html::geometry::euclid::{Rect, Size2D, Vector2D};
use dioxus::html::geometry::PixelsSize;
use dioxus::prelude::*;
use palette::{encoding, FromColor, Hsv, RgbHue, Srgb};

use std::rc::Rc;

mod color_naming;
use color_naming::color_name;

/// Represents an sRGB color.
pub type Color = Srgb<u8>;

const COLOR_AREA_MIN: f64 = 0.0;
const COLOR_AREA_MAX: f64 = 100.0;
const COLOR_AREA_RANGE: f64 = COLOR_AREA_MAX - COLOR_AREA_MIN;

fn color_hex(color: Color) -> String {
    format!("#{color:X}")
}

/// Context provided by [`ColorPicker`] to its descendants.
///
/// The picker is controlled in HSV — [`Self::color`] echoes the controlled
/// prop, and the setter methods emit `on_color_change` after applying the
/// requested edit on top of the current value.
#[derive(Clone, Copy)]
pub struct ColorPickerContext {
    color: ReadSignal<Hsv<encoding::Srgb, f64>>,
    on_color_change: Callback<Hsv<encoding::Srgb, f64>>,
}

impl ColorPickerContext {
    /// Read the current HSV color.
    pub fn color(&self) -> Hsv<encoding::Srgb, f64> {
        (self.color)()
    }

    /// Replace the entire HSV color.
    pub fn set_color(&self, c: Hsv<encoding::Srgb, f64>) {
        self.on_color_change.call(c);
    }

    /// Set hue, keeping saturation and value.
    pub fn set_hue(&self, h: f64) {
        let current = (self.color)();
        self.on_color_change.call(Hsv::<encoding::Srgb, f64>::new(
            RgbHue::new(h),
            current.saturation,
            current.value,
        ));
    }

    /// Set saturation and value as a pair, keeping hue.
    pub fn set_sv(&self, s: f64, v: f64) {
        let current = (self.color)();
        self.on_color_change
            .call(Hsv::<encoding::Srgb, f64>::new(current.hue, s, v));
    }
}

/// The props for the [`ColorPicker`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ColorPickerProps {
    /// The selected color
    #[props(default)]
    pub color: ReadSignal<Hsv<encoding::Srgb, f64>>,

    /// Callback when color changes
    #[props(default)]
    pub on_color_change: Callback<Hsv<encoding::Srgb, f64>>,

    /// Whether the color picker is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Optional label on the button
    #[props(default)]
    pub label: Option<String>,

    /// Render built-in dialog content (`ColorPickerSelect`) before `children`.
    #[props(default = true)]
    pub use_default_dialog: bool,

    /// Additional attributes to extend the color picker element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color picker element
    pub children: Element,
}

/// # ColorPicker
///
/// The [`ColorPicker`] component provides an accessible color input interface
/// and synchronizes a color value between multiple color components.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::color_picker::*;
/// #[component]
/// fn Demo() -> Element {
///    use palette::{IntoColor, encoding};
///    let mut color = use_signal(|| -> palette::Hsv<encoding::Srgb, f64> {
///        Color::new(155, 128, 255).into_format::<f64>().into_color()
///    });
///    rsx! {
///            ColorPicker {
///                label: "Pick",
///                color: color(),
///                on_color_change: move |c| {
///                    tracing::info!("Color changed: {:?}", c);
///                    color.set(c);
///                },
///            }
///    }
///}
/// ```
///
/// # Styling
///
/// The [`ColorPicker`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the ColorPicker is disabled. Possible values are `true` or `false`.
#[component]
pub fn ColorPicker(props: ColorPickerProps) -> Element {
    use_context_provider(|| ColorPickerContext {
        color: props.color,
        on_color_change: props.on_color_change,
    });

    rsx! {
        div {
            role: "group",
            aria_label: "Color picker",
            "data-disabled": (props.disabled)(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`ColorArea`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ColorAreaProps {
    /// The step value
    #[props(default = 1.0)]
    pub step: ReadSignal<f64>,

    /// Additional attributes to extend the color area element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color area element
    pub children: Element,
}

/// # ColorArea
///
/// The [`ColorArea`] allows users to adjust two channels of color value against a two-dimensional gradient background.
/// It is part of `ColorPickerSelect`
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::color_picker::*;
/// #[component]
/// fn Demo() -> Element {
///    use palette::{IntoColor, encoding};
///    let mut color = use_signal(|| -> palette::Hsv<encoding::Srgb, f64> {
///        Color::new(155, 128, 255).into_format::<f64>().into_color()
///    });
///    rsx! {
///            ColorPicker {
///                label: "Pick",
///                color: color(),
///                on_color_change: move |c| {
///                    tracing::info!("Color changed: {:?}", c);
///                    color.set(c);
///                },
///            }
///    }
///}
/// ```
#[component]
pub fn ColorArea(props: ColorAreaProps) -> Element {
    let picker_ctx = use_context::<ColorPickerContext>();
    let mut dragging = use_signal(|| false);

    // Thumb position is read straight from HSV state — no RGB round-trip
    // means saturation is preserved at brightness=0.
    let value = use_memo(move || {
        let hsv = picker_ctx.color();
        ClientPoint::new(hsv.saturation, hsv.value) * COLOR_AREA_RANGE
    });

    let update_xy = use_callback(move |point: ClientPoint| {
        let scaled = point / COLOR_AREA_RANGE;
        picker_ctx.set_sv(scaled.x, scaled.y);
    });

    let ctx = use_context_provider(|| ColorAreaContext {
        value,
        set_value: update_xy,
        step: props.step,
        dragging: dragging.into(),
    });

    let mut rect = use_signal(|| None);
    let mut div_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let mut granular_value = use_hook(|| CopyValue::new(value()));

    let size = rect().map(|r: Rect<f64, Pixels>| r.size);

    let mut current_pointer_id: Signal<Option<i32>> = use_signal(|| None);
    let mut last_processed_pos = use_hook(|| CopyValue::new(None));

    use_effect(move || {
        if !dragging() {
            return;
        }

        let Some(size) = size else {
            return;
        };

        let Some(active_pointer_id) = current_pointer_id() else {
            return;
        };

        let Some(pointer_position) = pointer::pointer_position(active_pointer_id) else {
            current_pointer_id.take();
            last_processed_pos.set(None);
            dragging.set(false);
            return;
        };

        let delta = if let Some(last_pos) = last_processed_pos.replace(Some(pointer_position)) {
            pointer_position - last_pos
        } else {
            Vector2D::zero()
        };

        let range = ctx.range_size();
        let d_s = delta.x / size.width * range;
        let d_h = delta.y / size.height * range;

        let new_value = granular_value() + Size2D::new(d_s, -d_h);
        granular_value.set(new_value);
        ctx.set_value.call(ctx.clamp_and_snap(new_value));
    });

    rsx! {
        div {
            role: "group",
            onmounted: move |e| async move {
                // Get the bounding rect of the area
                if let Ok(r) = e.data().get_client_rect().await {
                    rect.set(Some(r));
                }
                div_element.set(Some(e.data()));
            },
            onresize: move |_| async move {
                // Update the rect on resize
                let Some(div_element) = div_element() else {
                    return;
                };
                if let Ok(r) = div_element.get_client_rect().await {
                    rect.set(Some(r));
                }
            },
            onpointerdown: move |e| {
                // Prevent default to avoid loosing focus on the range
                e.prevent_default();
                e.stop_propagation();

                if current_pointer_id.read().is_some() || e.trigger_button() != Some(MouseButton::Primary) {
                    return;
                }

                current_pointer_id.set(Some(e.data().pointer_id()));
                pointer::track_pointer_down(e.data().pointer_id(), e.client_coordinates());

                // Handle pointer interaction
                spawn(async move {
                    let Some(div_element) = div_element() else {
                        return;
                    };

                    // Update the bounding rect of the slider in case it moved
                    if let Ok(r) = div_element.get_client_rect().await {
                        rect.set(Some(r));

                        let size = r.size;

                        // Get the mouse position relative to the slider
                        let top_left = r.origin;
                        let relative_pos = e.client_coordinates() - top_left.cast_unit();

                        let range = ctx.range_size();

                        let x = (relative_pos.x / size.width) * range;
                        let y = COLOR_AREA_MAX - ((relative_pos.y / size.height) * range);
                        let pt = ClientPoint::new(x, y);
                        granular_value.set(pt);
                        ctx.set_value.call(ctx.clamp_and_snap(pt));
                    }

                    dragging.set(true);
                });
            },
            ..props.attributes,
            AreaTrack {
                style: format!(
                    "--area-color: {}",
                    color_hex(
                        Srgb::<f64>::from_color(Hsv::<encoding::Srgb, f64>::new(
                            picker_ctx.color().hue,
                            1.0,
                            1.0,
                        ))
                        .into_format()
                    )
                ),
                AreaThumb {
                    background_color: color_hex(
                        Srgb::<f64>::from_color(picker_ctx.color()).into_format(),
                    ),
                }
            }
            {props.children}
        }
    }
}

/// The props for the [`AreaTrack`] component
#[derive(Props, Clone, PartialEq)]
struct AreaTrackProps {
    /// Additional attributes to apply to the track element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the track which should include a [`AreaThumb`]
    pub children: Element,
}

#[component]
fn AreaTrack(props: AreaTrackProps) -> Element {
    rsx! {
        div {
            class: "dx-color-area-track",
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`AreaThumb`] component
#[derive(Props, Clone, PartialEq)]
struct AreaThumbProps {
    /// Additional attributes to apply to the thumb element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the thumb element
    pub children: Element,
}

#[component]
fn AreaThumb(props: AreaThumbProps) -> Element {
    let picker_ctx = use_context::<ColorPickerContext>();
    let ctx = use_context::<ColorAreaContext>();

    let mut button_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    use_effect(move || {
        if let Some(button) = button_ref() {
            let dragging = ctx.dragging.cloned();
            if dragging {
                spawn(async move {
                    _ = button.set_focus(true).await;
                });
            }
        }
    });

    let percent = ctx.as_percent((ctx.value)());
    let style = format!(
        "left: {:.2}%; top: {:.2}%;",
        percent.width,
        100. - percent.height
    );
    let current = (ctx.value)();
    let min = COLOR_AREA_MIN;
    let max = COLOR_AREA_MAX;
    let step = (ctx.step)();
    let color_label = color_name(Srgb::<f64>::from_color(picker_ctx.color()).into_format());

    rsx! {
        div {
            class: "dx-color-area-thumb",
            role: "presentation",
            aria_label: "Color area thumb",
            "data-dragging": ctx.dragging,
            style,
            tabindex: 0,
            onmounted: move |evt| {
                // Store the mounted data for focus management
                button_ref.set(Some(evt.data()));
            },
            onmousedown: move |evt| {
                // Don't focus the button. The dragging state will handle focus
                evt.prevent_default();
            },
            ontouchstart: move |evt| {
                // Don't focus the button. The dragging state will handle focus
                evt.prevent_default();
            },
            onkeydown: move |evt| async move {
                let key = evt.data().key();
                let mut step = (ctx.step)();
                if evt.data().modifiers().shift() {
                    // If shift is pressed, increase the step size
                    step *= 10.0;
                }

                // Handle keyboard navigation
                let new_value = (ctx.value)() + match key {
                    Key::ArrowUp => {
                        Vector2D::new(0.0, step)
                    }
                    Key::ArrowDown => {
                        Vector2D::new(0.0, -step)
                    }
                    Key::ArrowRight => {
                        Vector2D::new(step, 0.0)
                    }
                    Key::ArrowLeft => {
                        Vector2D::new(-step, 0.0)
                    }
                    _ => return,
                };

                evt.prevent_default();
                // Clamp and snap the new value
                ctx.set_value.call(ctx.clamp_and_snap(new_value));
            },
            ..props.attributes,
            input {
                class: "dx-color-area-input",
                r#type: "range",
                aria_label: "Saturation",
                aria_roledescription: "2D Slider",
                aria_valuetext: format!("Saturation {:.0}%, {color_label}", percent.width),
                aria_orientation: "horizontal",
                min: "{min}",
                max: "{max}",
                step: "{step}",
                value: format!("{}", current.x),
            }
            input {
                class: "dx-color-area-input",
                r#type: "range",
                aria_label: "Value",
                aria_roledescription: "2D Slider",
                aria_valuetext: format!("Value {:.0}%, {color_label}", percent.height),
                aria_orientation: "vertical",
                min: "{min}",
                max: "{max}",
                step: "{step}",
                value: format!("{}", current.y),
            }
            {props.children}
        }
    }
}

#[derive(Copy, Clone)]
struct ColorAreaContext {
    value: Memo<ClientPoint>,
    set_value: Callback<ClientPoint>,
    step: ReadSignal<f64>,
    dragging: ReadSignal<bool>,
}

impl ColorAreaContext {
    fn range_size(&self) -> f64 {
        COLOR_AREA_RANGE
    }

    fn snap(&self, value: ClientPoint) -> ClientPoint {
        let step = (self.step)();
        value.map(|v| (v / step).round() * step)
    }

    fn clamp_and_snap(&self, value: ClientPoint) -> ClientPoint {
        let clamped = value.map(|v| v.clamp(COLOR_AREA_MIN, COLOR_AREA_MAX));
        self.snap(clamped)
    }

    fn as_percent(&self, value: ClientPoint) -> PixelsSize {
        let size = self.range_size();
        let scaled = value.map(|v| ((v - COLOR_AREA_MIN) / size * 100.0).clamp(0.0, 100.0));
        PixelsSize::new(scaled.x, scaled.y)
    }
}

#[cfg(test)]
mod tests {
    use super::Color;
    use palette::{encoding, FromColor, Hsv, IntoColor, RgbHue, Srgb};

    fn hsv_to_rgb(hsv: Hsv<encoding::Srgb, f64>) -> Color {
        Srgb::<f64>::from_color(hsv).into_format()
    }

    #[test]
    fn hsv_to_rgb_primaries() {
        let red = Hsv::<encoding::Srgb, f64>::new(RgbHue::new(0.0), 1.0, 1.0);
        let green = Hsv::<encoding::Srgb, f64>::new(RgbHue::new(120.0), 1.0, 1.0);
        let blue = Hsv::<encoding::Srgb, f64>::new(RgbHue::new(240.0), 1.0, 1.0);

        assert_eq!(hsv_to_rgb(red), Color::new(255, 0, 0));
        assert_eq!(hsv_to_rgb(green), Color::new(0, 255, 0));
        assert_eq!(hsv_to_rgb(blue), Color::new(0, 0, 255));
    }

    #[test]
    fn rgb_to_hsv_round_trip_primaries() {
        for rgb in [
            Color::new(255, 0, 0),
            Color::new(0, 255, 0),
            Color::new(0, 0, 255),
        ] {
            let hsv: Hsv<encoding::Srgb, f64> = rgb.into_format::<f64>().into_color();
            assert_eq!(hsv_to_rgb(hsv), rgb);
        }
    }
}
