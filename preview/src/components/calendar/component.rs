use dioxus::prelude::*;
use dioxus_primitives::calendar::{
    self, CalendarDayProps, CalendarGridProps, CalendarHeaderProps, CalendarMonthTitleProps,
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
pub fn CalendarGrid(props: CalendarGridProps) -> Element {
    rsx! {
        calendar::CalendarGrid {
            class: Styles::dx_calendar_grid,
            header_class: Some(Styles::dx_calendar_grid_header.to_string()),
            day_header_class: Some(Styles::dx_calendar_grid_day_header.to_string()),
            body_class: Some(Styles::dx_calendar_grid_body.to_string()),
            week_class: Some(Styles::dx_calendar_grid_week.to_string()),
            weeknum_class: Some(Styles::dx_calendar_grid_weeknum.to_string()),
            id: props.id,
            show_week_numbers: props.show_week_numbers,
            render_day: Callback::new(|date| rsx! { CalendarDay { date } }),
            attributes: props.attributes,
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
