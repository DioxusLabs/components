//! Defines the [`DatePicker`] component and its subcomponents, which allowing users to enter or select a date value

use crate::calendar::{
    Calendar, CalendarGrid, CalendarHeader, CalendarNavigation, CalendarNextMonthButton,
    CalendarPreviousMonthButton, CalendarSelectMonth, CalendarSelectYear,
};
use crate::{
    focus::{use_focus_provider, FocusState},
    use_animated_open, use_effect, use_id_or, use_unique_id,
};

use dioxus::prelude::*;
use time::{macros::format_description, Date, UtcDateTime};

/// The context provided by the [`DatePicker`] component to its children.
#[derive(Copy, Clone)]
pub struct DatePickerContext {
    /// The selected date
    pub selected_date: Signal<Option<Date>>,

    /// If the date select is open
    pub open: Signal<bool>,
    /// The ID of the calendar for ARIA attributes
    pub calendar_id: Signal<Option<String>>,
    /// The focus state for the date picker
    focus_state: FocusState,

    disabled: ReadOnlySignal<bool>,
}

/// The props for the [`DatePicker`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerProps {
    /// The selected date
    pub selected_date: Signal<Option<Date>>,

    /// Callback when selected date changes
    #[props(default)]
    pub on_date_change: Callback<Option<Date>>,

    /// Whether focus should loop around when reaching the end
    #[props(default = ReadOnlySignal::new(Signal::new(true)))]
    pub roving_loop: ReadOnlySignal<bool>,

    /// Whether the date picker is disabled
    #[props(default)]
    pub disabled: ReadOnlySignal<bool>,

    /// Additional attributes to extend the date picker element
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children of the date picker element
    children: Element,
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
    let calendar_id = use_signal(|| None);
    let focus_state = use_focus_provider(props.roving_loop);

    // Create context provider for child components
    use_context_provider(|| DatePickerContext {
        open,
        calendar_id,
        focus_state,
        selected_date: props.selected_date,
        disabled: props.disabled,
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
    attributes: Vec<Attribute>,

    /// The children to render inside the trigger
    children: Element,
}

/// # DatePickerTrigger
///
/// The trigger button for the [`DatePicker`](super::date_picker::DatePicker) component which controls if the [`DatePickerCalendar`](super::date_picker::DatePickerCalendar) is rendered.
///
/// This must be used inside a [`DatePicker`](super::date_picker::DatePicker) component.
///
/// ```rust
/// ```
#[component]
pub fn DatePickerTrigger(props: DatePickerTriggerProps) -> Element {
    let ctx = use_context::<DatePickerContext>();
    let mut open = ctx.open;

    rsx! {
        button {
            // Standard HTML attributes
            disabled: (ctx.disabled)(),

            onclick: move |_| {
                open.toggle();
            },

            // ARIA attributes
            aria_haspopup: "calendarbox",
            aria_expanded: open(),
            aria_controls: ctx.calendar_id,

            // Pass through other attributes
            ..props.attributes,

            // Render children (options)
            {props.children}
        }
    }
}

/// The props for the [`DatePickerInput`] component
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerInputProps {
    /// Additional attributes for the value element
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children of the date picker element
    children: Element,
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

    let display_value = use_memo(move || match (ctx.selected_date)() {
        Some(date) => date.to_string(),
        None => String::default(),
    });

    rsx! {
        input {
            placeholder: "YYYY-MM-DD",
            value: display_value,
            oninput: move |e| {
                let text = e.value().parse().unwrap_or(display_value());

                let format = format_description!("[year]-[month]-[day]");
                if let Ok(date) = Date::parse(&text, &format) {
                    ctx.selected_date.set(Some(date));
                }
            },
            ..props.attributes,
        }
        {props.children}
    }
}

/// The props for the [`DatePickerCalendar`] component
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerCalendarProps {
    /// The ID of the calendar for ARIA attributes
    #[props(default)]
    pub id: ReadOnlySignal<Option<String>>,

    /// Additional attributes for the list
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

/// # DatePickerCalendar
///
/// The Calendar popover for the [`DatePicker`](super::date_picker::DatePicker).
/// The calendar will only be rendered when the DatePicker is open.
///
/// This must be used inside a [`DatePicker`](super::date_picker::DatePicker).
///
/// ## Example
///
/// ```rust
/// ```
#[component]
pub fn DatePickerCalendar(props: DatePickerCalendarProps) -> Element {
    let mut ctx = use_context::<DatePickerContext>();

    let id = use_unique_id();
    let id = use_id_or(id, props.id);
    use_effect(move || {
        ctx.calendar_id.set(Some(id()));
    });

    let mut open = ctx.open;
    let mut calendarbox_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    let focused = move || open() && !ctx.focus_state.any_focused();

    use_effect(move || {
        let Some(calendarbox_ref) = calendarbox_ref() else {
            return;
        };
        if focused() {
            spawn(async move {
                _ = calendarbox_ref.set_focus(true);
            });
        }
    });

    let onkeydown = move |event: KeyboardEvent| {
        let key = event.key();

        if key == Key::Escape {
            open.set(false);
            event.prevent_default();
            event.stop_propagation();
        }
    };

    let render = use_animated_open(id, open);
    let render = use_memo(render);

    let mut view_date = use_signal(|| UtcDateTime::now().date());

    use_effect(move || {
        if let Some(date) = (ctx.selected_date)() {
            view_date.set(date);
        }
    });

    rsx! {
        if render() {
            div {
                id,
                role: "calendarbox",
                tabindex: if focused() { "0" } else { "-1" },

                // Data attributes
                "data-state": if open() { "open" } else { "closed" },

                onmounted: move |evt| calendarbox_ref.set(Some(evt.data())),
                onkeydown,

                ..props.attributes,

                Calendar {
                    selected_date: (ctx.selected_date)(),
                    on_date_change: move |date| {
                        ctx.selected_date.set(date);
                        open.set(false);
                    },
                    view_date: view_date(),
                    on_view_change: move |new_view: Date| {
                        view_date.set(new_view);
                    },
                    CalendarHeader {
                        CalendarNavigation {
                            CalendarPreviousMonthButton {
                                svg {
                                    class: "calendar-previous-month-icon",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    polyline { points: "15 6 9 12 15 18" }
                                }
                            }
                            CalendarSelectMonth { class: "calendar-month-select" }
                            CalendarSelectYear { class: "calendar-year-select" }
                            CalendarNextMonthButton {
                                svg {
                                    class: "calendar-next-month-icon",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    polyline { points: "9 18 15 12 9 6" }
                                }
                            }
                        }
                    }
                    CalendarGrid {}
                }
            }
        }
    }
}
