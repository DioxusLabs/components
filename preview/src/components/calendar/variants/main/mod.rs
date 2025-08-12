use dioxus::prelude::*;
use dioxus_i18n::tid;
use dioxus_primitives::calendar::{
    Calendar, CalendarContext, CalendarGrid, CalendarHeader, CalendarNavigation,
    CalendarNextMonthButton, CalendarPreviousMonthButton,
};

use chrono::{Datelike, Month, NaiveDate, Utc, Weekday};

#[component]
pub fn Demo() -> Element {
    let mut selected_date = use_signal(|| None::<NaiveDate>);
    let mut view_date = use_signal(|| Utc::now().date_naive());
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/calendar/variants/main/style.css"),
        }
        div { class: "calendar-example", style: "padding: 20px;",
            div { class: "calendar",
                Calendar {
                    selected_date: selected_date(),
                    on_date_change: move |date| {
                        tracing::info!("Selected date: {:?}", date);
                        selected_date.set(date);
                    },
                    view_date: view_date(),
                    on_view_change: move |new_view: NaiveDate| {
                        tracing::info!("View changed to: {}-{}", new_view.year(), new_view.month());
                        view_date.set(new_view);
                    },
                    on_format_weekday: Callback::new(|weekday: Weekday| tid!(&weekday.to_string())),
                    CalendarHeader {
                        CalendarNavigation {
                            CalendarPreviousMonthButton {
                                svg {
                                    class: "calendar-previous-month-icon",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    polyline { points: "15 6 9 12 15 18" }
                                }
                            }
                            MonthTitle {}
                            CalendarNextMonthButton {
                                svg {
                                    class: "calendar-next-month-icon",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    polyline { points: "9 18 15 12 9 6" }
                                }
                            }
                        }
                    }
                    CalendarGrid {}
                }
            }
        }
    }
}

#[component]
fn MonthTitle() -> Element {
    let calendar: CalendarContext = use_context();
    let view_date = calendar.view_date();
    let month_name = Month::try_from(view_date.month() as u8).unwrap().name();
    let year = view_date.year();
    let months = (1..=12).map(|month_i| Month::try_from(month_i).unwrap());

    rsx! {
        span { class: "calendar-month-select-container",
            select {
                class: "calendar-month-select",
                aria_label: "Month",
                onchange: move |e| {
                    let mut view_date = calendar.view_date();
                    let cur_month = e.value().parse().unwrap_or(view_date.month0());
                    view_date = view_date.with_month0(cur_month).unwrap_or(view_date);
                    calendar.set_view_date(view_date);
                },
                for (i , month) in months.enumerate() {
                    option {
                        value: i,
                        selected: calendar.view_date().month0() == i as u32,
                        {tid!(month.name())}
                    }
                }
            }
            span { class: "calendar-month-select-value",
                {tid!(month_name)}
                svg {
                    class: "select-expand-icon",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    polyline { points: "6 9 12 15 18 9" }
                }
            }
        }

        span { class: "calendar-year-select-container",
            select {
                class: "calendar-year-select",
                aria_label: "Year",
                onchange: move |e| {
                    let mut view_date = calendar.view_date();
                    let year = e.value().parse().unwrap_or(view_date.year());
                    view_date = view_date.with_year(year).unwrap_or(view_date);
                    calendar.set_view_date(view_date);
                },
                for year in 1925..=2050 {
                    option {
                        value: year,
                        selected: calendar.view_date().year() == year,
                        "{year}"
                    }
                }
            }
            span { class: "calendar-year-select-value",
                "{year}"
                svg {
                    class: "select-expand-icon",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    polyline { points: "6 9 12 15 18 9" }
                }
            }
        }
    }
}