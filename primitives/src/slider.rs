use crate::use_controlled;
use dioxus_lib::prelude::*;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq)]
pub enum SliderValue {
    Single(f64),
    Range(f64, f64),
}

impl std::fmt::Display for SliderValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SliderValue::Single(v) => write!(f, "{}", v),
            SliderValue::Range(start, end) => write!(f, "{}, {}", start, end),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SliderProps {
    /// The controlled value of the slider
    value: Option<Signal<SliderValue>>,

    /// The default value when uncontrolled
    #[props(default = SliderValue::Single(0.0))]
    default_value: SliderValue,

    /// The minimum value
    #[props(default = 0.0)]
    min: f64,

    /// The maximum value
    #[props(default = 100.0)]
    max: f64,

    /// The step value
    #[props(default = 1.0)]
    step: f64,

    /// Whether the slider is disabled
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    /// Orientation of the slider
    #[props(default = true)]
    horizontal: bool,

    /// Inverts the order of the values
    #[props(default)]
    inverted: bool,

    /// Callback when value changes
    #[props(default)]
    on_value_change: Callback<SliderValue>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Slider(props: SliderProps) -> Element {
    let (value, set_value) = use_controlled(
        props.value,
        props.default_value.clone(),
        props.on_value_change,
    );

    let orientation = if props.horizontal {
        "horizontal"
    } else {
        "vertical"
    };
    let range = props.min..=props.max;

    let _ctx = use_context_provider(|| SliderContext {
        value,
        set_value,
        min: props.min,
        max: props.max,
        step: props.step,
        disabled: props.disabled,
        horizontal: props.horizontal,
        inverted: props.inverted,
        range,
    });

    rsx! {
        div {
            role: "group",
            "data-disabled": props.disabled,
            "data-orientation": orientation,
            ..props.attributes,

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SliderTrackProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn SliderTrack(props: SliderTrackProps) -> Element {
    let ctx = use_context::<SliderContext>();
    let orientation = if ctx.horizontal {
        "horizontal"
    } else {
        "vertical"
    };

    rsx! {
        div {
            "data-disabled": ctx.disabled,
            "data-orientation": orientation,
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SliderRangeProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn SliderRange(props: SliderRangeProps) -> Element {
    let ctx = use_context::<SliderContext>();
    let orientation = if ctx.horizontal {
        "horizontal"
    } else {
        "vertical"
    };

    let style = use_memo(move || {
        let (start, end) = match (ctx.value)() {
            SliderValue::Single(v) => (ctx.min, v),
            SliderValue::Range(start, end) => (start, end),
        };

        let start_percent = ((start - ctx.min) / (ctx.max - ctx.min) * 100.0).clamp(0.0, 100.0);
        let end_percent = ((end - ctx.min) / (ctx.max - ctx.min) * 100.0).clamp(0.0, 100.0);

        if ctx.horizontal {
            format!("left: {}%; right: {}%", start_percent, 100.0 - end_percent)
        } else {
            format!("bottom: {}%; top: {}%", start_percent, 100.0 - end_percent)
        }
    });

    rsx! {
        div {
            "data-disabled": ctx.disabled,
            "data-orientation": orientation,
            style,
            ..props.attributes,
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SliderThumbProps {
    /// Which thumb this is in a range slider
    #[props(default)]
    index: Option<usize>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn SliderThumb(props: SliderThumbProps) -> Element {
    let ctx = use_context::<SliderContext>();
    let orientation = if ctx.horizontal {
        "horizontal"
    } else {
        "vertical"
    };

    let value = use_memo(move || match ((ctx.value)(), props.index) {
        (SliderValue::Single(v), _) => v,
        (SliderValue::Range(start, _), Some(0)) => start,
        (SliderValue::Range(_, end), Some(1)) => end,
        _ => ctx.min,
    });

    let percent = ((value() - ctx.min) / (ctx.max - ctx.min) * 100.0).clamp(0.0, 100.0);
    let style = if ctx.horizontal {
        format!("left: {}%", percent)
    } else {
        format!("bottom: {}%", percent)
    };

    let mut dragging = use_signal(|| false);
    let mut start_pos = use_signal(|| 0.0);
    let mut start_value = use_signal(|| 0.0);

    rsx! {
        button {
            r#type: "button",
            role: "slider",
            aria_valuemin: ctx.min,
            aria_valuemax: ctx.max,
            aria_valuenow: value,
            aria_orientation: orientation,
            "data-disabled": ctx.disabled,
            "data-orientation": orientation,
            "data-dragging": dragging,
            style,
            tabindex: 0,

            onmousedown: move |e| {
                if (ctx.disabled)() {
                    return;
                }
                dragging.set(true);
                start_pos
                    .set(
                        if ctx.horizontal {
                            e.data().client_coordinates().x
                        } else {
                            e.data().client_coordinates().y
                        } as f64,
                    );
                start_value.set(value());
            },

            onmousemove: move |e| {
                if !dragging() || (ctx.disabled)() {
                    return;
                }
                let current_pos = if ctx.horizontal {
                    e.data().client_coordinates().x
                } else {
                    e.data().client_coordinates().y
                } as f64;
                let delta = current_pos - start_pos();
                let range = ctx.max - ctx.min;
                let value_delta = if ctx.horizontal {
                    delta / 200.0 * range
                } else {
                    -delta / 200.0 * range
                };
                let new_value = (start_value() + value_delta).clamp(ctx.min, ctx.max);
                let stepped = (new_value / ctx.step).round() * ctx.step;
                match ((ctx.value)(), props.index) {
                    (SliderValue::Single(_), _) => {
                        ctx.set_value.call(SliderValue::Single(stepped));
                    }
                    (SliderValue::Range(_start, end), Some(0)) => {
                        ctx.set_value.call(SliderValue::Range(stepped.min(end), end));
                    }
                    (SliderValue::Range(start, _end), Some(1)) => {
                        ctx.set_value.call(SliderValue::Range(start, stepped.max(start)));
                    }
                    _ => {}
                }
            },

            onmouseup: move |_| dragging.set(false),
            onmouseleave: move |_| dragging.set(false),

            ..props.attributes,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
struct SliderContext {
    value: Memo<SliderValue>,
    set_value: Callback<SliderValue>,
    min: f64,
    max: f64,
    step: f64,
    disabled: ReadOnlySignal<bool>,
    horizontal: bool,
    inverted: bool,
    range: RangeInclusive<f64>,
}
