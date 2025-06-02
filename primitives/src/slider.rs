use crate::use_controlled;
use dioxus::html::geometry::euclid::Vector2D;
use dioxus::html::geometry::{ClientPoint, ClientSpace, ElementPoint};
use dioxus_lib::html::geometry::Pixels;
use dioxus_lib::html::geometry::euclid::Rect;
use dioxus_lib::prelude::*;
use std::cell::RefCell;
use std::ops::RangeInclusive;
use std::rc::Rc;

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

static MOUSE_DOWN: Global<ReadOnlySignal<bool>> = Global::new(|| {
    let mut signal = Signal::new_in_scope(false, ScopeId::ROOT);
    let runtime = Runtime::current().unwrap();
    queue_effect(move || {
        runtime.spawn(ScopeId::ROOT, async move {
            let mut mouse_updates = dioxus::document::eval(
                "window.addEventListener('mousedown', (e) => {
                    dioxus.send(true);
                });",
            );
            while let Ok(_) = mouse_updates.recv::<bool>().await {
                signal.set(true);
            }
        });
        runtime.spawn(ScopeId::ROOT, async move {
            let mut mouse_updates = dioxus::document::eval(
                "window.addEventListener('mouseup', (e) => {
                    dioxus.send(false);
                });",
            );
            while let Ok(_) = mouse_updates.recv::<bool>().await {
                signal.set(false);
            }
        });
    });
    signal.into()
});

static MOUSE_POSITION: Global<ReadOnlySignal<Option<ClientPoint>>> = Global::new(|| {
    let mut signal = Signal::new_in_scope(None, ScopeId::ROOT);
    let runtime = Runtime::current().unwrap();
    queue_effect(move || {
        runtime.spawn(ScopeId::ROOT, async move {
            let mut mouse_updates = dioxus::document::eval(
                "window.addEventListener('mousemove', (e) => {
                    dioxus.send([e.clientX, e.clientY]);
                });",
            );
            while let Ok([x, y]) = mouse_updates.recv::<[f64; 2]>().await {
                signal.set(Some(ClientPoint::new(x, y)));
            }
        });
    });
    signal.into()
});

static MOUSE_DELTA: GlobalMemo<Vector2D<f64, ClientSpace>> = Global::new(|| {
    static LAST_MOUSE_POSITION: Global<Rc<RefCell<Option<ClientPoint>>>> =
        Global::new(|| Rc::new(RefCell::new(None)));
    let last_position_resolved = LAST_MOUSE_POSITION.resolve();
    let mut last_position_mut = last_position_resolved.borrow_mut();

    let Some(new_position) = MOUSE_POSITION.resolve().cloned() else {
        return Default::default();
    };
    let last_position = last_position_mut.clone();
    *last_position_mut = Some(new_position.clone());
    let Some(last_position) = last_position else {
        return Default::default();
    };
    new_position - last_position.cast_unit()
});

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

    let mut dragging = use_signal(|| false);

    let ctx = use_context_provider(|| SliderContext {
        value,
        set_value,
        min: props.min,
        max: props.max,
        step: props.step,
        disabled: props.disabled,
        horizontal: props.horizontal,
        inverted: props.inverted,
        dragging: dragging.into(),
    });

    let mut rect = use_signal(|| None);
    let mut div_element = use_signal(|| None);
    let mut granular_value = use_hook(|| CopyValue::new(props.default_value.clone()));

    let size = rect().map(|r: Rect<f64, Pixels>| {
        if props.horizontal {
            r.width()
        } else {
            r.height()
        }
    });

    use_effect(move || {
        if !dragging() {
            return;
        }

        let Some(size) = size else {
            tracing::warn!("Slider size is not (yet) set");
            return;
        };
        let delta = MOUSE_DELTA();

        let delta_pos = if ctx.horizontal { delta.x } else { delta.y } as f64;

        let delta = delta_pos / size as f64 * ctx.range_size();

        let current_value = match granular_value.cloned() {
            SliderValue::Single(v) => v,
            SliderValue::Range(start, _) => {
                // TODO: Handle range sliders
                start
            }
        };
        let new = current_value + delta;
        granular_value.set(SliderValue::Single(new));
        let clamped = new.clamp(ctx.min, ctx.max);
        let stepped = (clamped / ctx.step).round() * ctx.step;
        ctx.set_value.call(SliderValue::Single(stepped));
    });

    use_effect(move || {
        let mouse_down = MOUSE_DOWN.resolve().cloned();
        if !*dragging.peek() {
            return;
        }

        if !mouse_down {
            dragging.set(false);
        }
    });

    rsx! {
        div {
            role: "group",
            "data-disabled": props.disabled,
            "data-orientation": orientation,

            onmounted: move |evt| async move {
                // Get the bounding rect of the slider
                if let Ok(r) = evt.data().get_client_rect().await {
                    rect.set(Some(r));
                }
                div_element.set(Some(evt.data()));
            },
            onresize: move |_| async move {
                // Update the rect on resize
                let Some(div_element) = div_element() else {
                    tracing::warn!("Slider div element is not (yet) set");
                    return;
                };
                if let Ok(r) = div_element.get_client_rect().await {
                    rect.set(Some(r));
                }
            },

            onmousedown: move |evt| async move {
                if (ctx.disabled)() {
                    return;
                }
                let Some(div_element) = div_element() else {
                    tracing::warn!("Slider div element is not (yet) set");
                    return;
                };

                // Update the bounding rect of the slider in case it moved
                if let Ok(r) = div_element.get_client_rect().await {
                    rect.set(Some(r));

                    let size = if props.horizontal {
                        r.width()
                    } else {
                        r.height()
                    };

                    // Get the mouse position relative to the slider
                    let top_left = r.origin;
                    let mouse_pos = evt.data().client_coordinates();
                    let relative_pos = mouse_pos - top_left.cast_unit();

                    let offset = if ctx.horizontal {
                        relative_pos.x
                    } else {
                        relative_pos.y
                    };
                    let new = (offset / size) * ctx.range_size() + ctx.min;
                    granular_value.set(SliderValue::Single(new));
                    let stepped = (new / ctx.step).round() * ctx.step;
                    ctx.set_value.call(SliderValue::Single(stepped));
                }

                dragging.set(true);
            },

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
    let mut ctx = use_context::<SliderContext>();
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

    let mut button_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    use_effect(move || {
        let button_ref = button_ref();
        if let Some(button) = button_ref {
            // Focus the button while dragging
            if !(ctx.disabled)() && (ctx.dragging)() {
                button.set_focus(true);
            }
        }
    });

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
            onkeydown: move |evt| async move {
                if (ctx.disabled)() {
                    return;
                }

                let key = evt.data().key();
                let mut step = ctx.step;
                if evt.data().modifiers().shift() {
                    // If shift is pressed, increase the step size
                    step *= 10.0;
                }

                // Handle keyboard navigation
                let mut new_value = match key {
                    Key::ArrowUp | Key::ArrowRight => {
                        value() + step
                    }
                    Key::ArrowDown | Key::ArrowLeft => {
                        value() - step
                    }
                    _ => return,
                };

                // Clamp the new value to the range
                new_value = new_value.clamp(ctx.min, ctx.max);
                let stepped_value = (new_value / ctx.step).round() * ctx.step;

                // Update the value
                ctx.set_value.call(SliderValue::Single(stepped_value));
            },
            ..props.attributes,
        }
    }
}

/// Scale the input value which must be somewhere within the range into
/// a number between 0 and 1
fn normalize(range: [f64; 2]) -> impl Fn(f64) -> f64 {
    let [in_min, in_max] = range;

    move |x: f64| {
        // Calculate position in input range (0.0 ~ 1.0)
        (x - in_min) / (in_max - in_min)
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
struct SliderContext {
    value: Memo<SliderValue>,
    set_value: Callback<SliderValue>,
    min: f64,
    max: f64,
    step: f64,
    disabled: ReadOnlySignal<bool>,
    horizontal: bool,
    inverted: bool,
    dragging: ReadOnlySignal<bool>,
}

impl SliderContext {
    fn range(&self) -> [f64; 2] {
        if !self.inverted {
            [self.min, self.max]
        } else {
            [self.max, self.min]
        }
    }

    fn range_size(&self) -> f64 {
        let [range_min, range_max] = self.range();
        range_max - range_min
    }

    fn range_inclusive(&self) -> RangeInclusive<f64> {
        self.min..=self.max
    }
}
