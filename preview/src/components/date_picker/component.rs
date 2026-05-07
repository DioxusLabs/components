use dioxus::prelude::*;

use dioxus_primitives::icon;
use dioxus_primitives::{
    date_picker::{
        self, DatePickerDaySegmentProps, DatePickerInputProps, DatePickerMonthSegmentProps,
        DatePickerProps, DatePickerSeparatorProps, DatePickerYearSegmentProps,
        DateRangePickerEndValueProps, DateRangePickerProps, DateRangePickerStartValueProps,
    },
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
    let children = props.children.unwrap_or_else(|| {
        rsx! {
            date_picker::DatePickerInputValue {
                on_format_day_placeholder: props.on_format_day_placeholder,
                on_format_month_placeholder: props.on_format_month_placeholder,
                on_format_year_placeholder: props.on_format_year_placeholder,
                DatePickerYearSegment {}
                DatePickerSeparator {}
                DatePickerMonthSegment {}
                DatePickerSeparator {}
                DatePickerDaySegment {}
            }
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
    });

    rsx! {
        date_picker::DatePickerInput {
            on_format_day_placeholder: props.on_format_day_placeholder,
            on_format_month_placeholder: props.on_format_month_placeholder,
            on_format_year_placeholder: props.on_format_year_placeholder,
            attributes: merged,
            {children}
        }
    }
}

#[component]
pub fn DateRangePickerInput(props: DatePickerInputProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_date_picker_group
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    let children = props.children.unwrap_or_else(|| {
        rsx! {
            date_picker::DateRangePickerInputValue {
                on_format_day_placeholder: props.on_format_day_placeholder,
                on_format_month_placeholder: props.on_format_month_placeholder,
                on_format_year_placeholder: props.on_format_year_placeholder,
                DateRangePickerStartValue {
                    DatePickerYearSegment {}
                    DatePickerSeparator {}
                    DatePickerMonthSegment {}
                    DatePickerSeparator {}
                    DatePickerDaySegment {}
                }
                DatePickerSeparator { symbol: '—' }
                DateRangePickerEndValue {
                    DatePickerYearSegment {}
                    DatePickerSeparator {}
                    DatePickerMonthSegment {}
                    DatePickerSeparator {}
                    DatePickerDaySegment {}
                }
            }
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
    });

    rsx! {
        date_picker::DateRangePickerInput {
            on_format_day_placeholder: props.on_format_day_placeholder,
            on_format_month_placeholder: props.on_format_month_placeholder,
            on_format_year_placeholder: props.on_format_year_placeholder,
            attributes: merged,
            {children}
        }
    }
}

#[component]
pub fn DatePickerYearSegment(props: DatePickerYearSegmentProps) -> Element {
    rsx! {
        date_picker::DatePickerYearSegment {
            class: Styles::dx_date_segment,
            attributes: props.attributes,
        }
    }
}

#[component]
pub fn DatePickerMonthSegment(props: DatePickerMonthSegmentProps) -> Element {
    rsx! {
        date_picker::DatePickerMonthSegment {
            class: Styles::dx_date_segment,
            attributes: props.attributes,
        }
    }
}

#[component]
pub fn DatePickerDaySegment(props: DatePickerDaySegmentProps) -> Element {
    rsx! {
        date_picker::DatePickerDaySegment {
            class: Styles::dx_date_segment,
            attributes: props.attributes,
        }
    }
}

#[component]
pub fn DatePickerSeparator(props: DatePickerSeparatorProps) -> Element {
    rsx! {
        date_picker::DatePickerSeparator {
            class: Styles::dx_date_segment,
            symbol: props.symbol,
            attributes: props.attributes,
        }
    }
}

#[component]
pub fn DateRangePickerStartValue(props: DateRangePickerStartValueProps) -> Element {
    rsx! {
        date_picker::DateRangePickerStartValue {
            {props.children}
        }
    }
}

#[component]
pub fn DateRangePickerEndValue(props: DateRangePickerEndValueProps) -> Element {
    rsx! {
        date_picker::DateRangePickerEndValue {
            {props.children}
        }
    }
}

#[component]
pub fn DatePickerPopoverTrigger(props: PopoverTriggerProps) -> Element {
    rsx! {
        PopoverTrigger {
            class: Styles::dx_date_picker_popover_trigger,
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
