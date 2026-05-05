use super::super::component::*;
use dioxus::prelude::*;
use time::{macros::date, Date};

const COMPACT_STYLE: &str = r#"
.dx-calendar-example .dx-calendar-grid-cell { width: 1.6rem; font-size: 12px; }
.dx-calendar-example .dx-calendar-grid-day-header { font-size: 11px; }
.dx-calendar-example .dx-calendar-navigation { padding: 0.4rem 2rem 0.2rem; }
.dx-calendar-example .dx-calendar-grid { padding: 0.25rem; }
.dx-calendar-example .dx-calendar-grid-body { gap: 0.1rem; }
.dx-calendar-example .dx-calendar-month-select-value,
.dx-calendar-example .dx-calendar-year-select-value { font-size: 0.85rem; }
.dx-calendar-example .dx-calendar-nav-prev,
.dx-calendar-example .dx-calendar-nav-next { width: 1.4rem; height: 1.4rem; }
"#;

#[component]
pub fn Demo() -> Element {
    let mut selected_date = use_signal(|| None::<Date>);
    let mut view_date = use_signal(|| date!(2026 - 05 - 15));
    rsx! {
        style { {COMPACT_STYLE} }
        div { class: "dx-calendar-example",
            Calendar {
                selected_date: selected_date(),
                on_date_change: move |date| {
                    selected_date.set(date);
                },
                view_date: view_date(),
                on_view_change: move |new_view: Date| {
                    view_date.set(new_view);
                },
                min_date: date!(1995 - 07 - 21),
                max_date: date!(2035 - 09 - 11),
                CalendarView {
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
}
