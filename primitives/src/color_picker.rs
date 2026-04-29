//! Defines the [`ColorPicker`] component and its sub-components.

use crate::dioxus_core::{queue_effect, Runtime};
use crate::dioxus_elements::geometry::{ClientPoint, Pixels};
use crate::dioxus_elements::input_data::MouseButton;
use dioxus::html::geometry::euclid::{Rect, Size2D, Vector2D};
use dioxus::html::geometry::PixelsSize;
use dioxus::prelude::*;
use palette::{encoding, Hsv, IntoColor, Oklch, RgbHue, Srgb};

use std::{rc::Rc, str::FromStr};

/// Represents RGB color
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Color(pub Srgb<u8>);

/// HSV components with explicit achromatic handling.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct HsvColor {
    /// Hue in degrees [0-360). `None` means hue is undefined for achromatic colors.
    pub hue: Option<f64>,
    /// Saturation in [0-1].
    pub saturation: f64,
    /// Value in [0-1].
    pub value: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Error returned when parsing a [`Color`] from a string.
pub enum ParseColorError {
    /// The input doesn't match a supported HEX length (`#RGB` or `#RRGGBB`).
    InvalidLength,
    /// The input contains non-hexadecimal digits.
    InvalidHex,
}

impl Color {
    /// Small epsilon used to classify achromatic colors in sRGB (1 channel step).
    const ACHROMATIC_EPSILON: f64 = 1.0 / 255.0;

    /// Lightness threshold between orange and brown.
    const ORANGE_LIGHTNESS_THRESHOLD: f64 = 0.68;

    /// Lightness threshold between pure yellow and "yellow green".
    const YELLOW_GREEN_LIGHTNESS_THRESHOLD: f64 = 0.85;

    /// The maximum lightness considered to be "dark".
    const MAX_DARK_LIGHTNESS: f64 = 0.55;

    /// The chroma threshold between gray and color.
    const GRAY_THRESHOLD: f64 = 0.001;

    const OKLCH_HUES: [(f64, &str); 10] = [
        (0.0, "pink"),
        (15.0, "red"),
        (48.0, "orange"),
        (94.0, "yellow"),
        (135.0, "green"),
        (175.0, "cyan"),
        (264.0, "blue"),
        (284.0, "purple"),
        (320.0, "magenta"),
        (349.0, "pink"),
    ];

    /// Create an RGB color.
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self(Srgb::new(red, green, blue))
    }

    /// Generates a random RGB color using the 'rand' crate.
    pub fn random_rgb() -> Self {
        let [r, g, b] = rand::random::<[u8; 3]>();
        Self::new(r, g, b)
    }

    /// Converts the color to a CSS-compatible HEX string (e.g., "#FF00AA").
    pub fn to_hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0.red, self.0.green, self.0.blue)
    }

    /// Converts the color to a CSS-compatible RGB string (e.g., "rgb(255 0 170)").
    pub fn to_css_rgb(&self) -> String {
        format!("rgb({} {} {})", self.0.red, self.0.green, self.0.blue)
    }

    /// Converts the RGB color to HSV (Hue, Saturation, Value).
    /// Returns a tuple: (Hue in degrees [0-360], Saturation [0-1], Value [0-1]).
    pub fn to_hsv(self) -> (f64, f64, f64) {
        // Convert u8 [0-255] to f64 [0.0-1.0] and then to HSV
        let hsv: Hsv<encoding::Srgb, f64> = self.0.into_format::<f64>().into_color();
        (hsv.hue.into_positive_degrees(), hsv.saturation, hsv.value)
    }

    /// Converts the RGB color to a (L, C, h) tuple.
    fn to_oklch(self) -> (f64, f64, f64) {
        // Convert u8 [0-255] to f64 [0.0-1.0] and then to Oklch
        let oklch: Oklch<f64> = self.0.into_format::<f64>().into_color();
        let (l, c, h) = oklch.into_components();
        (l, c, h.into_degrees())
    }

    /// Converts RGB to HSV and marks hue as undefined for achromatic colors.
    pub fn to_hsv_achromatic(self) -> HsvColor {
        let (h, s, v) = self.to_hsv();
        let hue = if s <= Self::ACHROMATIC_EPSILON || v <= Self::ACHROMATIC_EPSILON {
            None
        } else {
            Some(h)
        };
        HsvColor {
            hue,
            saturation: s,
            value: v,
        }
    }

    /// Creates a Color instance from HSV components
    pub fn from_hsv(h: f64, s: f64, v: f64) -> Self {
        let hue = h.rem_euclid(360.0);
        let hsv = Hsv::new(RgbHue::new(hue), s.clamp(0.0, 1.0), v.clamp(0.0, 1.0));
        // Convert HSV back to f64 RGB and then to u8 RGB
        let rgb: Srgb<f64> = hsv.into_color();
        Self(rgb.into_format())
    }

    /// Extracts the Hue component from the current color.
    pub fn hue(&self) -> f64 {
        self.to_hsv().0
    }

    /// Returns hue if chromatic, otherwise provided fallback hue.
    pub fn hue_or(&self, fallback: f64) -> f64 {
        self.to_hsv_achromatic().hue.unwrap_or(fallback)
    }

    /// Update only Hue while keeping current Saturation and Value.
    pub fn with_hue(self, h: f64) -> Self {
        let (_, s, v) = self.to_hsv();
        Self::from_hsv(h, s, v)
    }

    /// Update Saturation and Value while keeping the current Hue.
    pub fn with_sv(self, s: f64, v: f64) -> Self {
        let h = self.hue();
        Self::from_hsv(h, s, v)
    }

    /// Returns name for the color, for use in visual or accessibility labels.
    pub fn color_name(&self) -> String {
        let (l, c, h) = self.to_oklch();

        if l > 0.999 {
            return String::from("white");
        }

        if l < 0.001 {
            return String::from("black");
        }

        let (hue, l) = Self::oklch_hue(l, c, h);

        let lightness = if l < 0.3 {
            "very dark"
        } else if l < Self::MAX_DARK_LIGHTNESS {
            "dark"
        } else if l < 0.7 {
            // none
            ""
        } else if l < 0.85 {
            "light"
        } else {
            "very light"
        };

        let chroma = if (Self::GRAY_THRESHOLD..=0.1).contains(&c) {
            if l >= 0.7 {
                "pale"
            } else {
                "grayish"
            }
        } else if c >= 0.15 {
            "vibrant"
        } else {
            ""
        };

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

    /// Returns name for the hue, for use in visual or accessibility labels.
    pub fn hue_name(&self) -> String {
        let (l, c, h) = self.to_oklch();
        let (hue, _) = Self::oklch_hue(l, c, h);
        hue
    }

    fn oklch_hue(l: f64, c: f64, h: f64) -> (String, f64) {
        if c < Self::GRAY_THRESHOLD {
            return ("gray".to_string(), l);
        }

        let h = h.rem_euclid(360.0);

        for (index, &(hue, hue_name)) in Self::OKLCH_HUES.iter().enumerate() {
            let mut new_l = l;
            let mut new_hue_name = hue_name.to_string();

            let (next_hue, next_hue_name) = if index + 1 < Self::OKLCH_HUES.len() {
                Self::OKLCH_HUES[index + 1]
            } else {
                (360.0, "pink")
            };

            if h >= hue && h < next_hue {
                // Split orange hue into brown/orange depending on lightness.
                if hue_name == "orange" {
                    if l < Self::ORANGE_LIGHTNESS_THRESHOLD {
                        new_hue_name = "brown".to_string();
                    } else {
                        // Adjust lightness.
                        new_l = (l - Self::ORANGE_LIGHTNESS_THRESHOLD) + Self::MAX_DARK_LIGHTNESS;
                    }
                }

                // If the hue is at least halfway to the next hue, add the next hue name as well.
                if h > hue + (next_hue - hue) / 2.0 && new_hue_name != next_hue_name {
                    new_hue_name = format!("{new_hue_name} {next_hue_name}");
                } else if new_hue_name == "yellow" && new_l < Self::YELLOW_GREEN_LIGHTNESS_THRESHOLD
                {
                    // Yellow shifts toward green at lower lightnesses.
                    new_hue_name = "yellow green".to_string();
                }

                return (new_hue_name, new_l);
            }
        }

        unreachable!("Unexpected hue")
    }
}

impl FromStr for Color {
    type Err = ParseColorError;

    /// Parses a HEX string into a Color.
    /// Supports shorthand (e.g., "#ABC" -> "#AABBCC") and standard formats (e.g., "#FF0000").
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('#');
        let radix = 16;
        match s.len() {
            // Shorthand format: "ABC"
            3 => {
                let r =
                    u8::from_str_radix(&s[0..1], radix).map_err(|_| ParseColorError::InvalidHex)?;
                let g =
                    u8::from_str_radix(&s[1..2], radix).map_err(|_| ParseColorError::InvalidHex)?;
                let b =
                    u8::from_str_radix(&s[2..3], radix).map_err(|_| ParseColorError::InvalidHex)?;
                // Expand 4-bit to 8-bit (e.g., 0xf -> 0xff)
                Ok(Color::new(r << 4 | r, g << 4 | g, b << 4 | b))
            }
            // Standard format: "AABBCC"
            6 => {
                let r =
                    u8::from_str_radix(&s[0..2], radix).map_err(|_| ParseColorError::InvalidHex)?;
                let g =
                    u8::from_str_radix(&s[2..4], radix).map_err(|_| ParseColorError::InvalidHex)?;
                let b =
                    u8::from_str_radix(&s[4..6], radix).map_err(|_| ParseColorError::InvalidHex)?;
                Ok(Color::new(r, g, b))
            }
            _ => Err(ParseColorError::InvalidLength),
        }
    }
}

impl From<Color> for u32 {
    /// Packs the RGB color into a single u32 integer (Big Endian: 0x00RRGGBB).
    fn from(c: Color) -> u32 {
        u32::from_be_bytes([0, c.0.red, c.0.green, c.0.blue])
    }
}

impl From<u32> for Color {
    /// Unpacks a u32 integer (0x...RRGGBB) into a Color instance.
    /// discarding any possibly significant alpha value.
    fn from(v: u32) -> Self {
        let [_, r, g, b] = v.to_be_bytes();
        Self::new(r, g, b)
    }
}

/// Context provided by [`ColorPicker`] to its descendants.
#[derive(Clone, Copy)]
pub struct ColorPickerContext {
    /// The current selected color
    pub color: ReadSignal<Color>,
    /// Callback when color changes
    pub on_color_change: Callback<Color>,
}

/// The props for the [`ColorPicker`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ColorPickerProps {
    /// The selected color
    #[props(default)]
    pub color: ReadSignal<Color>,

    /// Callback when color changes
    #[props(default)]
    pub on_color_change: Callback<Color>,

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
///    let rgb = Color::random_rgb();
///    let mut color = use_signal(|| rgb);
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
                    dioxus.send(['down', [e.pointerId, e.pageX, e.pageY]]);
                });
                window.addEventListener('pointermove', (e) => {
                    dioxus.send(['move', [e.pointerId, e.pageX, e.pageY]]);
                });
                window.addEventListener('pointerup', (e) => {
                    dioxus.send(['up', [e.pointerId, e.pageX, e.pageY]]);
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
    /// The controlled value of the slider
    pub color: ReadSignal<Color>,

    /// The minimum value
    #[props(default = 0.0)]
    pub min: ReadSignal<f64>,

    /// The maximum value
    #[props(default = 100.0)]
    pub max: ReadSignal<f64>,

    /// The step value
    #[props(default = 1.0)]
    pub step: ReadSignal<f64>,

    /// Callback when value changes
    #[props(default)]
    pub on_value_change: Callback<ClientPoint>,

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
///    let rgb = Color::random_rgb();
///    let mut color = use_signal(|| rgb);
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
    let mut dragging = use_signal(|| false);
    let mut stable_hue = use_signal(|| (props.color)().hue());

    let initial_value = {
        let hsv = (props.color)().to_hsv();
        ClientPoint::new(hsv.1, hsv.2) * (props.max)()
    };
    let mut value = use_memo(move || initial_value);

    // Keep local pointer position stable during drag.
    use_effect(move || {
        if dragging() {
            return;
        }

        let hsv = (props.color)().to_hsv();
        let next = ClientPoint::new(hsv.1, hsv.2) * (props.max)();
        if next != value() {
            value.set(next);
        }
    });

    // Preserve the last chromatic hue while current color is achromatic.
    use_effect(move || {
        if let Some(h) = (props.color)().to_hsv_achromatic().hue {
            if h != stable_hue() {
                stable_hue.set(h);
            }
        }
    });

    let update_xy = Callback::new(move |point: ClientPoint| {
        value.set(point);
        let new_value = point / (props.max)();
        props.on_value_change.call(new_value);
    });

    let ctx = use_context_provider(|| ColorAreaContext {
        value,
        set_value: update_xy,
        min: props.min,
        max: props.max,
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
            return;
        };

        let delta = if let Some(last_pos) = last_processed_pos.replace(Some(pointer.position)) {
            pointer.position - last_pos
        } else {
            Vector2D::zero()
        };

        let min = (props.min)();
        let range = ctx.range_size();
        let d_s = delta.x / size.width * range + min;
        let d_h = delta.y / size.height * range + min;

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

                        let min = (props.min)();
                        let range = ctx.range_size();

                        let x = (relative_pos.x / size.width) * range + min;
                        let y = (props.max)() - ((relative_pos.y / size.height) * range + min);
                        let pt = ClientPoint::new(x, y);
                        granular_value.set(pt);
                        ctx.set_value.call(ctx.snap(pt));
                    }

                    dragging.set(true);
                });
            },
            ..props.attributes,
            AreaTrack {
                style: format!(
                    "--area-color: {}",
                    Color::from_hsv((props.color)().hue_or(stable_hue()), 1.0, 1.0).to_css_rgb()
                ),
                AreaThumb {
                    background_color: (props.color)().to_css_rgb(),
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
    let min = (ctx.min)();
    let max = (ctx.max)();
    let step = (ctx.step)();

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
            ..props.attributes,
            input {
                class: "dx-color-area-input",
                r#type: "range",
                aria_label: "Saturation",
                aria_roledescription: "2D Slider",
                aria_valuetext: format!("Saturation {:.0}%, {}", (current.x / max * 100.0).clamp(0.0, 100.0), (picker_ctx.color)().color_name()),
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
                aria_valuetext: format!("Value {:.0}%, {}", (current.y / max * 100.0).clamp(0.0, 100.0), (picker_ctx.color)().color_name()),
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
    min: ReadSignal<f64>,
    max: ReadSignal<f64>,
    step: ReadSignal<f64>,
    dragging: ReadSignal<bool>,
}

impl ColorAreaContext {
    fn range(&self) -> [f64; 2] {
        [(self.min)(), (self.max)()]
    }

    fn range_size(&self) -> f64 {
        let [range_min, range_max] = self.range();
        range_max - range_min
    }

    fn snap(&self, value: ClientPoint) -> ClientPoint {
        let step = (self.step)();
        value.map(|v| (v / step).round() * step)
    }

    fn clamp_and_snap(&self, value: ClientPoint) -> ClientPoint {
        let clamped = value.map(|v| v.clamp((self.min)(), (self.max)()));
        self.snap(clamped)
    }

    fn as_percent(&self, value: ClientPoint) -> PixelsSize {
        let min = (self.min)();
        let size = self.range_size();
        let scaled = value.map(|v| ((v - min) / size * 100.0).clamp(0.0, 100.0));
        PixelsSize::new(scaled.x, scaled.y)
    }
}
