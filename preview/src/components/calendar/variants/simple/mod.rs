use super::super::component::*;
use dioxus::prelude::*;
use time::{Date, UtcDateTime};

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
                CalendarHeader {
                    CalendarNavigation {
                        CalendarPreviousMonthButton {}
                        CalendarMonthTitle {}
                        CalendarNextMonthButton {}
                    }
                }
                CalendarGrid {}
            }
        }
    }
}
