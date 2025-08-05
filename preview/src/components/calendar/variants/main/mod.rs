use dioxus::prelude::*;
use dioxus_primitives::calendar::{
    Calendar, CalendarContext, CalendarDate, CalendarGrid, CalendarHeader, CalendarNavigation,
    CalendarNextMonthButton, CalendarPreviousMonthButton,
};

#[component]
pub fn Demo() -> Element {
    let mut selected_date = use_signal(|| None::<CalendarDate>);
    let mut view_date = use_signal(|| CalendarDate::new(2025, 6, 5));
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
                    on_view_change: move |new_view: CalendarDate| {
                        tracing::info!("View changed to: {}-{}", new_view.year, new_view.month);
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
    let month = view_date.month_abbreviation();
    let year = view_date.year;

    rsx! {
        span {
            class: "calendar-month-select-container",
            select {
                class: "calendar-month-select",
                aria_label: "Month",
                onchange: move |e| {
                    let mut view_date = calendar.view_date();
                    view_date.month = e.value().parse().unwrap_or(view_date.month);
                    calendar.set_view_date(view_date);
                },
                for (i, month) in CalendarDate::MONTH_ABBREVIATIONS.iter().enumerate() {
                    option {
                        value: i + 1,
                        selected: calendar.view_date().month == (i as u32 + 1),
                        "{month}"
                    }
                }
            }
            span {
                class: "calendar-month-select-value",
                "{month}"
                svg {
                    class: "select-expand-icon",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    polyline { points: "6 9 12 15 18 9" }
                }
            }
        }

        span {
            class: "calendar-year-select-container",
            select {
                class: "calendar-year-select",
                aria_label: "Year",
                onchange: move |e| {
                    let mut view_date = calendar.view_date();
                    view_date.year = e.value().parse().unwrap_or(view_date.year);
                    calendar.set_view_date(view_date);
                },
                for year in 1925..=2050 {
                    option {
                        value: year,
                        selected: calendar.view_date().year == year,
                        "{year}"
                    }
                }
            }
            span {
                class: "calendar-year-select-value",
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
