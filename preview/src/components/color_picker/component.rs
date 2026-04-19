use dioxus::prelude::*;
use dioxus_primitives::label::Label;
use dioxus_primitives::slider::*;

use crate::components::dialog::*;
use crate::components::input::Input;
use palette::{Hsv, IntoColor, RgbHue, Srgb};

use std::str::FromStr;

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

    /// Converts the RGB color to HSV (Hue, Saturation, Value).
    /// Returns a tuple: (Hue in degrees [0-360], Saturation [0-1], Value [0-1]).
    pub fn to_hsv(self) -> (f64, f64, f64) {
        // Convert u8 [0-255] to f64 [0.0-1.0] and then to HSV
        let hsv: Hsv<palette::encoding::Srgb, f64> = self.0.into_format::<f64>().into_color();
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

    /// Creates a fully saturated and bright color based only on a Hue angle.
    pub fn from_hue(angle: f64) -> Self {
        Self::from_hsv(angle, 1.0, 0.5)
    }
}

impl FromStr for Color {
    type Err = ();

    /// Parses a HEX string into a Color.
    /// Supports shorthand (e.g., "#abc" -> "#aabbcc") and standard formats (e.g., "#ff0000").
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('#');
        match s.len() {
            // Shorthand format: "abc"
            3 => {
                let r = u8::from_str_radix(&s[0..1], 16).map_err(|_| ())?;
                let g = u8::from_str_radix(&s[1..2], 16).map_err(|_| ())?;
                let b = u8::from_str_radix(&s[2..3], 16).map_err(|_| ())?;
                // Expand 4-bit to 8-bit (e.g., 0xf -> 0xff)
                Ok(Color::new(r << 4 | r, g << 4 | g, b << 4 | b))
            }
            // Standard format: "aabbcc"
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
    fn from(v: u32) -> Self {
        let [_, r, g, b] = v.to_be_bytes();
        Self::new(r, g, b)
    }
}

/// The props for the [`ColorField`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ColorFieldProps {
    /// The selected color
    #[props(default)]
    pub color: ReadSignal<Option<Color>>,

    /// Callback when color changes
    #[props(default)]
    pub on_value_change: Callback<Option<Color>>,

    #[props(default)]
    pub label: Option<String>,

    #[props(default)]
    pub description: Option<String>,

    /// Additional attributes to extend the color field element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color field element
    pub children: Element,
}

#[component]
pub fn ColorField(props: ColorFieldProps) -> Element {
    let mut value = use_signal(|| (props.color)().map(|c| c.to_hex()).unwrap_or_default());

    use_effect(move || {
        if let Some(external_color) = (props.color)() {
            if let Ok(color) = Color::from_str(&value()) {
                if color != external_color {
                    value.set(external_color.to_hex());
                }
            }

            if value.is_empty() {
                value.set(external_color.to_hex());
            }
        }
    });

    let offset_color = move |v: f64| {
        let delta = v.signum();
        if delta == 0.0 || delta.is_nan() {
            return;
        }

        if let Ok(color) = value().parse::<Color>() {
            let val = u32::from(color);
            let new_color = if delta < 0.0 {
                val.saturating_sub(1)
            } else {
                val.saturating_add(1)
            };

            props.on_value_change.call(Some(Color::from(new_color)));
        }
    };

    let onkeydown = move |event: Event<KeyboardData>| {
        match event.key() {
            Key::ArrowDown => offset_color(-1.0),
            Key::ArrowUp => offset_color(1.0),
            _ => return,
        }
        event.prevent_default();
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div {
            class: "field-container",
            ..props.attributes,
            if let Some(label) = props.label {
                Label {
                    html_for: "color_field",
                    class: "color-slider-title",
                    {label}
                }
            }
            Input {
                id: "color_field",
                placeholder: "Enter a color",
                value: "{value}",
                oninput: move |e: FormEvent| {
                    let mut input = e.value();

                    input.retain(|c| c == '#' || c.is_ascii_hexdigit());

                    if !input.starts_with('#') && !input.is_empty() {
                        input.insert(0, '#');
                    }

                    input.truncate(7);
                    value.set(input.to_uppercase());

                    if let Ok(parsed) = Color::from_str(&input) {
                        props.on_value_change.call(Some(parsed));
                    }
                },
                onwheel: move |e: WheelEvent| {
                    e.prevent_default();
                    let delta_y = e.data.delta().strip_units().y;
                    offset_color(-delta_y);
                },
                onkeydown
            }
            if let Some(text) = props.description {
                span { class: "description", {text} }
            }
            {props.children}
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum SwatchSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl SwatchSize {
    fn to_class(self) -> &'static str {
        match self {
            SwatchSize::Small => "swatch-sm",
            SwatchSize::Medium => "swatch-md",
            SwatchSize::Large => "swatch-lg",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum SwatchShape {
    Circle,
    #[default]
    Rounded,
}

impl SwatchShape {
    fn to_class(self) -> &'static str {
        match self {
            SwatchShape::Circle => "swatch-circle",
            SwatchShape::Rounded => "swatch-rounded",
        }
    }
}

/// The props for the [`ColorSwatch`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ColorSwatchProps {
    /// The selected color
    #[props(default)]
    pub color: ReadSignal<Color>,

    #[props(default)]
    pub size: SwatchSize,

    #[props(default)]
    pub shape: SwatchShape,

    /// Additional attributes to extend the color swatch element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color swatch element
    pub children: Element,
}

#[component]
pub fn ColorSwatch(props: ColorSwatchProps) -> Element {
    let hex_color = use_memo(move || (props.color)().to_hex());

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div {
            class: "color-swatch {props.size.to_class()} {props.shape.to_class()}",
            style: "--swatch-color: {hex_color}",
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`ColorSlider`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ColorSliderProps {
    /// The controlled value of the slider
    pub value: ReadSignal<f64>,

    /// Callback when value changes
    #[props(default)]
    pub on_value_change: Callback<f64>,

    pub title: ReadSignal<String>,

    /// Additional attributes to extend the color slider element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color slider element
    pub children: Element,
}

#[component]
pub fn ColorSlider(props: ColorSliderProps) -> Element {
    let mut current_value = use_signal(|| (props.value)());

    use_effect(move || {
        let value = (props.value)();
        let current = current_value();

        let is_wrap_around = (value - current).abs() > 350.0;

        // Update the signal only if this is an actual new position,
        // and not a "flip" of the circle by the palette library.
        if !is_wrap_around && value != current {
            current_value.set(value);
        }
    });

    let display_value = {
        let value = current_value();
        format!("{:.2}", value)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
            + "°"
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div {
            class: "color-slider-container",
            ..props.attributes,
            label { class: "color-slider-title", {props.title} }
            output { class: "color-slider-output", "{display_value}" }
            Slider {
                class: "color-slider",
                label: "Color Slider",
                horizontal: true,
                min: 0.0,
                max: 360.0,
                step: 1.0,
                value: SliderValue::Single(current_value()),
                on_value_change: move |value: SliderValue| {
                    let SliderValue::Single(v) = value;
                    current_value.set(v);
                    props.on_value_change.call(v);
                },
                SliderTrack {
                    class: "color-slider-track",
                    SliderThumb {
                        class: "color-slider-thumb",
                        style: "--hue-value: {current_value}",
                    }
                }
            }
            {props.children}
        }
    }
}

/// The props for the [`ColorArea`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ColorAreaProps {
    /// The selected color
    #[props(default)]
    pub hue: ReadSignal<f64>,

    /// Additional attributes to extend the color area element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color area element
    pub children: Element,
}

#[component]
pub fn ColorArea(props: ColorAreaProps) -> Element {
    rsx! {
        div {
            class: "color-area-container",
            background: format!("linear-gradient(to top, black, transparent), linear-gradient(to right, white, transparent), hsl({}, 100%, 50%)", (props.hue)()),
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`ColorPickerProps`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ColorPickerProps {
    /// The selected color
    #[props(default)]
    pub color: ReadSignal<Color>,

    /// Callback when color changes
    #[props(default)]
    pub on_value_change: Callback<Color>,

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
    let mut current_color = use_signal(|| (props.color)());
    let mut hue = use_signal(|| (props.color)().hue());

    let mut open = use_signal(|| false);

    // Synchronization on external change
    use_effect(move || {
        let external_color = (props.color)();
        if external_color == current_color() {
            return;
        }

        let h = external_color.hue();
        if (hue() - h).abs() > 0.01 {
            hue.set(h);
        }

        current_color.set(external_color);
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div {
            role: "group",
            ..props.attributes,
            button {
                class: "color-picker-button",
                onclick: move |_| open.set(true),
                ColorSwatch { color: current_color }
                if let Some(label) = props.label { span { {label} } }
                {props.children}
            }
            DialogRoot {
                open: open(),
                on_open_change: move |v| open.set(v),
                DialogContent {
                    width: "auto",
                    div {
                        class: "color-picker-dialog",
                        ColorArea { hue: RgbHue::new(hue()) }
                        ColorSlider {
                            title: "Hue",
                            value: hue(),
                            on_value_change: move |angle| {
                                hue.set(angle);

                                let color = Color::from_hue(angle);
                                current_color.set(color);
                                props.on_value_change.call(color);
                            },
                        }
                        div {
                            class: "color-picker-input",
                            ColorField {
                                label: "Hex",
                                color: current_color(),
                                on_value_change: move |c: Option<Color>| {
                                    if let Some(color) = c {
                                        hue.set(color.hue());
                                        current_color.set(color);
                                        props.on_value_change.call(color);
                                    }
                                }
                            }
                            ColorSwatch { color: current_color }
                        }
                    }
                }
            }
        }
    }
}
