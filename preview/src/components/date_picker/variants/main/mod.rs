use dioxus::prelude::*;
use dioxus_primitives::date_picker::{
    DatePicker, DatePickerCalendar, DatePickerInput, DatePickerTrigger, DatePickerValue,
};

#[component]
pub fn Demo() -> Element {
    let mut value = use_signal(|| None::<DatePickerValue>);

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/date_picker/variants/main/style.css"),
        }
        div {
            DatePicker {
                class: "date-picker",
                value: value(),
                on_value_change: move |date| {
                    tracing::info!("Selected date: {:?}", date);
                    value.set(date);
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
