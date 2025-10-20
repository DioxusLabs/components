//! Defines the [`DatePicker`] component and its subcomponents, which allowing users to enter or select a date value

use crate::{
    calendar::{weekday_abbreviation, Calendar, CalendarProps},
    focus::{use_focus_controlled_item, use_focus_provider, FocusState},
    popover::*,
    use_unique_id,
};

use dioxus::prelude::*;
use num_integer::Integer;
use std::{fmt::Display, str::FromStr};
use time::{macros::date, Date, Month, UtcDateTime, Weekday};

/// The context provided by the [`DatePicker`] component to its children.
#[derive(Copy, Clone)]
struct DatePickerContext {
    // State
    on_value_change: Callback<Option<Date>>,
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
        let value = { self.selected_date.peek().cloned() };
        if value != date {
            self.on_value_change.call(date);
        }

        self.open.set(false);
    }
}

/// The props for the [`DatePicker`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerProps {
    /// Callback when value changes
    #[props(default)]
    pub on_value_change: Callback<Option<Date>>,

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
            role: "group",
            "aria-label": "Date",
            "data-disabled": (props.disabled)(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DatePickerPopover`] component.
#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerPopoverProps {
    /// Whether the popover is a modal and should capture focus.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub is_modal: ReadSignal<bool>,

    /// The controlled open state of the popover.
    pub open: ReadSignal<Option<bool>>,

    /// The default open state when uncontrolled.
    #[props(default)]
    pub default_open: bool,

    /// Callback fired when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Additional attributes to apply to the popover root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the popover root component.
    pub children: Element,

    /// The popover root component to use.
    #[props(default = PopoverRoot)]
    pub popover_root: fn(PopoverRootProps) -> Element,
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
pub fn DatePickerPopover(props: DatePickerPopoverProps) -> Element {
    let ctx = use_context::<DatePickerContext>();
    let mut open = ctx.open;

    let PopoverRoot = props.popover_root;

    rsx! {
        PopoverRoot {
            open: open(),
            on_open_change: move |v| open.set(v),
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`Calendar`] component.
#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerCalendarProps {
    /// The selected date
    #[props(default)]
    pub selected_date: ReadSignal<Option<Date>>,

    /// Callback when selected date changes
    #[props(default)]
    pub on_date_change: Callback<Option<Date>>,

    /// Callback when display weekday
    #[props(default = Callback::new(|weekday: Weekday| weekday_abbreviation(weekday).to_string()))]
    pub on_format_weekday: Callback<Weekday, String>,

    /// Callback when display month
    #[props(default = Callback::new(|month: Month| month.to_string()))]
    pub on_format_month: Callback<Month, String>,

    /// The month being viewed
    #[props(default = ReadSignal::new(Signal::new(UtcDateTime::now().date())))]
    pub view_date: ReadSignal<Date>,

    /// The current date (used for highlighting today)
    #[props(default = UtcDateTime::now().date())]
    pub today: Date,

    /// Callback when view date changes
    #[props(default)]
    pub on_view_change: Callback<Date>,

    /// Whether the calendar is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// First day of the week
    #[props(default = Weekday::Sunday)]
    pub first_day_of_week: Weekday,

    /// Lower limit of the range of available dates
    #[props(default = date!(1925-01-01))]
    pub min_date: Date,

    /// Upper limit of the range of available dates
    #[props(default = date!(2050-12-31))]
    pub max_date: Date,

    /// Additional attributes to extend the calendar element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the calendar element
    pub children: Element,

    /// The calendar to render with
    #[props(default = Calendar)]
    pub calendar: fn(CalendarProps) -> Element,
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
pub fn DatePickerCalendar(props: DatePickerCalendarProps) -> Element {
    let mut ctx = use_context::<DatePickerContext>();
    #[allow(non_snake_case)]
    let Calendar = props.calendar;
    let mut view_date = use_signal(|| UtcDateTime::now().date());
    use_effect(move || {
        if let Some(date) = (props.selected_date)() {
            view_date.set(date);
        }
    });

    rsx! {
        Calendar {
            selected_date: ctx.selected_date,
            on_date_change: move |date| {
                tracing::info!("calendar selected date {date:?}");
                ctx.set_date(date)
            },
            on_format_weekday: props.on_format_weekday,
            on_format_month: props.on_format_month,
            view_date: view_date(),
            on_view_change: move |date| view_date.set(date),
            today: props.today,
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
    pub min: ReadSignal<T>,

    // The maximum value
    pub max: ReadSignal<T>,

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
        let min = props.min.cloned();
        let max = props.max.cloned();

        let value = text.parse::<T>().map(|v| v.min(max)).ok();
        if let Some(value) = value {
            let inRange = value >= min && value <= max;

            // If adding a new digit would exceed max, move to next segment
            let newValue = (text + "0").parse::<T>().unwrap_or(value);
            if inRange && newValue > max {
                ctx.focus.focus_next();
            }
        };

        props.on_value_change.call(value);
    };
    use_effect(move || {
        // If this item is not focused, always keep the value clamped
        if !ctx.focus.is_focused(props.index.cloned()) {
            if let Some(value) = (props.value)() {
                let clamped_value = value.clamp(props.min.cloned(), props.max.cloned());
                if clamped_value != value {
                    props.on_value_change.call(Some(clamped_value));
                }
            }
        }
    });

    let roll_value = move |value: T| {
        let min = props.min.cloned();
        let max = props.max.cloned();
        if value < min {
            max
        } else if value > max {
            min
        } else {
            value
        }
    };

    let handle_keydown = move |event: Event<KeyboardData>| {
        let key = event.key();
        match key {
            Key::Character(actual_char) => {
                // Don't block keyboard shortcuts
                if event.modifiers().ctrl() || event.modifiers().meta() || event.modifiers().alt() {
                    return;
                }
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
                if event.modifiers().ctrl() || event.modifiers().meta() {
                    text.clear();
                } else {
                    text.pop();
                }
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
            Key::Enter => {
                ctx.focus.focus_next();
                event.prevent_default();
                event.stop_propagation();
            }
            Key::ArrowUp => {
                let value = match (props.value)() {
                    Some(mut value) => {
                        value.inc();
                        roll_value(value)
                    }
                    None => props.default,
                };
                props.on_value_change.call(Some(value));
            }
            Key::ArrowDown => {
                let value = match (props.value)() {
                    Some(mut value) => {
                        value.dec();
                        roll_value(value)
                    }
                    None => props.default,
                };
                props.on_value_change.call(Some(value));
            }
            _ => (),
        }
    };

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
            tabindex: "0",
            enterkeyhint: "next",
            onkeydown: handle_keydown,
            onmounted,
            onfocus: move |_| {
                reset_value.set(true);
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
        if let (Some(year), Some(month), Some(day)) = (
            year_value(),
            month_value().and_then(|m| Month::try_from(m).ok()),
            day_value(),
        ) {
            if let Some(date) = Date::from_calendar_date(year, month, day)
                .ok()
                .filter(|date| ctx.min_date <= *date && *date <= ctx.max_date)
            {
                tracing::info!("Parsed date: {date:?}");
                ctx.set_date(Some(date));
            }
        }
    });

    let today = UtcDateTime::now().date();

    let min_year = ctx.min_date.year();
    let max_year = ctx.max_date.year();
    let min_month = match year_value() {
        Some(year) if year == min_year => ctx.min_date.month(),
        _ => Month::January,
    };
    let max_month = match year_value() {
        Some(year) if year == max_year => ctx.max_date.month(),
        _ => Month::December,
    };
    let min_day = match (year_value(), month_value()) {
        (Some(year), Some(month)) if year == min_year && month == ctx.min_date.month() as u8 => {
            ctx.min_date.day()
        }
        _ => 1,
    };
    let max_day = match (year_value(), month_value()) {
        (Some(year), Some(month)) if year == max_year && month == ctx.max_date.month() as u8 => {
            ctx.max_date.day()
        }
        (Some(year), Some(month)) => {
            if let Ok(month) = Month::try_from(month) {
                month.length(year)
            } else {
                31
            }
        }
        _ => 31,
    };

    rsx! {
        div {
            class: "date-picker-group",
            ..props.attributes,
            DateSegment {
                aria_label: "year",
                index: 0usize,
                value: year_value,
                default: today.year(),
                on_value_change: move |value: Option<i32>| year_value.set(value),
                min: min_year,
                max: max_year,
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
                min: min_month as u8,
                max: max_month as u8,
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
                min: min_day,
                max: max_day,
                max_length: 2,
                on_format_placeholder: props.on_format_day_placeholder,
            }
            {props.children}
        }
    }
}
