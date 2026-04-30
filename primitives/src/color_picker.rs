//! Defines the [`ColorPicker`] component and its sub-components.

use crate::dioxus_core::{queue_effect, Runtime};
use crate::dioxus_elements::geometry::{ClientPoint, Pixels};
use crate::dioxus_elements::input_data::MouseButton;
use dioxus::html::geometry::euclid::{Rect, Size2D, Vector2D};
use dioxus::html::geometry::PixelsSize;
use dioxus::prelude::*;
use palette::{encoding, FromColor, Hsv, IntoColor, Oklch, RgbHue, Srgb};

use std::rc::Rc;

/// Represents an sRGB color.
pub type Color = Srgb<u8>;

const COLOR_AREA_MIN: f64 = 0.0;
const COLOR_AREA_MAX: f64 = 100.0;
const COLOR_AREA_RANGE: f64 = COLOR_AREA_MAX - COLOR_AREA_MIN;

/// Lightness threshold between orange and brown.
const ORANGE_LIGHTNESS_THRESHOLD: f64 = 0.68;

/// Lightness threshold between pure yellow and "yellow green".
const YELLOW_GREEN_LIGHTNESS_THRESHOLD: f64 = 0.85;

/// The maximum lightness considered to be "dark".
const MAX_DARK_LIGHTNESS: f64 = 0.55;

/// The chroma threshold between gray and color.
const GRAY_THRESHOLD: f64 = 0.001;

fn color_name(color: Color) -> String {
    let (l, c, h) = to_oklch(color);

    match l {
        ..0.001 => return String::from("black"),
        0.999.. => return String::from("white"),
        _ => {}
    }

    let (hue, l) = oklch_hue(l, c, h);

    let (lightness, chroma) = color_modifiers(l, c);

    let mut parts = Vec::new();
    if !lightness.is_empty() {
        parts.push(lightness);
    }
    if !chroma.is_empty() {
        parts.push(chroma);
    }
    if !hue.is_empty() {
        parts.push(&hue);
    }

    parts.join(" ")
}

fn color_modifiers(lightness: f64, chroma: f64) -> (&'static str, &'static str) {
    match (lightness, chroma) {
        (..0.3, GRAY_THRESHOLD..=0.1) => ("very dark", "grayish"),
        (..0.3, 0.15..) => ("very dark", "vibrant"),
        (..0.3, _) => ("very dark", ""),

        (0.3..MAX_DARK_LIGHTNESS, GRAY_THRESHOLD..=0.1) => ("dark", "grayish"),
        (0.3..MAX_DARK_LIGHTNESS, 0.15..) => ("dark", "vibrant"),
        (0.3..MAX_DARK_LIGHTNESS, _) => ("dark", ""),

        (MAX_DARK_LIGHTNESS..0.7, GRAY_THRESHOLD..=0.1) => ("", "grayish"),
        (MAX_DARK_LIGHTNESS..0.7, 0.15..) => ("", "vibrant"),
        (MAX_DARK_LIGHTNESS..0.7, _) => ("", ""),

        (0.7..0.85, GRAY_THRESHOLD..=0.1) => ("light", "pale"),
        (0.7..0.85, 0.15..) => ("light", "vibrant"),
        (0.7..0.85, _) => ("light", ""),

        (0.85.., GRAY_THRESHOLD..=0.1) => ("very light", "pale"),
        (0.85.., 0.15..) => ("very light", "vibrant"),
        (0.85.., _) => ("very light", ""),

        (_, GRAY_THRESHOLD..=0.1) => ("very light", "grayish"),
        (_, 0.15..) => ("very light", "vibrant"),
        _ => ("very light", ""),
    }
}

fn color_hex(color: Color) -> String {
    format!("#{color:X}")
}

/// Converts the RGB color to a (L, C, h) tuple.
fn to_oklch(color: Color) -> (f64, f64, f64) {
    let oklch: Oklch<f64> = color.into_format::<f64>().into_color();
    let (l, c, h) = oklch.into_components();
    (l, c, h.into_degrees())
}

fn oklch_hue(lightness: f64, chroma: f64, hue: f64) -> (String, f64) {
    if let ..GRAY_THRESHOLD = chroma {
        return ("gray".to_string(), lightness);
    }

    let hue = hue.rem_euclid(360.0);

    match (hue, lightness) {
        (0.0..=7.5, _) | (349.0..360.0, _) => ("pink".to_string(), lightness),
        (7.5..15.0, _) => ("pink red".to_string(), lightness),
        (15.0..=31.5, _) => ("red".to_string(), lightness),
        (31.5..48.0, _) => ("red orange".to_string(), lightness),
        (48.0..=71.0, ..ORANGE_LIGHTNESS_THRESHOLD) => ("brown".to_string(), lightness),
        (71.0..94.0, ..ORANGE_LIGHTNESS_THRESHOLD) => ("brown yellow".to_string(), lightness),
        (48.0..=71.0, _) => (
            "orange".to_string(),
            (lightness - ORANGE_LIGHTNESS_THRESHOLD) + MAX_DARK_LIGHTNESS,
        ),
        (71.0..94.0, _) => (
            "orange yellow".to_string(),
            (lightness - ORANGE_LIGHTNESS_THRESHOLD) + MAX_DARK_LIGHTNESS,
        ),
        (94.0..135.0, ..YELLOW_GREEN_LIGHTNESS_THRESHOLD) => {
            ("yellow green".to_string(), lightness)
        }
        (94.0..=114.5, _) => ("yellow".to_string(), lightness),
        (114.5..135.0, _) => ("yellow green".to_string(), lightness),
        (135.0..=155.0, _) => ("green".to_string(), lightness),
        (155.0..175.0, _) => ("green cyan".to_string(), lightness),
        (175.0..=219.5, _) => ("cyan".to_string(), lightness),
        (219.5..264.0, _) => ("cyan blue".to_string(), lightness),
        (264.0..=274.0, _) => ("blue".to_string(), lightness),
        (274.0..284.0, _) => ("blue purple".to_string(), lightness),
        (284.0..=302.0, _) => ("purple".to_string(), lightness),
        (302.0..320.0, _) => ("purple magenta".to_string(), lightness),
        (320.0..=334.5, _) => ("magenta".to_string(), lightness),
        (334.5..349.0, _) => ("magenta pink".to_string(), lightness),
        _ => unreachable!("Unexpected hue"),
    }
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
        self.on_color_change
            .call(Hsv::<encoding::Srgb, f64>::new(RgbHue::new(h), current.saturation, current.value));
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

#[derive(Debug)]
struct Pointer {
    id: i32,
    position: ClientPoint,
}

static POINTERS: GlobalSignal<Vec<Pointer>> = Global::new(|| {
    let runtime = Runtime::current();
    queue_effect(move || {
        runtime.spawn(ScopeId::ROOT, async move {
            let mut pointer_updates = dioxus::document::eval(
                "window.addEventListener('pointerdown', (e) => {
                    dioxus.send(['down', [e.pointerId, e.clientX, e.clientY]]);
                });
                window.addEventListener('pointermove', (e) => {
                    dioxus.send(['move', [e.pointerId, e.clientX, e.clientY]]);
                });
                window.addEventListener('pointerup', (e) => {
                    dioxus.send(['up', [e.pointerId, e.clientX, e.clientY]]);
                });
                window.addEventListener('pointercancel', (e) => {
                    dioxus.send(['up', [e.pointerId, e.clientX, e.clientY]]);
                });",
            );

            while let Ok((event_type, (pointer_id, x, y))) =
                pointer_updates.recv::<(String, (i32, f64, f64))>().await
            {
                let position = ClientPoint::new(x, y);

                match event_type.as_str() {
                    "down" => {
                        // Add a new pointer
                        POINTERS.write().push(Pointer {
                            id: pointer_id,
                            position,
                        });
                    }
                    "move" => {
                        // Update the position of an existing pointer
                        if let Some(pointer) =
                            POINTERS.write().iter_mut().find(|p| p.id == pointer_id)
                        {
                            pointer.position = position;
                        }
                    }
                    "up" => {
                        // Remove the pointer
                        POINTERS.write().retain(|p| p.id != pointer_id);
                    }
                    _ => {}
                }
            }
        });
    });

    Vec::new()
});

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
        let pointers = POINTERS.read();

        if !dragging() {
            return;
        }

        let Some(size) = size else {
            return;
        };

        let Some(active_pointer_id) = current_pointer_id() else {
            return;
        };

        let Some(pointer) = pointers.iter().find(|p| p.id == active_pointer_id) else {
            current_pointer_id.take();
            last_processed_pos.set(None);
            dragging.set(false);
            return;
        };

        let delta = if let Some(last_pos) = last_processed_pos.replace(Some(pointer.position)) {
            pointer.position - last_pos
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
                POINTERS.write().push(Pointer {
                    id: e.data().pointer_id(),
                    position: e.client_coordinates(),
                });

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
        for rgb in [Color::new(255, 0, 0), Color::new(0, 255, 0), Color::new(0, 0, 255)] {
            let hsv: Hsv<encoding::Srgb, f64> = rgb.into_format::<f64>().into_color();
            assert_eq!(hsv_to_rgb(hsv), rgb);
        }
    }
}
