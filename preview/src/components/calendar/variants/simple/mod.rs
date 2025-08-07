use dioxus::prelude::*;
use dioxus_primitives::calendar::{
    Calendar, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
};

use chrono::{Datelike, NaiveDate};

#[component]
pub fn Demo() -> Element {
    let mut selected_date = use_signal(|| None::<NaiveDate>);
    let mut view_date = use_signal(|| NaiveDate::from_ymd_opt(2025, 6, 5).unwrap());
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/calendar/variants/simple/style.css"),
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
                            CalendarMonthTitle {}
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
