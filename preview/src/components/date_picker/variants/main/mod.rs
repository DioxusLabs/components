use super::super::component::*;
use dioxus::prelude::*;

use dioxus_i18n::tid;
use dioxus_primitives::date_picker::DatePickerValue;

use time::{Date, Month, Weekday};

#[component]
pub fn Demo() -> Element {
    let v = DatePickerValue::new_day(None);
    let mut value = use_signal(|| v);

    let mut selected_date = use_signal(|| None::<Date>);

    rsx! {
        div {
            DatePicker {
                value: value(),
                selected_date: selected_date(),
                on_value_change: move |v| {
                    tracing::info!("Selected: {v}");
                    value.set(v);
                    selected_date.set(v.date());
                },
                on_format_placeholder: || tid!("YMD"),
                DatePickerInput {
                    DatePickerTrigger {
                        aria_label: "DatePicker Trigger",
                        DatePickerCalendar {
                            selected_date: selected_date(),
                            on_date_change: move |date| selected_date.set(date),
                            on_format_weekday: |weekday: Weekday| tid!(&weekday.to_string()),
                            on_format_month: |month: Month| tid!(&month.to_string()),
                        }
                    }
                }
            }
        }
    }
}
