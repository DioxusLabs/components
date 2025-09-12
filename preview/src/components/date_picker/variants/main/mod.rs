use dioxus::prelude::*;
use dioxus_primitives::date_picker::{
    DatePicker, DatePickerCalendar, DatePickerInput, DatePickerTrigger,
};

use time::Date;

#[component]
pub fn Demo() -> Element {
    let mut selected_date = use_signal(|| None::<Date>);

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/date_picker/variants/main/style.css"),
        }
        div {
            DatePicker {
                class: "date-picker",
                selected_date: selected_date,
                on_date_change: move |date| {
                    tracing::info!("Selected date: {:?}", date);
                    selected_date.set(date);
                },
                DatePickerInput {
                    class: "date-picker-input",
                    DatePickerTrigger {
                    class: "date-picker-trigger",
                    aria_label: "DatePicker Trigger",
                    svg {
                        class: "date-picker-expand-icon",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        polyline { points: "6 9 12 15 18 9" }
                    }
                }
                }
                DatePickerCalendar { class: "date-picker-calendar" }
            }
        }
    }
}
