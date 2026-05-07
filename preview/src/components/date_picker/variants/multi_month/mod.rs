use super::super::component::*;
use crate::components::calendar::{
    CalendarGrid, CalendarHeader, CalendarNavigation, CalendarNextMonthButton,
    CalendarPreviousMonthButton, CalendarSelectMonth, CalendarSelectYear, CalendarView,
    RangeCalendar,
};
use dioxus::prelude::*;

use dioxus_primitives::{calendar::DateRange, date_picker, ContentAlign};

#[component]
pub fn Demo() -> Element {
    let mut selected_range = use_signal(|| None::<DateRange>);

    rsx! {
        div {
            DateRangePicker {
                selected_range: selected_range(),
                on_range_change: move |range| {
                    tracing::info!("Selected range: {:?}", range);
                    selected_range.set(range);
                },
                DateRangePickerInput {
                    date_picker::DateRangePickerInputValue {
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
                    DatePickerPopoverContent { align: ContentAlign::Center,
                        date_picker::DateRangePickerCalendar { calendar: RangeCalendar,
                            CalendarView {
                                CalendarHeader {
                                    CalendarNavigation {
                                        CalendarPreviousMonthButton {}
                                        CalendarSelectMonth {}
                                        CalendarSelectYear {}
                                    }
                                }
                                CalendarGrid {}
                            }
                            CalendarView {
                                CalendarHeader {
                                    CalendarNavigation {
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
    }
}
