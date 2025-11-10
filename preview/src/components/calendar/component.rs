use dioxus::prelude::*;
use dioxus_primitives::calendar::{
    self, CalendarDayProps, CalendarGridProps, CalendarHeaderProps, CalendarMonthTitleProps,
    CalendarNavigationProps, CalendarProps, CalendarSelectMonthProps, CalendarSelectYearProps,
    RangeCalendarProps,
};

#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "calendar",
            calendar::Calendar {
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
                disabled_ranges: props.disabled_ranges,
                attributes: props.attributes,
                {props.children}
            }
        }
    }
}

#[component]
pub fn RangeCalendar(props: RangeCalendarProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "calendar",
            calendar::RangeCalendar {
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
                disabled_ranges: props.disabled_ranges,
                attributes: props.attributes,
                {props.children}
            }
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
        calendar::CalendarNavigation { attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn CalendarPreviousMonthButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        calendar::CalendarPreviousMonthButton { attributes,
            svg {
                class: "calendar-previous-month-icon",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                polyline { points: "15 6 9 12 15 18" }
            }
        }
    }
}

#[component]
pub fn CalendarNextMonthButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        calendar::CalendarNextMonthButton { attributes,
            svg {
                class: "calendar-next-month-icon",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                polyline { points: "9 18 15 12 9 6" }
            }
        }
    }
}

#[component]
pub fn CalendarSelectMonth(props: CalendarSelectMonthProps) -> Element {
    rsx! {
        calendar::CalendarSelectMonth { class: "calendar-month-select", attributes: props.attributes }
    }
}

#[component]
pub fn CalendarSelectYear(props: CalendarSelectYearProps) -> Element {
    rsx! {
        calendar::CalendarSelectYear { class: "calendar-year-select", attributes: props.attributes }
    }
}

#[component]
pub fn CalendarGrid(props: CalendarGridProps) -> Element {
    rsx! {
        calendar::CalendarGrid {
            id: props.id,
            show_week_numbers: props.show_week_numbers,
            render_day: props.render_day,
            attributes: props.attributes,
        }
    }
}

#[component]
pub fn CalendarMonthTitle(props: CalendarMonthTitleProps) -> Element {
    calendar::CalendarMonthTitle(props)
}

#[component]
pub fn CalendarDay(props: CalendarDayProps) -> Element {
    calendar::CalendarDay(props)
}
