use super::super::component::*;
use dioxus::prelude::*;
use dioxus_i18n::tid;

use time::{macros::date, Date, Month, UtcDateTime, Weekday};

#[component]
pub fn Demo() -> Element {
    let mut selected_date = use_signal(|| None::<Date>);
    let mut view_date = use_signal(|| UtcDateTime::now().date());
    rsx! {
        div { class: "calendar-example", style: "padding: 20px;",
            Calendar {
                selected_date: selected_date(),
                on_date_change: move |date| {
                    tracing::info!("Selected date: {:?}", date);
                    selected_date.set(date);
                },
                view_date: view_date(),
                on_view_change: move |new_view: Date| {
                    tracing::info!("View changed to: {}-{}", new_view.year(), new_view.month());
                    view_date.set(new_view);
                },
                on_format_weekday: |weekday: Weekday| tid!(&weekday.to_string()),
                on_format_month: |month: Month| tid!(&month.to_string()),
                min_date: date!(1995-07-21),
                max_date: date!(2035-09-11),
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
