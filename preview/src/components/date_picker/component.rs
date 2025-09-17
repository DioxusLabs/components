use crate::components::calendar::component::*;
use dioxus::prelude::*;

use dioxus_primitives::{
    calendar::CalendarProps,
    date_picker::{self, DatePickerInputProps, DatePickerProps, DatePickerTriggerProps},
};

use time::UtcDateTime;

#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/date_picker/style.css"),
        }
        div {
            date_picker::DatePicker {
                class: "date-picker",
                value: props.value,
                on_value_change: props.on_value_change,
                selected_date: props.selected_date,
                disabled: props.disabled,
                read_only: props.read_only,
                separator: props.separator,
                on_format_placeholder: props.on_format_placeholder,
                attributes: props.attributes,
                {props.children}
            }
        }
    }
}

#[component]
pub fn DatePickerInput(props: DatePickerInputProps) -> Element {
    rsx! {
        date_picker::DatePickerInput {
            class: "date-picker-input",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn DatePickerTrigger(props: DatePickerTriggerProps) -> Element {
    rsx! {
        date_picker::DatePickerTrigger {
            class: "date-picker-trigger",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn DatePickerCalendar(props: CalendarProps) -> Element {
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
            min_date: props.min_date,
            max_date: props.max_date,
            attributes: props.attributes,
            CalendarHeader {
                CalendarNavigation {
                    CalendarPreviousMonthButton {}
                    CalendarSelectMonth {}
                    CalendarSelectYear {}
                    CalendarNextMonthButton {}
                }
            }
            CalendarGrid {}
        }
    }
}
