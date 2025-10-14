The DatePicker component is used to display a date input and a Calendar popover, allowing users to enter or select a date value.

## Component Structure

```rust
DatePicker {
    // The currently value in the date picker (type DatePickerValue).
    value,
    // The currently selected date in the date picker (if any).
    selected_date,
    on_value_change: move |v: DatePickerValue| {
        // This callback is triggered when a date is selected in the
        // calendar or the user entered it from the keyboard.
        // The date parameter contains the selected date.
    },
    // Allows the user to enter a date using the keyboard. 
    // The input field should contain a button to display the calendar and the calendar itself.
    DatePickerInput {
        // The DatePickerPopover is the root popover component that contains the trigger and Calendar.
        DatePickerPopover {
            // The DatePickerPopoverTrigger contains the elements that will trigger the popover 
            // to display Calendar when clicked.
            DatePickerPopoverTrigger {}
            // The DatePickerPopoverContent contains the Calendar that will be displayed when
            // the user clicks on the trigger.
            DatePickerPopoverContent {
                // The alignment of the DatePickerPopoverContent relative to the DatePickerPopoverTrigger. 
                // Can be one of Start, Center, or End. Recommended use End for default value.
                align: ContentAlign::End,
                // Customized Calendar components
                DatePickerCalendar {}
            }
        }
    }
}
```