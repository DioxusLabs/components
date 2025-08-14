use dioxus::prelude::*;
use dioxus_i18n::tid;
use dioxus_primitives::calendar::{
    Calendar, CalendarGrid, CalendarHeader, CalendarNavigation, CalendarNextMonthButton,
    CalendarPreviousMonthButton, CalendarSelectMonth, CalendarSelectYear,
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
                    on_format_month: Callback::new(|month: Month| tid!(month.name())),
                    min_date: NaiveDate::from_ymd_opt(1995, 7, 21).unwrap(),
                    max_date: NaiveDate::from_ymd_opt(2035, 9, 11).unwrap(),
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
                            CalendarSelectMonth {}
                            CalendarSelectYear {}
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