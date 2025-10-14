use dioxus::prelude::*;

use dioxus_primitives::{
    calendar::CalendarProps,
    date_picker::{self, DatePickerInputProps, DatePickerProps},
    popover::{PopoverContentProps, PopoverRootProps, PopoverTriggerProps},
};

use crate::components::calendar::component::*;

#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            date_picker::DatePicker {
                class: "date-picker",
                value: props.value,
                on_value_change: props.on_value_change,
                selected_date: props.selected_date,
                disabled: props.disabled,
                read_only: props.read_only,
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
            on_format_day_placeholder: props.on_format_day_placeholder,
            on_format_month_placeholder: props.on_format_month_placeholder,
            on_format_year_placeholder: props.on_format_year_placeholder,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn DatePickerPopover(props: PopoverRootProps) -> Element {
    rsx! {
        date_picker::DatePickerPopover {
            class: "popover",
            is_modal: props.is_modal,
            default_open: props.default_open,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn DatePickerPopoverTrigger(props: PopoverTriggerProps) -> Element {
    rsx! {
        date_picker::DatePickerPopoverTrigger {
            class: "date-picker-trigger",
            attributes: props.attributes,
            svg {
                class: "date-picker-expand-icon",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                polyline { points: "6 9 12 15 18 9" }
            }
        }
    }
}

#[component]
pub fn DatePickerPopoverContent(props: PopoverContentProps) -> Element {
    rsx! {
        date_picker::DatePickerPopoverContent {
            class: "popover-content",
            id: props.id,
            side: props.side,
            align: props.align,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn DatePickerCalendar(props: CalendarProps) -> Element {
    rsx! {
        date_picker::DatePickerCalendar {
            class: "calendar",
            selected_date: props.selected_date,
            on_date_change: props.on_date_change,
            on_format_weekday: props.on_format_weekday,
            on_format_month: props.on_format_month,
            view_date: props.view_date,
            today: props.today,
            on_view_change: props.on_view_change,
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
