//! Defines the [`DatePicker`] component and its subcomponents, which allowing users to enter or select a date value

use crate::{
    dioxus_elements::input_data::MouseButton,
    popover::{PopoverContent, PopoverRoot, PopoverTrigger},
    ContentAlign,
};

use dioxus::prelude::*;
use time::{macros::format_description, Date};

/// The value of the [`DatePicker`] component.
#[derive(Copy, Clone)]
pub struct DatePickerValue {
    /// A dates range value or single day
    is_range: bool,
    /// Current date value
    value: DateValue,
}

impl DatePickerValue {
    /// Create a single day value
    pub fn new_day(date: Option<Date>) -> Self {
        let is_range = false;

        match date {
            Some(date) => Self {
                is_range,
                value: DateValue::Single { date },
            },
            None => Self::new_empty(is_range),
        }
    }

    /// Create new date range value
    pub fn new_range(date: Option<Date>) -> Self {
        let is_range = true;

        match date {
            Some(date) => Self {
                is_range,
                value: DateValue::Range {
                    start: date,
                    end: None,
                },
            },
            None => Self::new_empty(is_range),
        }
    }

    /// Create full date range value
    pub fn range(start: Date, end: Date) -> Self {
        let value = if end < start {
            DateValue::Range {
                start: end,
                end: Some(start),
            }
        } else {
            DateValue::Range {
                start,
                end: Some(end),
            }
        };
        Self {
            is_range: true,
            value,
        }
    }

    fn new(is_range: bool, date: Option<Date>) -> Self {
        if is_range {
            Self::new_range(date)
        } else {
            Self::new_day(date)
        }
    }

    fn new_empty(is_range: bool) -> Self {
        Self {
            is_range,
            value: DateValue::Empty,
        }
    }

    fn part_count(&self) -> usize {
        if self.is_range {
            2
        } else {
            1
        }
    }

    fn set_date(&self, date: Option<Date>) -> Self {
        match self.value {
            DateValue::Range { start, end } => {
                if end.is_some() {
                    Self::new_range(date)
                } else {
                    match date {
                        Some(end) => Self::range(start, end),
                        None => *self,
                    }
                }
            }
            _ => Self::new(self.is_range, date),
        }
    }

    /// Return current selected date
    pub fn date(&self) -> Option<Date> {
        match self.value {
            DateValue::Single { date } => Some(date),
            DateValue::Range { start, end } => {
                if end.is_some() {
                    return end;
                }

                Some(start)
            }
            DateValue::Empty => None,
        }
    }

    // Returns `true` if the given date is selected
    fn is_date_selected(&self, date: Option<Date>) -> bool {
        self.date() == date
    }

    fn ready_to_close(&self) -> bool {
        match self.value {
            DateValue::Range { end, .. } => end.is_some(),
            _ => true,
        }
    }
}

impl std::fmt::Display for DatePickerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// The value type of the [`DatePicker`] component.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DateValue {
    /// A single value for the date picker
    Single {
        /// The selected date
        date: Date,
    },
    /// A dates range value for the date picker
    Range {
        /// The first range date
        start: Date,
        /// The last range date
        end: Option<Date>,
    },
    /// None value for the date picker
    Empty,
}

impl std::fmt::Display for DateValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DateValue::Single { date } => write!(f, "{date}"),
            DateValue::Range { start, end } => {
                let end_str = match end {
                    Some(date) => date.to_string(),
                    None => String::default(),
                };
                write!(f, "{start} - {end_str}")
            }
            DateValue::Empty => write!(f, ""),
        }
    }
}

/// The context provided by the [`DatePicker`] component to its children.
#[derive(Copy, Clone)]
struct DatePickerContext {
    // State
    value: ReadOnlySignal<DatePickerValue>,
    on_value_change: Callback<DatePickerValue>,
    selected_date: ReadOnlySignal<Option<Date>>,
    open: Signal<bool>,
    read_only: ReadOnlySignal<bool>,

    // Configuration
    disabled: ReadOnlySignal<bool>,
    separator: &'static str,
    format_placeholder: Callback<(), String>,
}

impl DatePickerContext {
    fn set_date(&mut self, date: Option<Date>) {
        let value = (self.value)();
        if value.is_date_selected(date) {
            return;
        }

        let value = value.set_date(date);
        self.on_value_change.call(value);

        if value.ready_to_close() {
            self.open.set(false);
        }
    }
}

/// The props for the [`DatePicker`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerProps {
    /// The controlled value of the date picker
    pub value: ReadOnlySignal<DatePickerValue>,

    /// Callback when value changes
    #[props(default)]
    pub on_value_change: Callback<DatePickerValue>,

    /// The selected date
    #[props(default)]
    pub selected_date: ReadOnlySignal<Option<Date>>,

    /// Whether the date picker is disabled
    #[props(default)]
    pub disabled: ReadOnlySignal<bool>,

    /// Whether the date picker is enable user input
    #[props(default = ReadOnlySignal::new(Signal::new(false)))]
    pub read_only: ReadOnlySignal<bool>,

    /// Separator between range value
    #[props(default = " - ")]
    pub separator: &'static str,

    /// Callback when display placeholder
    #[props(default = Callback::new(|_| "YYYY-MM-DD".to_string()))]
    pub on_format_placeholder: Callback<(), String>,

    /// Additional attributes to extend the date picker element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the date picker element
    pub children: Element,
}

/// # DatePicker
///
/// The [`DatePicker`] component provides an accessible date input interface.
///
/// ## Example
/// ```rust
/// ```
///
/// # Styling
///
/// The [`DatePicker`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the DatePicker is disabled. Possible values are `true` or `false`.
#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    let open = use_signal(|| false);

    // Create context provider for child components
    use_context_provider(|| DatePickerContext {
        open,
        value: props.value,
        on_value_change: props.on_value_change,
        selected_date: props.selected_date,
        disabled: props.disabled,
        read_only: props.read_only,
        separator: props.separator,
        format_placeholder: props.on_format_placeholder,
    });

    rsx! {
        div {
            role: "application",
            "aria-label": "DatePicker",
            "data-disabled": (props.disabled)(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`SelectDateTrigger`] component
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerTriggerProps {
    /// Additional attributes for the trigger button
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the trigger
    pub children: Element,
}

/// # DatePickerTrigger
///
/// The `PopoverTrigger` is a button that toggles the visibility of the [`PopoverContent`].
///
/// ```rust
/// ```
#[component]
pub fn DatePickerTrigger(props: DatePickerTriggerProps) -> Element {
    let mut ctx = use_context::<DatePickerContext>();
    let mut open = ctx.open;

    use_effect(move || {
        let date = (ctx.selected_date)();
        ctx.set_date(date);
    });

    rsx! {
        PopoverRoot {
            class: "popover",
            open: open(),
            on_open_change: move |v| open.set(v),
            PopoverTrigger { attributes: props.attributes,
                svg {
                    class: "date-picker-expand-icon",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    polyline { points: "6 9 12 15 18 9" }
                }
            }
            PopoverContent {
                gap: "0.25rem",
                align: ContentAlign::End,
                {props.children}
            }
        }
    }
}

/// The props for the [`DatePickerInput`] component
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerInputProps {
    /// Additional attributes for the value element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the date picker element
    pub children: Element,
}

/// # DatePickerInput
///
/// The input element for the [`DatePicker`](super::date_picker::DatePicker) component which allow users to enter a date value.
///
/// ```rust
/// ```
#[component]
pub fn DatePickerInput(props: DatePickerInputProps) -> Element {
    let mut ctx = use_context::<DatePickerContext>();

    let display_value = use_memo(move || ctx.value.to_string());

    let placeholder = {
        let capacity = (ctx.value)().part_count();
        let text = ctx.format_placeholder.call(());
        vec![text; capacity].join(ctx.separator)
    };

    let handle_input = move |e: Event<FormData>| {
        let text = e.value().parse().unwrap_or(display_value());

        let value = (ctx.value)();
        let format = format_description!("[year]-[month]-[day]");

        if value.is_range {
        } else {
            if text.is_empty() {
                ctx.set_date(None);
                return;
            }

            let date = Date::parse(&text, &format).ok();
            if date.is_some_and(|_| !value.is_date_selected(date)) {
                ctx.set_date(date);
            }
        }
    };

    rsx! {
        input {
            style: "min-width: 240px",
            placeholder,
            value: display_value,
            disabled: ctx.disabled,
            readonly: ctx.read_only,
            cursor: if (ctx.read_only)() { "pointer" } else { "text" },
            oninput: handle_input,
            onpointerdown: move |event| {
                if (ctx.read_only)() && event.trigger_button() == Some(MouseButton::Primary) {
                    ctx.open.toggle();
                }
            },
            ..props.attributes,
        }
        {props.children}
    }
}
