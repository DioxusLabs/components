//! Defines the [`ColorPicker`] component and its sub-components.

use crate::dioxus_core::{queue_effect, Runtime};
use crate::dioxus_elements::geometry::Pixels;
use crate::dioxus_elements::{geometry::ClientPoint, input_data::MouseButton};
use dioxus::html::geometry::euclid::Rect;
use dioxus::html::geometry::euclid::Vector2D;
use dioxus::prelude::*;
use palette::{encoding, Hsv, IntoColor, RgbHue, Srgb};

use std::{rc::Rc, str::FromStr};

/// Represents RGB color
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Color(pub Srgb<u8>);

impl Color {
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

    /// Creates a Color instance from HSV components
    pub fn from_hsv(h: f64, s: f64, v: f64) -> Self {
        let hsv = Hsv::new(RgbHue::new(h), s.clamp(0.0, 1.0), v.clamp(0.0, 1.0));
        // Convert HSV back to f64 RGB and then to u8 RGB
        let rgb: Srgb<f64> = hsv.into_color();
        Self(rgb.into_format())
    }

    /// Extracts the Hue component from the current color.
    pub fn hue(&self) -> f64 {
        self.to_hsv().0
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
}

impl FromStr for Color {
    type Err = ();

    /// Parses a HEX string into a Color.
    /// Supports shorthand (e.g., "#ABC" -> "#AABBCC") and standard formats (e.g., "#FF0000").
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('#');
        let radix = 16;
        match s.len() {
            // Shorthand format: "ABC"
            3 => {
                let r = u8::from_str_radix(&s[0..1], radix).map_err(|_| ())?;
                let g = u8::from_str_radix(&s[1..2], radix).map_err(|_| ())?;
                let b = u8::from_str_radix(&s[2..3], 16).map_err(|_| ())?;
                // Expand 4-bit to 8-bit (e.g., 0xf -> 0xff)
                Ok(Color::new(r << 4 | r, g << 4 | g, b << 4 | b))
            }
            // Standard format: "AABBCC"
            6 => {
                let r = u8::from_str_radix(&s[0..2], 16).map_err(|_| ())?;
                let g = u8::from_str_radix(&s[2..4], 16).map_err(|_| ())?;
                let b = u8::from_str_radix(&s[4..6], 16).map_err(|_| ())?;
                Ok(Color::new(r, g, b))
            }
            _ => Err(()),
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

    /// Optional label on the button
    #[props(default)]
    pub label: Option<String>,

    /// Additional attributes to extend the color picker element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color picker element
    pub children: Element,
}

#[component]
pub fn ColorPicker(props: ColorPickerProps) -> Element {
    use_context_provider(|| ColorPickerContext {
        color: props.color,
        on_color_change: props.on_color_change,
    });

    rsx! {
        div {
            role: "group",
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

#[derive(Copy, Clone, PartialEq)]
struct SV {
    pub s: f64,
    pub v: f64,
}

impl SV {
    pub fn new(s: f64, v: f64) -> Self {
        Self { s, v }
    }

    pub fn shift(&self, d_s: f64, d_v: f64) -> Self {
        let s = self.s + d_s;
        let v = self.v + d_v;
        SV::new(s, v)
    }

    pub fn snap(&self, step: f64) -> Self {
        let s = (self.s / step).round() * step;
        let v = (self.v / step).round() * step;
        SV::new(s, v)
    }

    pub fn clamp(self, min: f64, max: f64) -> Self {
        Self {
            s: self.s.clamp(min, max),
            v: self.v.clamp(min, max),
        }
    }
}

/// The props for the [`ColorArea`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ColorAreaProps {
    /// The controlled value of the slider
    pub color: ReadSignal<Color>,

    /// The minimum value
    #[props(default = 0.0)]
    pub min: ReadSignal<f64>,

    /// The maximum value
    #[props(default = 255.0)]
    pub max: ReadSignal<f64>,

    /// The step value
    #[props(default = 1.0)]
    pub step: ReadSignal<f64>,

    /// Callback when value changes
    #[props(default)]
    pub on_color_change: Callback<Color>,

    /// Additional attributes to extend the color area element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color area element
    pub children: Element,
}

#[component]
pub fn ColorArea(props: ColorAreaProps) -> Element {
    let mut dragging = use_signal(|| false);

    let value = use_memo(move || {
        let current_color = (props.color)().to_hsv();
        SV::new(current_color.1 * 255.0, current_color.2 * 255.0)
    });
    let update_sv = Callback::new(move |sv: SV| {
        let new_color = (props.color)().with_sv(sv.s / 255.0, sv.v / 255.0);
        props.on_color_change.call(new_color);
    });

    let ctx = use_context_provider(|| ColorAreaContext {
        value,
        set_value: update_sv,
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

        let new_value = granular_value().shift(d_s, -d_h);
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

                        let s = (relative_pos.x / size.width) * range + min;
                        let h = (props.max)() - ((relative_pos.y / size.height) * range + min);
                        let sv = SV::new(s, h);
                        granular_value.set(sv);
                        ctx.set_value.call(ctx.snap(sv));
                    }

                    dragging.set(true);
                });
            },
            ..props.attributes,
            AreaTrack {
                style: format!("--area-color: {}", Color::from_hsv((props.color)().hue(), 1.0, 1.0).to_css_rgb()),
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
            class: "color-area-track",
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
    let style = format!("left: {:.2}%; top: {:.2}%;", percent.0, 100. - percent.1);

    rsx! {
        button {
            class: "color-area-thumb",
            type: "button",
            role: "region",
            aria_valuemin: format!("({} {})", ctx.min, ctx.min),
            aria_valuemax: format!("({} {})", ctx.max, ctx.max),
            aria_valuetext: format!("({:.2} {:.2})", (ctx.value)().s, (ctx.value)().v),
            aria_label: "area-thumb",
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
            {props.children}
        }
    }
}

#[derive(Copy, Clone)]
struct ColorAreaContext {
    value: Memo<SV>,
    set_value: Callback<SV>,
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

    fn snap(&self, value: SV) -> SV {
        let step = (self.step)();
        value.snap(step)
    }

    fn clamp_and_snap(&self, value: SV) -> SV {
        let clamped = value.clamp((self.min)(), (self.max)());
        self.snap(clamped)
    }

    fn as_percent(&self, value: SV) -> (f64, f64) {
        let min = (self.min)();
        let size = self.range_size();
        (
            ((value.s - min) / size * 100.0).clamp(0.0, 100.0),
            ((value.v - min) / size * 100.0).clamp(0.0, 100.0),
        )
    }
}
