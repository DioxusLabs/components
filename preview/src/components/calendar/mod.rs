use dioxus::prelude::*;
use dioxus_primitives::calendar::{
    Calendar, CalendarDate, CalendarGrid, CalendarHeader, CalendarNavigation,
};
#[component]
pub(super) fn Demo() -> Element {
    let mut selected_date = use_signal(|| None::<CalendarDate>);
    let mut view_date = use_signal(|| CalendarDate::new(2024, 5, 15));
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/calendar/style.css"),
        }
        div { class: "calendar-example", style: "padding: 20px;",
            div { class: "calendar",
                Calendar {
                    selected_date: selected_date(),
                    on_date_change: move |date| {
                        println!("Selected date: {:?}", date);
                        selected_date.set(date);
                    },
                    view_date: view_date(),
                    on_view_change: move |new_view: CalendarDate| {
                        println!("View changed to: {}-{}", new_view.year, new_view.month);
                        view_date.set(new_view);
                    },
                    CalendarHeader { CalendarNavigation {} }
                    CalendarGrid {}
                }
            }
            div { class: "selected-date", style: "margin-top: 20px;",
                if let Some(date) = selected_date() {
                    p { style: "font-weight: bold;", "Selected date: {date}" }
                } else {
                    p { "No date selected" }
                }
            }
        }
    }
}
