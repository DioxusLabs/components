//! Defines the [`DatePicker`] component and its subcomponents, which allowing users to enter or select a date value

use crate::{
    calendar::{Calendar, CalendarProps},
    focus::{use_focus_controlled_item, use_focus_provider, FocusState},
    popover::*,
    use_unique_id,
};

use dioxus::prelude::*;
use num_integer::Integer;
use std::{fmt::Display, str::FromStr};
use time::{macros::date, Date, Month, UtcDateTime};

/// The value of the [`DatePicker`] component.
/// Currently this can only be a single date, but support for ranges is planned.
#[derive(Copy, Clone)]
pub struct DatePickerValue {
    /// Current date value
    value: DateValue,
}

impl DatePickerValue {
    /// Create a single day value
    pub fn new_day(date: Option<Date>) -> Self {
        match date {
            Some(date) => Self {
                value: DateValue::Single { date },
            },
            None => Self {
                value: DateValue::Empty,
            },
        }
    }

    /// Return current selected date
    pub fn date(&self) -> Option<Date> {
        match self.value {
            DateValue::Single { date } => Some(date),
            DateValue::Empty => None,
        }
    }

    // Returns `true` if the given date is selected
    fn is_date_selected(&self, date: Option<Date>) -> bool {
        self.date() == date
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
    /// None value for the date picker
    Empty,
}

impl std::fmt::Display for DateValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DateValue::Single { date } => write!(f, "{date}"),
            DateValue::Empty => write!(f, ""),
        }
    }
}

/// The context provided by the [`DatePicker`] component to its children.
#[derive(Copy, Clone)]
struct DatePickerContext {
    // State
    value: ReadSignal<DatePickerValue>,
    on_value_change: Callback<DatePickerValue>,
    selected_date: ReadSignal<Option<Date>>,
    open: Signal<bool>,
    read_only: ReadSignal<bool>,

    // Configuration
    disabled: ReadSignal<bool>,
    focus: FocusState,
    min_date: Date,
    max_date: Date,
}

impl DatePickerContext {
    fn set_date(&mut self, date: Option<Date>) {
        let value = (self.value)();
        if value.is_date_selected(date) {
            return;
        }

        let value = DatePickerValue::new_day(date);
        self.on_value_change.call(value);

        self.open.set(false);
    }
}

/// The props for the [`DatePicker`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerProps {
    /// The controlled value of the date picker
    pub value: ReadSignal<DatePickerValue>,

    /// Callback when value changes
    #[props(default)]
    pub on_value_change: Callback<DatePickerValue>,

    /// The selected date
    #[props(default)]
    pub selected_date: ReadSignal<Option<Date>>,

    /// Whether the date picker is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the date picker is enable user input
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub read_only: ReadSignal<bool>,

    /// Lower limit of the range of available dates
    #[props(default = date!(1925-01-01))]
    pub min_date: Date,

    /// Upper limit of the range of available dates
    #[props(default = date!(2050-12-31))]
    pub max_date: Date,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub roving_loop: ReadSignal<bool>,

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
/// use dioxus::prelude::*;
/// use dioxus_primitives::{date_picker::*, ContentAlign};
/// use time::Date;
/// #[component]
/// pub fn Demo() -> Element {
///    let v = DatePickerValue::new_day(None);
///    let mut value = use_signal(|| v);
///    let mut selected_date = use_signal(|| None::<Date>);
///    rsx! {
///        div {
///            DatePicker {
///                value: value(),
///                selected_date: selected_date(),
///                on_value_change: move |v| {
///                    tracing::info!("Date changed to: {v}");
///                    value.set(v);
///                    selected_date.set(v.date());
///               },
///                DatePickerInput {
///                    DatePickerPopover {
///                        DatePickerPopoverTrigger {}
///                        DatePickerPopoverContent {
///                            align: ContentAlign::End,
///                            DatePickerCalendar {
///                                selected_date: selected_date(),
///                                on_date_change: move |date| selected_date.set(date),
///                            }
///                        }
///                    }
///                }
///            }
///        }
///    }
///}
/// ```
///
/// # Styling
///
/// The [`DatePicker`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the DatePicker is disabled. Possible values are `true` or `false`.
#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    let open = use_signal(|| false);
    let focus = use_focus_provider(props.roving_loop);

    // Create context provider for child components
    use_context_provider(|| DatePickerContext {
        value: props.value,
        on_value_change: props.on_value_change,
        selected_date: props.selected_date,
        open,
        read_only: props.read_only,
        disabled: props.disabled,
        focus,
        min_date: props.min_date,
        max_date: props.max_date,
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

/// # DatePickerPopover
///
/// The `DatePickerPopover` component wraps all the popover components and manages the state.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{date_picker::*, ContentAlign};
/// use time::Date;
/// #[component]
/// pub fn Demo() -> Element {
///    let v = DatePickerValue::new_day(None);
///    let mut value = use_signal(|| v);
///    let mut selected_date = use_signal(|| None::<Date>);
///    rsx! {
///        div {
///            DatePicker {
///                value: value(),
///                selected_date: selected_date(),
///                on_value_change: move |v| {
///                    tracing::info!("Date changed to: {v}");
///                    value.set(v);
///                    selected_date.set(v.date());
///               },
///                DatePickerInput {
///                    DatePickerPopover {
///                        DatePickerPopoverTrigger {}
///                        DatePickerPopoverContent {
///                            align: ContentAlign::End,
///                            DatePickerCalendar {
///                                selected_date: selected_date(),
///                                on_date_change: move |date| selected_date.set(date),
///                            }
///                        }
///                    }
///                }
///            }
///        }
///    }
///}
/// ```
#[component]
pub fn DatePickerPopover(props: PopoverRootProps) -> Element {
    let ctx = use_context::<DatePickerContext>();
    let mut open = ctx.open;

    rsx! {
        PopoverRoot {
            open: open(),
            on_open_change: move |v| open.set(v),
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// # DatePickerPopoverTrigger
///
/// The `DatePickerPopoverTrigger` is a button that toggles the visibility of the [`DatePickerPopoverContent`].
///
/// This must be used inside a [`DatePickerPopover`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{date_picker::*, ContentAlign};
/// use time::Date;
/// #[component]
/// pub fn Demo() -> Element {
///    let v = DatePickerValue::new_day(None);
///    let mut value = use_signal(|| v);
///    let mut selected_date = use_signal(|| None::<Date>);
///    rsx! {
///        div {
///            DatePicker {
///                value: value(),
///                selected_date: selected_date(),
///                on_value_change: move |v| {
///                    tracing::info!("Date changed to: {v}");
///                    value.set(v);
///                    selected_date.set(v.date());
///               },
///                DatePickerInput {
///                    DatePickerPopover {
///                        DatePickerPopoverTrigger {}
///                        DatePickerPopoverContent {
///                            align: ContentAlign::End,
///                            DatePickerCalendar {
///                                selected_date: selected_date(),
///                                on_date_change: move |date| selected_date.set(date),
///                            }
///                        }
///                    }
///                }
///            }
///        }
///    }
///}
/// ```
#[component]
pub fn DatePickerPopoverTrigger(props: PopoverTriggerProps) -> Element {
    rsx! {
        PopoverTrigger {
            aria_label: "Show Calendar",
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// # DatePickerPopoverContent
///
/// The `DatePickerPopoverContent` component defines the content of the popover. This component will
/// only be rendered if the popover is open.
///
/// This must be used inside a [`DatePickerPopover`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{date_picker::*, ContentAlign};
/// use time::Date;
/// #[component]
/// pub fn Demo() -> Element {
///    let v = DatePickerValue::new_day(None);
///    let mut value = use_signal(|| v);
///    let mut selected_date = use_signal(|| None::<Date>);
///    rsx! {
///        div {
///            DatePicker {
///                value: value(),
///                selected_date: selected_date(),
///                on_value_change: move |v| {
///                    tracing::info!("Date changed to: {v}");
///                    value.set(v);
///                    selected_date.set(v.date());
///               },
///                DatePickerInput {
///                    DatePickerPopover {
///                        DatePickerPopoverTrigger {}
///                        DatePickerPopoverContent {
///                            align: ContentAlign::End,
///                            DatePickerCalendar {
///                                selected_date: selected_date(),
///                                on_date_change: move |date| selected_date.set(date),
///                            }
///                        }
///                    }
///                }
///            }
///        }
///    }
///}
/// ```
#[component]
pub fn DatePickerPopoverContent(props: PopoverContentProps) -> Element {
    rsx! {
        PopoverContent {
            id: props.id,
            side: props.side,
            align: props.align,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// # DatePickerCalendar
///
/// The [`DatePickerCalendar`] component provides an accessible calendar interface with arrow key navigation, month switching, and date selection.
/// Used as date picker popover component
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{date_picker::*, ContentAlign};
/// use time::Date;
/// #[component]
/// pub fn Demo() -> Element {
///    let v = DatePickerValue::new_day(None);
///    let mut value = use_signal(|| v);
///    let mut selected_date = use_signal(|| None::<Date>);
///    rsx! {
///        div {
///            DatePicker {
///                value: value(),
///                selected_date: selected_date(),
///                on_value_change: move |v| {
///                    tracing::info!("Date changed to: {v}");
///                    value.set(v);
///                    selected_date.set(v.date());
///               },
///                DatePickerInput {
///                    DatePickerPopover {
///                        DatePickerPopoverTrigger {}
///                        DatePickerPopoverContent {
///                            align: ContentAlign::End,
///                            DatePickerCalendar {
///                                selected_date: selected_date(),
///                                on_date_change: move |date| selected_date.set(date),
///                            }
///                        }
///                    }
///                }
///            }
///        }
///    }
///}
/// ```
#[component]
pub fn DatePickerCalendar(props: CalendarProps) -> Element {
    let ctx = use_context::<DatePickerContext>();
    let mut view_date = use_signal(|| UtcDateTime::now().date());

    use_effect(move || {
        if let Some(date) = (props.selected_date)() {
            view_date.set(date);
        }
    });

    rsx! {
        Calendar {
                selected_date: props.selected_date,
                on_date_change: props.on_date_change,
                on_format_weekday: props.on_format_weekday,
                on_format_month: props.on_format_month,
                view_date: view_date(),
                today: props.today,
                on_view_change: move |date| view_date.set(date),
                disabled: props.disabled,
                first_day_of_week: props.first_day_of_week,
                min_date: ctx.min_date,
                max_date: ctx.max_date,
                attributes: props.attributes,
                {props.children}
        }
    }
}

// The props for the [`DateSegment`] component
#[derive(Props, Clone, PartialEq)]
struct DateSegmentProps<T: Clone + Integer + 'static> {
    // The index of the segment
    pub index: ReadSignal<usize>,

    // The controlled value of the date picker
    pub value: ReadSignal<Option<T>>,

    // Default value
    pub default: T,

    // Callback when value changes
    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    // The minimum value
    pub min: T,

    // The maximum value
    pub max: T,

    // Max field length
    pub max_length: usize,

    // Callback when display placeholder
    pub on_format_placeholder: Callback<(), String>,

    // Additional attributes for the value element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
fn DateSegment<T: Clone + Copy + Integer + FromStr + Display + 'static>(
    props: DateSegmentProps<T>,
) -> Element {
    let mut text_value = use_signal(|| "".to_string());
    use_effect(move || {
        let text = match (props.value)() {
            Some(value) => value.to_string(),
            None => String::default(),
        };
        text_value.set(text);
    });

    let mut reset_value = use_signal(|| false);

    // The formatted text for the segment
    let display_value = use_memo(move || {
        let value = (props.value)();
        match value {
            Some(value) => format!("{:0>width$}", value, width = props.max_length),
            None => props
                .on_format_placeholder
                .call(())
                .repeat(props.max_length),
        }
    });

    let now_value = use_memo(move || (props.value)().unwrap_or(props.default));

    let mut ctx = use_context::<DatePickerContext>();

    let mut set_value = move |text: String| {
        if text.is_empty() {
            props.on_value_change.call(None);
            ctx.focus.focus_prev();
            return;
        }

        let value = text.parse::<T>().map(|v| v.min(props.max)).ok();
        if let Some(value) = value {
            let inRange = value >= props.min && value <= props.max;

            let newValue = (text + "0").parse::<T>().unwrap_or(value);
            if inRange && newValue > props.max {
                ctx.focus.focus_next();
                reset_value.set(true);
            }
        };

        props.on_value_change.call(value);
    };

    let clamp_value = move |value: T| {
        if value < props.min {
            props.max
        } else if value > props.max {
            props.min
        } else {
            value
        }
    };

    let handle_keydown = move |event: Event<KeyboardData>| {
        let key = event.key();
        match key {
            Key::Character(actual_char) => {
                if actual_char.parse::<T>().is_ok() {
                    let mut text = text_value();
                    if text.len() == props.max_length || reset_value() {
                        text = String::default();
                        reset_value.set(false);
                    };
                    text.push_str(&actual_char);
                    set_value(text);
                }

                event.prevent_default();
                event.stop_propagation();
            }
            Key::Backspace => {
                let mut text = text_value();
                text.pop();
                set_value(text);
            }
            Key::Delete => {
                let mut text = text_value();
                text.remove(0);
                set_value(text);
            }
            Key::ArrowLeft => {
                ctx.focus.focus_prev();
            }
            Key::ArrowRight => {
                ctx.focus.focus_next();
            }
            Key::ArrowUp => {
                let value = match (props.value)() {
                    Some(mut value) => {
                        value.inc();
                        clamp_value(value)
                    }
                    None => props.default,
                };
                props.on_value_change.call(Some(value));
            }
            Key::ArrowDown => {
                let value = match (props.value)() {
                    Some(mut value) => {
                        value.dec();
                        clamp_value(value)
                    }
                    None => props.default,
                };
                props.on_value_change.call(Some(value));
            }
            _ => (),
        }
    };

    let focused = move || ctx.focus.is_focused(props.index.cloned());
    let onmounted = use_focus_controlled_item(props.index);

    let span_id = use_unique_id();
    let id = use_memo(move || format!("span-{span_id}"));
    let label_id = format!("{id}-label");

    rsx! {
        span {
            class: "date-segment",
            id,
            role: "spinbutton",
            aria_valuemin: props.min.to_string(),
            aria_valuemax: props.max.to_string(),
            aria_valuenow: now_value.to_string(),
            aria_labelledby: "{label_id}",
            inputmode: "numeric",
            contenteditable: !(ctx.read_only)(),
            spellcheck: false,
            tabindex: if focused() { "0" } else { "-1" },
            enterkeyhint: "next",
            onkeydown: handle_keydown,
            onmounted,
            onfocus: move |_| {
                ctx.focus.set_focus(Some(props.index.cloned()));
                if (ctx.open)() {
                    ctx.open.set(false);
                }
            },
            "no-date": (props.value)().is_none(),
            "data-disabled": (ctx.disabled)(),
            ..props.attributes,
            {display_value}
        }
    }
}

#[component]
fn DateSeparator() -> Element {
    rsx! {
        span {
            class: "date-segment",
            aria_hidden: "true",
            tabindex: "-1",
            "is-separator": true,
            "no-date": true,
            {"-"}
        }
    }
}

/// The props for the [`DatePickerInput`] component
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerInputProps {
    /// Callback when display day placeholder
    #[props(default = Callback::new(|_| "D".to_string()))]
    pub on_format_day_placeholder: Callback<(), String>,

    /// Callback when display month placeholder
    #[props(default = Callback::new(|_| "M".to_string()))]
    pub on_format_month_placeholder: Callback<(), String>,

    /// Callback when display year placeholder
    #[props(default = Callback::new(|_| "Y".to_string()))]
    pub on_format_year_placeholder: Callback<(), String>,

    /// Additional attributes for the value element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the date picker element
    pub children: Element,
}

/// # DatePickerInput
///
/// The input element for the [`DatePicker`] component which allow users to enter a date value.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{date_picker::*, ContentAlign};
/// use time::Date;
/// #[component]
/// pub fn Demo() -> Element {
///    let v = DatePickerValue::new_day(None);
///    let mut value = use_signal(|| v);
///    let mut selected_date = use_signal(|| None::<Date>);
///    rsx! {
///        div {
///            DatePicker {
///                value: value(),
///                selected_date: selected_date(),
///                on_value_change: move |v| {
///                    tracing::info!("Date changed to: {v}");
///                    value.set(v);
///                    selected_date.set(v.date());
///               },
///                DatePickerInput {
///                    DatePickerPopover {
///                        DatePickerPopoverTrigger {}
///                        DatePickerPopoverContent {
///                            align: ContentAlign::End,
///                            DatePickerCalendar {
///                                selected_date: selected_date(),
///                                on_date_change: move |date| selected_date.set(date),
///                            }
///                        }
///                    }
///                }
///            }
///        }
///    }
///}
/// ```
#[component]
pub fn DatePickerInput(props: DatePickerInputProps) -> Element {
    let mut ctx = use_context::<DatePickerContext>();

    let mut day_value = use_signal(|| None);
    let mut month_value = use_signal(|| None);
    let mut year_value = use_signal(|| None);

    use_effect(move || {
        let date = (ctx.selected_date)();
        year_value.set(date.map(|d| d.year()));
        month_value.set(date.map(|d| d.month() as u8));
        day_value.set(date.map(|d| d.day()));
    });

    use_effect(move || {
        if let Some(year) = year_value() {
            let value = month_value().unwrap_or(0);
            if let Ok(month) = Month::try_from(value) {
                if let Some(value) = day_value() {
                    let max = month.length(year);
                    let day = value.clamp(1, max);

                    let date = Date::from_calendar_date(year, month, day)
                        .ok()
                        .map(|date| date.clamp(ctx.min_date, ctx.max_date));

                    tracing::info!("Parsed date: {date:?}");
                    ctx.set_date(date);
                    return;
                }
            }
        }

        ctx.set_date(None);
    });

    let today = UtcDateTime::now().date();

    rsx! {
        div {
            class: "date-picker-group",
            div {
                class: "date-picker-container",
                DateSegment {
                    aria_label: "year",
                    index: 0usize,
                    value: year_value,
                    default: today.year(),
                    on_value_change: move |value: Option<i32>| year_value.set(value),
                    min: 1,
                    max: 9999,
                    max_length: 4,
                    on_format_placeholder: props.on_format_year_placeholder,
                }
                DateSeparator {}
                DateSegment {
                    aria_label: "month",
                    index: 1usize,
                    value: month_value,
                    default: today.month() as u8,
                    on_value_change: move |value: Option<u8>| month_value.set(value),
                    min: Month::January as u8,
                    max: Month::December as u8,
                    max_length: 2,
                    on_format_placeholder: props.on_format_month_placeholder,
                }
                DateSeparator {}
                DateSegment {
                    aria_label: "day",
                    index: 2usize,
                    value: day_value,
                    default: today.day(),
                    on_value_change: move |value: Option<u8>| day_value.set(value),
                    min: 1,
                    max: 31,
                    max_length: 2,
                    on_format_placeholder: props.on_format_day_placeholder,
                }
            }
            {props.children}
        }
    }
}
