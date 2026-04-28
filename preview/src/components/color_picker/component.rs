use dioxus::prelude::*;
use dioxus_primitives::color_picker::{
    self, Color, ColorAreaProps, ColorPickerContext, ColorPickerProps,
};
use dioxus_primitives::label::Label;
use dioxus_primitives::slider::*;

use crate::components::dialog::*;
use crate::components::input::Input;
use crate::dioxus_elements::geometry::ClientPoint;

use std::str::FromStr;

#[component]
pub fn ColorPicker(props: ColorPickerProps) -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        color_picker::ColorPicker {
            class: "dx-color-picker",
            color: props.color,
            on_color_change: props.on_color_change,
            disabled: props.disabled,
            attributes: props.attributes,
            button {
                class: "dx-color-picker-button",
                disabled: if (props.disabled)() { true },
                onclick: move |_| open.set(true),
                ColorSwatch { color: props.color }
                if let Some(label) = props.label { span { {label} } }
            }
            DialogRoot {
                open: open(),
                on_open_change: move |v| open.set(v),
                DialogContent {
                    width: "auto",
                    {props.children}
                }
            }
        }
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
    pub on_color_change: Callback<Option<Color>>,

    /// Optional label above the input field
    #[props(default)]
    pub label: Option<String>,

    /// Optional props for the text description element
    #[props(default)]
    pub description: Option<String>,

    /// Additional attributes to extend the color field element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color field element
    pub children: Element,
}

/// # ColorField
///
/// The [`ColorField`] allows users to edit a hex color.
#[component]
fn ColorField(props: ColorFieldProps) -> Element {
    let mut value = use_signal(|| (props.color)().map(|c| c.to_hex()).unwrap_or_default());

    // Synchronize local text with external color changes
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

    // Helper to increment or decrement the color value.
    // It treats the RGB color as a u32 integer for simple channel-based stepping.
    let offset_color = move |v: f64| {
        let delta = v.signum();
        if delta == 0.0 || delta.is_nan() {
            return;
        }

        if let Ok(color) = value().parse::<Color>() {
            let val = u32::from(color);
            // Saturating add/sub prevents overflow/underflow at #FFFFFF or #000000.
            let new_color = if delta < 0.0 {
                val.saturating_sub(1)
            } else {
                val.saturating_add(1)
            };

            props.on_color_change.call(Some(Color::from(new_color)));
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
        div {
            class: "dx-color-field-container",
            ..props.attributes,
            if let Some(label) = props.label {
                Label {
                    html_for: "color_field",
                    class: "dx-color-slider-title",
                    {label}
                }
            }
            Input {
                id: "color_field",
                placeholder: "Enter a color",
                value: "{value}",
                oninput: move |e: FormEvent| {
                    let mut input = e.value();

                    // Sanitize input: allow only '#' and hex digits, length limit.
                    input.retain(|c| c == '#' || c.is_ascii_hexdigit());

                    // Automatically prepend '#' if missing.
                    if !input.starts_with('#') && !input.is_empty() {
                        input.insert(0, '#');
                    }

                    input.truncate(7);
                    value.set(input.to_uppercase());

                    if let Ok(parsed) = Color::from_str(&input) {
                        props.on_color_change.call(Some(parsed));
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
                span { class: "dx-color-field-description", {text} }
            }
            {props.children}
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
#[allow(dead_code)]
pub enum SwatchSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl SwatchSize {
    fn to_class(self) -> &'static str {
        match self {
            SwatchSize::Small => "dx-swatch-sm",
            SwatchSize::Medium => "dx-swatch-md",
            SwatchSize::Large => "dx-swatch-lg",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
#[allow(dead_code)]
pub enum SwatchShape {
    Circle,
    #[default]
    Rounded,
}

impl SwatchShape {
    fn to_class(self) -> &'static str {
        match self {
            SwatchShape::Circle => "dx-swatch-circle",
            SwatchShape::Rounded => "dx-swatch-rounded",
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

/// # ColorSwatch
///
/// The [`ColorSwatch`] displays a preview of a selected color.
#[component]
fn ColorSwatch(props: ColorSwatchProps) -> Element {
    let hex_color = use_memo(move || (props.color)().to_hex());

    rsx! {
        div {
            role: "img",
            aria_label: (props.color)().to_css_rgb(),
            class: "dx-color-swatch {props.size.to_class()} {props.shape.to_class()}",
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

/// # ColorSlider
///
/// The [`ColorSlider`] allows users to adjust a hue of a color value.
#[component]
fn ColorSlider(props: ColorSliderProps) -> Element {
    let mut current_hue = use_signal(|| (props.value)());

    let mut thumb_color = use_signal(|| Color::from_hsv(current_hue(), 1.0, 1.0));

    use_effect(move || {
        let value = (props.value)();
        let current = current_hue();

        let is_wrap_around = (value - current).abs() > 350.0;

        // Update the signal only if this is an actual new position,
        // and not a "flip" of the circle by the palette library.
        if !is_wrap_around && value != current {
            current_hue.set(value);
            thumb_color.set(Color::from_hsv(value, 1.0, 1.0));
        }
    });

    let display_value = {
        let value = current_hue();
        format!("{value:.2}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
            + "°"
    };

    rsx! {

        div {
            class: "dx-color-slider-container",
            ..props.attributes,
            label { class: "dx-color-slider-title", {props.title} }
            output { class: "dx-color-slider-output", "{display_value}" }
            Slider {
                class: "dx-color-slider",
                label: "Color Slider",
                horizontal: true,
                max: 360.0,
                value: SliderValue::Single(current_hue()),
                on_value_change: move |value: SliderValue| {
                    let SliderValue::Single(h) = value;

                    // Allow the value to be exactly 360.0
                    // The palette will understand that 360.0 == 0.0, but the signal will remain 360.0 for the UI.
                    current_hue.set(h);
                    thumb_color.set(Color::from_hsv(h, 1.0, 1.0));

                    props.on_value_change.call(h);
                },
                SliderTrack {
                    class: "dx-color-slider-track",
                    SliderThumb {
                        class: "dx-color-slider-thumb",
                        background_color: thumb_color().to_css_rgb(),
                    }
                }
            }
            {props.children}
        }
    }
}

#[component]
fn ColorArea(props: ColorAreaProps) -> Element {
    rsx! {
        color_picker::ColorArea {
            class: "dx-color-area-container",
            color: props.color,
            min: props.min,
            max: props.max,
            step: props.step,
            on_value_change: props.on_value_change,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`ColorPickerSelect`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ColorPickerSelectProps {
    /// Additional attributes to extend the color picker select element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color picker select element
    pub children: Element,
}

#[component]
pub fn ColorPickerSelect(props: ColorPickerSelectProps) -> Element {
    let ctx = use_context::<ColorPickerContext>();

    // Keep local HSV state to avoid slider/area "jumps" caused by RGB<->HSV round trips
    let (_h, _s, _v) = (ctx.color)().to_hsv();
    let mut hue = use_signal(|| _h);
    let mut sat = use_signal(|| _s);
    let mut val = use_signal(|| _v);

    let current_color = use_memo(move || Color::from_hsv(hue(), sat(), val()));

    // Synchronization on external change
    use_effect(move || {
        let external_color = (ctx.color)();
        if external_color != current_color() {
            let (h, s, v) = external_color.to_hsv();
            hue.set(h);
            sat.set(s);
            val.set(v);
        }
    });

    let update_color = Callback::new(move |color: Color| {
        if color == current_color() {
            return;
        }

        let (h, s, v) = color.to_hsv();
        hue.set(h);
        sat.set(s);
        val.set(v);
        ctx.on_color_change.call(color);
    });

    rsx! {
        div {
            class: "dx-color-picker-dialog",
            ..props.attributes,
            ColorArea {
                color: current_color(),
                on_value_change: move |value: ClientPoint| {
                    sat.set(value.x);
                    val.set(value.y);
                    ctx.on_color_change
                        .call(Color::from_hsv(hue(), value.x, value.y));
                },
            }
            ColorSlider {
                title: "Hue",
                value: hue,
                on_value_change: move |h: f64| {
                    hue.set(h);
                    ctx.on_color_change.call(Color::from_hsv(h, sat(), val()));
                },
            }
            div {
                class: "dx-color-picker-input",
                ColorField {
                    label: "Hex",
                    color: current_color(),
                    on_color_change: move |c: Option<Color>| {
                        if let Some(color) = c {
                            update_color.call(color);
                        }
                    }
                }
                ColorSwatch { color: current_color }
            }
        }
    }
}
