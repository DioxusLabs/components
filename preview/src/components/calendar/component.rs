use dioxus::prelude::*;
use dioxus_primitives::calendar::{
    self, CalendarDayProps, CalendarGridProps, CalendarHeaderProps, CalendarMonthTitleProps,
    CalendarNavigationProps, CalendarProps, CalendarSelectMonthProps, CalendarSelectYearProps,
    RangeCalendarProps,
};
use dioxus_primitives::icon::Icon;

#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        calendar::Calendar {
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
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        calendar::RangeCalendar {
            class: "calendar",
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
            class: "calendar-view",
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
        calendar::CalendarNavigation { attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn CalendarPreviousMonthButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        calendar::CalendarPreviousMonthButton { attributes,
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
        calendar::CalendarNextMonthButton { attributes,
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
            class: "calendar-month-select",
            attributes: props.attributes,
            DropDownIcon { }
        }
    }
}

#[component]
pub fn CalendarSelectYear(props: CalendarSelectYearProps) -> Element {
    rsx! {
        calendar::CalendarSelectYear {
            class: "calendar-year-select",
            attributes: props.attributes,
            DropDownIcon { }
        }
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
