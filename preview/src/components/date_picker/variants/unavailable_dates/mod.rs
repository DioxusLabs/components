use super::super::component::*;
use dioxus::prelude::*;

use dioxus_primitives::{calendar::DateRange, utc_now};
use time::{ext::NumericalDuration, UtcDateTime};

#[component]
pub fn Demo() -> Element {
    let mut selected_range = use_signal(|| None::<DateRange>);

    let now = utc_now().date();
    let disabled_ranges = use_signal(|| {
        vec![
            DateRange::new(now, now.saturating_add(3.days())),
            DateRange::new(now.saturating_add(15.days()), now.saturating_add(18.days())),
            DateRange::new(now.saturating_add(22.days()), now.saturating_add(23.days())),
        ]
    });

    rsx! {
        div {
            DateRangePicker {
                selected_range: selected_range(),
                on_range_change: move |range| {
                    tracing::info!("Selected range: {:?}", range);
                    selected_range.set(range);
                },
                disabled_ranges: disabled_ranges,
                DateRangePickerInput {}
            }
        }
    }
}
