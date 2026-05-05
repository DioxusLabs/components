use dioxus::prelude::*;
use dioxus_primitives::calendar::{
    self, CalendarDayProps, CalendarGridBodyProps, CalendarGridCellProps,
    CalendarGridDayHeaderProps, CalendarGridHeadProps, CalendarGridHeaderRowProps,
    CalendarGridRootProps, CalendarGridWeekProps, CalendarHeaderProps, CalendarMonthTitleProps,
    CalendarNavigationProps, CalendarProps, CalendarSelectMonthProps, CalendarSelectYearProps,
    RangeCalendarProps,
};
use dioxus_primitives::icon::Icon;

#[css_module("/src/components/calendar/style.css")]
struct Styles;

#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    rsx! {
        calendar::Calendar {
            class: Styles::dx_calendar,
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
            month_count: props.month_count,
            disabled_ranges: props.disabled_ranges,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn RangeCalendar(props: RangeCalendarProps) -> Element {
    rsx! {
        calendar::RangeCalendar {
            class: Styles::dx_calendar,
            selected_range: props.selected_range,
            on_range_change: props.on_range_change,
            on_format_weekday: props.on_format_weekday,
            on_format_month: props.on_format_month,
            view_date: props.view_date,
            today: props.today,
            on_view_change: props.on_view_change,
            disabled: props.disabled,
            first_day_of_week: props.first_day_of_week,
            min_date: props.min_date,
            max_date: props.max_date,
            month_count: props.month_count,
            disabled_ranges: props.disabled_ranges,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn CalendarView(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: Styles::dx_calendar_view,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CalendarHeader(props: CalendarHeaderProps) -> Element {
    rsx! {
        calendar::CalendarHeader { id: props.id, attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn CalendarNavigation(props: CalendarNavigationProps) -> Element {
    rsx! {
        calendar::CalendarNavigation { class: Styles::dx_calendar_navigation, attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn CalendarPreviousMonthButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        calendar::CalendarPreviousMonthButton { class: Styles::dx_calendar_nav_prev, attributes,
            Icon {
                width: "20px",
                height: "20px",
                path { d: "m15 18-6-6 6-6" }
            }
        }
    }
}

#[component]
pub fn CalendarNextMonthButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        calendar::CalendarNextMonthButton { class: Styles::dx_calendar_nav_next, attributes,
            Icon {
                width: "20px",
                height: "20px",
                path { d: "m9 18 6-6-6-6" }
            }
        }
    }
}

#[component]
pub fn CalendarSelectMonth(props: CalendarSelectMonthProps) -> Element {
    rsx! {
        calendar::CalendarSelectMonth {
            container_class: Some(Styles::dx_calendar_month_select_container.to_string()),
            value_class: Some(Styles::dx_calendar_month_select_value.to_string()),
            class: Styles::dx_calendar_month_select,
            attributes: props.attributes,
            DropDownIcon { }
        }
    }
}

#[component]
pub fn CalendarSelectYear(props: CalendarSelectYearProps) -> Element {
    rsx! {
        calendar::CalendarSelectYear {
            container_class: Some(Styles::dx_calendar_year_select_container.to_string()),
            value_class: Some(Styles::dx_calendar_year_select_value.to_string()),
            class: Styles::dx_calendar_year_select,
            attributes: props.attributes,
            DropDownIcon { }
        }
    }
}

#[component]
pub fn CalendarGrid(
    #[props(default)] id: Option<String>,
    #[props(default)] show_week_numbers: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let _ = show_week_numbers;
    let grid = calendar::use_calendar_grid();

    rsx! {
        CalendarGridRoot { id, attributes,
            CalendarGridHead {
                CalendarGridHeaderRow {
                    for weekday in grid.weekdays().iter().cloned() {
                        CalendarGridDayHeader {
                            key: "{weekday.weekday():?}",
                            weekday: weekday.weekday(),
                            {weekday.label().to_string()}
                        }
                    }
                }
            }
            CalendarGridBody {
                for week in grid.weeks() {
                    CalendarGridWeek {
                        for date in week.iter().copied() {
                            CalendarGridCell {
                                key: "{date}",
                                date,
                                CalendarDay { date }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn CalendarGridRoot(props: CalendarGridRootProps) -> Element {
    rsx! {
        calendar::CalendarGridRoot {
            class: Styles::dx_calendar_grid,
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn CalendarGridHead(props: CalendarGridHeadProps) -> Element {
    rsx! {
        calendar::CalendarGridHead {
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn CalendarGridHeaderRow(props: CalendarGridHeaderRowProps) -> Element {
    rsx! {
        calendar::CalendarGridHeaderRow {
            class: Styles::dx_calendar_grid_header,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn CalendarGridDayHeader(props: CalendarGridDayHeaderProps) -> Element {
    rsx! {
        calendar::CalendarGridDayHeader {
            class: Styles::dx_calendar_grid_day_header,
            weekday: props.weekday,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn CalendarGridBody(props: CalendarGridBodyProps) -> Element {
    rsx! {
        calendar::CalendarGridBody {
            class: Styles::dx_calendar_grid_body,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn CalendarGridWeek(props: CalendarGridWeekProps) -> Element {
    rsx! {
        calendar::CalendarGridWeek {
            class: Styles::dx_calendar_grid_week,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn CalendarGridCell(props: CalendarGridCellProps) -> Element {
    rsx! {
        calendar::CalendarGridCell {
            date: props.date,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn CalendarMonthTitle(props: CalendarMonthTitleProps) -> Element {
    rsx! {
        calendar::CalendarMonthTitle {
            class: Styles::dx_calendar_month_title,
            attributes: props.attributes,
        }
    }
}

#[component]
pub fn CalendarDay(props: CalendarDayProps) -> Element {
    rsx! {
        calendar::CalendarDay {
            class: Styles::dx_calendar_grid_cell,
            date: props.date,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
fn DropDownIcon() -> Element {
    rsx! {
        Icon {
            width: "20px",
            height: "20px",
            stroke: "var(--secondary-color-4)",
            path { d: "m6 9 6 6 6-6" }
        }
    }
}
