The DatePicker component is used to display a date input and a Calendar popover, allowing users to enter or select a date value.

## Component Structure

```rust
DatePicker {
    // The currently selected date in the date picker (if any).
    selected_date,
    on_value_change: move |v: Option<Date>| {
        // This callback is triggered when a date is selected in the
        // calendar or the user entered it from the keyboard.
        // The date parameter contains the selected date.
    },

    // The DatePickerPopover is the root popover component that wraps the input and calendar.
    DatePickerPopover {
        // Allows the user to enter a date using the keyboard.
        // The input field includes a button to display the calendar.
        DatePickerInput {
            // Optional placeholder formatters for the input fields
            on_format_day_placeholder: || "D",
            on_format_month_placeholder: || "M",
            on_format_year_placeholder: || "Y",

            // The DatePickerPopoverTrigger contains the button that triggers the popover
            // to display the Calendar when clicked.
            DatePickerPopoverTrigger {}

            // The DatePickerPopoverContent contains the Calendar that will be displayed when
            // the user clicks on the trigger.
            DatePickerPopoverContent {
                // The alignment of the DatePickerPopoverContent relative to the DatePickerPopoverTrigger.
                // Can be one of Start, Center, or End.
                align: ContentAlign::Center,

                // The DatePickerCalendar component wraps the Calendar with its navigation components
                DatePickerCalendar {
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
```