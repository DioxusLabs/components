use dioxus::prelude::*;

use dioxus_primitives::icon;
use dioxus_primitives::{
    date_picker::{self, DatePickerInputProps, DatePickerProps, DateRangePickerProps},
    dioxus_attributes::attributes,
    merge_attributes,
    popover::{PopoverContentProps, PopoverTriggerProps},
    ContentAlign,
};

use super::super::calendar::*;
use super::super::popover::*;

#[css_module("/src/components/date_picker/style.css")]
struct Styles;

#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_date_picker
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        div {
            date_picker::DatePicker {
                on_value_change: props.on_value_change,
                selected_date: props.selected_date,
                disabled: props.disabled,
                read_only: props.read_only,
                min_date: props.min_date,
                max_date: props.max_date,
                month_count: props.month_count,
                disabled_ranges: props.disabled_ranges,
                roving_loop: props.roving_loop,
                attributes: merged,
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
    let base = attributes!(div {
        class: Styles::dx_date_picker
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        div {
            date_picker::DateRangePicker {
                on_range_change: props.on_range_change,
                selected_range: props.selected_range,
                disabled: props.disabled,
                read_only: props.read_only,
                min_date: props.min_date,
                max_date: props.max_date,
                month_count: props.month_count,
                disabled_ranges: props.disabled_ranges,
                roving_loop: props.roving_loop,
                attributes: merged,
                date_picker::DatePickerPopover { popover_root: PopoverRoot, {props.children} }
            }
        }
    }
}

#[component]
pub fn DatePickerInput(props: DatePickerInputProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_date_picker_group
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        date_picker::DatePickerInput {
            segment_class: Some(Styles::dx_date_segment.to_string()),
            on_format_day_placeholder: props.on_format_day_placeholder,
            on_format_month_placeholder: props.on_format_month_placeholder,
            on_format_year_placeholder: props.on_format_year_placeholder,
            attributes: merged,
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
    let base = attributes!(div {
        class: Styles::dx_date_picker_group
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        date_picker::DateRangePickerInput {
            segment_class: Some(Styles::dx_date_segment.to_string()),
            on_format_day_placeholder: props.on_format_day_placeholder,
            on_format_month_placeholder: props.on_format_month_placeholder,
            on_format_year_placeholder: props.on_format_year_placeholder,
            attributes: merged,
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
        PopoverTrigger {
            class: Styles::dx_date_picker_popover_trigger.to_string(),
            aria_label: "Show Calendar",
            attributes: props.attributes,
            icon::Icon {
                class: Styles::dx_date_picker_trigger,
                width: "20px",
                height: "20px",
                stroke: "var(--primary-color-7)",
                polyline { points: "6 9 12 15 18 9" }
            }
        }
    }
}

#[component]
pub fn DatePickerPopoverContent(props: PopoverContentProps) -> Element {
    rsx! {
        PopoverContent {
            class: Styles::dx_date_picker_popover_content.to_string(),
            id: props.id,
            side: props.side,
            align: props.align,
            attributes: props.attributes,
            {props.children}
        }
    }
}
