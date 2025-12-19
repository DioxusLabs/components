use dioxus::prelude::*;

use dioxus_primitives::{
    date_picker::{self, DatePickerInputProps, DatePickerProps, DateRangePickerProps},
    popover::{PopoverContentProps, PopoverTriggerProps},
    ContentAlign,
};

use super::super::calendar::*;
use super::super::popover::*;

#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            date_picker::DatePicker {
                class: "date-picker",
                on_value_change: props.on_value_change,
                selected_date: props.selected_date,
                disabled: props.disabled,
                read_only: props.read_only,
                min_date: props.min_date,
                max_date: props.max_date,
                month_count: props.month_count,
                disabled_ranges: props.disabled_ranges,
                roving_loop: props.roving_loop,
                attributes: props.attributes,
                date_picker::DatePickerPopover {
                    popover_root: PopoverRoot,
                    {props.children}
                }
            }
        }
    }
}

#[component]
pub fn DateRangePicker(props: DateRangePickerProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            date_picker::DateRangePicker {
                class: "date-picker",
                on_range_change: props.on_range_change,
                selected_range: props.selected_range,
                disabled: props.disabled,
                read_only: props.read_only,
                min_date: props.min_date,
                max_date: props.max_date,
                month_count: props.month_count,
                disabled_ranges: props.disabled_ranges,
                roving_loop: props.roving_loop,
                attributes: props.attributes,
                date_picker::DatePickerPopover { popover_root: PopoverRoot, {props.children} }
            }
        }
    }
}

#[component]
pub fn DatePickerInput(props: DatePickerInputProps) -> Element {
    rsx! {
        date_picker::DatePickerInput {
            on_format_day_placeholder: props.on_format_day_placeholder,
            on_format_month_placeholder: props.on_format_month_placeholder,
            on_format_year_placeholder: props.on_format_year_placeholder,
            attributes: props.attributes,
            {props.children}
            DatePickerPopoverTrigger {}
            DatePickerPopoverContent { align: ContentAlign::Center,
                date_picker::DatePickerCalendar { calendar: Calendar,
                    CalendarView {
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
        }
    }
}

#[component]
pub fn DateRangePickerInput(props: DatePickerInputProps) -> Element {
    rsx! {
        date_picker::DateRangePickerInput {
            on_format_day_placeholder: props.on_format_day_placeholder,
            on_format_month_placeholder: props.on_format_month_placeholder,
            on_format_year_placeholder: props.on_format_year_placeholder,
            attributes: props.attributes,
            {props.children}
            DatePickerPopoverTrigger {}
            DatePickerPopoverContent {
                align: ContentAlign::Center,
                date_picker::DateRangePickerCalendar {
                    calendar: RangeCalendar,
                    CalendarView {
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
        }
    }
}

#[component]
pub fn DatePickerPopoverTrigger(props: PopoverTriggerProps) -> Element {
    rsx! {
        PopoverTrigger { aria_label: "Show Calendar", attributes: props.attributes,
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
        PopoverContent {
            class: "popover-content",
            id: props.id,
            side: props.side,
            align: props.align,
            attributes: props.attributes,
            {props.children}
        }
    }
}
