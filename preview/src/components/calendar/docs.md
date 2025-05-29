The Calendar component is used to display a calendar interface, allowing users to select dates. It provides a grid layout of days for a specific month and year.

## Component Structure

```rust
Calendar {
    // The currently selected date in the calendar (if any).
    selected_date,
    on_date_change: |date: Option<CalendarDate>| {
        // This callback is triggered when a date is selected in the calendar.
        // The date parameter contains the selected date.
    },
    // The current view date of the calendar, which determines the month and year displayed.
    view_date,
    on_view_change: |date: CalendarDate| {
        // This callback is triggered when the view date changes.
        // The date parameter contains the new view date.
    },
    // The calendar header should contain the navigation controls and the title for the calendar.
    CalendarHeader {
        // The calendar navigation handles switching between months and years within the calendar view.
        CalendarNavigation {} 
    }
    // The calendar grid displays the days of the month in a grid layout.
    CalendarGrid {}
}
```
