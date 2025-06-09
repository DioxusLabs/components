use crate::use_unique_id;
use dioxus_lib::prelude::*;
use std::fmt::{self, Display};

// Calendar date representation
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CalendarDate {
    pub year: i32,
    pub month: u32, // 1-12
    pub day: u32,   // 1-31
}

impl CalendarDate {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    pub fn today() -> Self {
        // In a real implementation, we would use chrono or time crate
        // For now, let's use a placeholder date
        Self {
            year: 2024,
            month: 5,
            day: 15,
        }
    }

    pub fn format(&self, _format: &str) -> String {
        // Simple formatting - in a real implementation we would use a date library
        format!("{}-{:02}-{:02}", self.year, self.month, self.day)
    }

    // Get the first day of the month (1-based, Monday = 1, Sunday = 7)
    pub fn first_day_of_month(&self) -> u32 {
        // This is a simplified implementation
        // In a real implementation, we would use chrono or time crate
        ((self.day + 6) % 7) + 1
    }

    // Get the number of days in the month
    pub fn days_in_month(&self) -> u32 {
        days_in_month(self.year, self.month)
    }

    // Get the previous month
    pub fn prev_month(&self) -> Self {
        if self.month == 1 {
            Self {
                year: self.year - 1,
                month: 12,
                day: 1,
            }
        } else {
            Self {
                year: self.year,
                month: self.month - 1,
                day: 1,
            }
        }
    }

    // Get the next month
    pub fn next_month(&self) -> Self {
        if self.month == 12 {
            Self {
                year: self.year + 1,
                month: 1,
                day: 1,
            }
        } else {
            Self {
                year: self.year,
                month: self.month + 1,
                day: 1,
            }
        }
    }

    // Check if this date is the same as another date
    pub fn is_same_day(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day
    }

    // Check if this date is in the same month as another date
    pub fn is_same_month(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            // Leap year check (simplified)
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => unreachable!(), // Invalid month
    }
}

// Zeller's Congruence
fn month_start_day_of_week(year: i32, month: u32) -> u32 {
    let month_offsets = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let year = if month < 3 { year - 1 } else { year };
    ((year + year / 4 - year / 100 + year / 400 + month_offsets[month as usize - 1] + 1) % 7) as _
}

impl fmt::Display for CalendarDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

// Calendar view mode
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CalendarMode {
    Day,
    Month,
    Year,
}

// Calendar context for child components
#[allow(dead_code)]
#[derive(Clone)]
struct CalendarContext {
    // State
    selected_date: ReadOnlySignal<Option<CalendarDate>>,
    set_selected_date: Callback<Option<CalendarDate>>,
    view_date: ReadOnlySignal<CalendarDate>,
    set_view_date: Callback<CalendarDate>,
    mode: ReadOnlySignal<CalendarMode>,
    set_mode: Callback<CalendarMode>,

    // Configuration
    disabled: ReadOnlySignal<bool>,
    disabled_dates: ReadOnlySignal<Vec<CalendarDate>>,
    min_date: ReadOnlySignal<Option<CalendarDate>>,
    max_date: ReadOnlySignal<Option<CalendarDate>>,

    // Accessibility
    calendar_id: ReadOnlySignal<String>,
}

// Main Calendar component props
#[derive(Props, Clone, PartialEq)]
pub struct CalendarProps {
    /// The selected date
    #[props(default)]
    selected_date: ReadOnlySignal<Option<CalendarDate>>,

    /// Callback when selected date changes
    #[props(default)]
    on_date_change: Callback<Option<CalendarDate>>,

    /// The month being viewed
    view_date: ReadOnlySignal<CalendarDate>,

    /// Callback when view date changes
    #[props(default)]
    on_view_change: Callback<CalendarDate>,

    /// The calendar mode (day, month, year)
    #[props(default = CalendarMode::Month)]
    mode: CalendarMode,

    /// Callback when mode changes
    #[props(default)]
    on_mode_change: Callback<CalendarMode>,

    /// Whether the calendar is disabled
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    /// Dates that should be disabled/unselectable
    #[props(default = ReadOnlySignal::new(Signal::new(Vec::new())))]
    disabled_dates: ReadOnlySignal<Vec<CalendarDate>>,

    /// Minimum selectable date
    #[props(default = ReadOnlySignal::new(Signal::new(None)))]
    min_date: ReadOnlySignal<Option<CalendarDate>>,

    /// Maximum selectable date
    #[props(default = ReadOnlySignal::new(Signal::new(None)))]
    max_date: ReadOnlySignal<Option<CalendarDate>>,

    /// Optional ID for the calendar
    #[props(default)]
    id: Option<String>,

    /// First day of the week (1 = Monday, 7 = Sunday)
    #[props(default = 1)]
    first_day_of_week: u32,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// Child components
    children: Element,
}

// Main Calendar component
#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    // State for calendar mode
    let mut mode = use_signal(|| props.mode);
    let set_mode = use_callback(move |new_mode: CalendarMode| {
        mode.set(new_mode);
        props.on_mode_change.call(new_mode);
    });

    // Generate a unique ID for the calendar
    let calendar_id = match props.id {
        Some(ref id) => use_signal(|| id.clone()),
        None => use_unique_id(),
    };

    // Create context provider for child components
    let _ctx = use_context_provider(|| CalendarContext {
        selected_date: props.selected_date,
        set_selected_date: props.on_date_change,
        view_date: props.view_date,
        set_view_date: props.on_view_change,
        mode: mode.into(),
        set_mode,
        disabled: props.disabled,
        disabled_dates: props.disabled_dates,
        min_date: props.min_date,
        max_date: props.max_date,
        calendar_id: calendar_id.into(),
    });

    rsx! {
        div {
            role: "application",
            "aria-label": "Calendar",
            id: props.id,
            "data-disabled": (props.disabled)(),
            ..props.attributes,

            {props.children}
        }
    }
}

// Calendar Header component props
#[derive(Props, Clone, PartialEq)]
pub struct CalendarHeaderProps {
    /// Optional ID for the header
    #[props(default)]
    id: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// Child components
    children: Element,
}

// Calendar Header component
#[component]
pub fn CalendarHeader(props: CalendarHeaderProps) -> Element {
    let _ctx: CalendarContext = use_context();

    rsx! {
        div {
            role: "heading",
            "aria-level": "2",
            id: props.id,
            ..props.attributes,

            {props.children}
        }
    }
}

// Calendar Navigation component props
#[derive(Props, Clone, PartialEq)]
pub struct CalendarNavigationProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// Child components (optional)
    #[props(default)]
    children: Element,
}

// Calendar Navigation component
#[component]
pub fn CalendarNavigation(props: CalendarNavigationProps) -> Element {
    rsx! {
        div { class: "calendar-navigation", ..props.attributes,

            {props.children}
        }
    }
}

/// Next month navigation button component props
#[derive(Props, Clone, PartialEq)]
pub struct CalendarPreviousMonthButtonProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

/// Next month navigation button component
#[component]
pub fn CalendarPreviousMonthButton(props: CalendarPreviousMonthButtonProps) -> Element {
    let ctx: CalendarContext = use_context();
    // Handle navigation to previous month
    let handle_prev_month = move |e: Event<MouseData>| {
        e.prevent_default();
        let current_view = (ctx.view_date)();
        ctx.set_view_date.call(current_view.prev_month());
    };

    rsx! {
        button {
            class: "calendar-nav-prev",
            aria_label: "Previous month",
            r#type: "button",
            onclick: handle_prev_month,
            disabled: (ctx.disabled)(),
            ..props.attributes,

            {props.children}
        }
    }
}

/// Next month navigation button component props
#[derive(Props, Clone, PartialEq)]
pub struct CalendarNextMonthButtonProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

/// Next month navigation button component
#[component]
pub fn CalendarNextMonthButton(props: CalendarNextMonthButtonProps) -> Element {
    let ctx: CalendarContext = use_context();
    // Handle navigation to next month
    let handle_next_month = move |e: Event<MouseData>| {
        e.prevent_default();
        let current_view = (ctx.view_date)();
        ctx.set_view_date.call(current_view.next_month());
    };

    rsx! {
        button {
            class: "calendar-nav-next",
            aria_label: "Next month",
            r#type: "button",
            onclick: handle_next_month,
            disabled: (ctx.disabled)(),
            ..props.attributes,

            {props.children}
        }
    }
}

/// Calendar Month Title component props
#[derive(Props, Clone, PartialEq)]
pub struct CalendarMonthTitleProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

/// Calendar Month Title component
#[component]
pub fn CalendarMonthTitle(props: CalendarMonthTitleProps) -> Element {
    let ctx: CalendarContext = use_context();
    // Format the current month and year
    let month_year = use_memo(move || {
        let view_date = (ctx.view_date)();
        let month_names = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];
        let month_name = month_names[(view_date.month - 1) as usize];
        format!("{} {}", month_name, view_date.year)
    });

    rsx! {
        div {
            class: "calendar-month-title",
            ..props.attributes,

            {month_year}
        }
    }
}

// Calendar Grid component props
#[derive(Props, Clone, PartialEq)]
pub struct CalendarGridProps {
    /// Optional ID for the grid
    #[props(default)]
    id: Option<String>,

    /// Whether to show week numbers
    #[props(default)]
    show_week_numbers: bool,

    /// Day labels (Sun, Mon, etc.)
    #[props(default = vec!["Su".to_string(), "Mo".to_string(), "Tu".to_string(), "We".to_string(), "Th".to_string(), "Fr".to_string(), "Sa".to_string()])]
    day_labels: Vec<String>,

    /// The callback that will be used to render each day in the grid
    #[props(default = Callback::new(|date: CalendarDate| {
        rsx! { CalendarDay { date } }
    }))]
    render_day: Callback<CalendarDate, Element>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

// Calendar Grid component
#[component]
pub fn CalendarGrid(props: CalendarGridProps) -> Element {
    let ctx: CalendarContext = use_context();

    // We'll use the view_date from context in the memo below

    // Generate a grid of days with proper layout
    // Use the view_date as a dependency to ensure the grid updates when the view changes
    let days_grid = use_memo(move || {
        // Get the current view date from context
        let view_date = (ctx.view_date)();
        let days_in_month = view_date.days_in_month();

        let first_day_offset = month_start_day_of_week(view_date.year, view_date.month);

        // Create a grid with empty cells for padding and actual days
        let mut grid = Vec::new();

        // Add empty cells for days before the first day of the month
        let previous_month = view_date.prev_month();
        for i in 0..first_day_offset {
            let day = (previous_month.days_in_month() + i + 1 - first_day_offset) as u32;
            grid.push(CalendarDate::new(
                previous_month.year,
                previous_month.month,
                day,
            ));
        }

        // Add days of the month
        for day in 1..=days_in_month {
            grid.push(CalendarDate::new(view_date.year, view_date.month, day));
        }

        // Add empty cells to complete the grid (for a clean layout)
        let remainder = grid.len() % 7;
        let next_month = view_date.next_month();
        if remainder > 0 {
            for day in 1..=(7 - remainder) {
                grid.push(CalendarDate::new(next_month.year, next_month.month, day as _));
            }
        }

        // Turn the flat grid into a 2D grid (7 columns)
        grid.chunks(7)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>()
    });

    rsx! {
        table {
            role: "grid",
            id: props.id,
            class: "calendar-grid",
            ..props.attributes,

            // Day headers
            thead { role: "row",
                tr {
                    class: "calendar-grid-header",
                    // Day name headers
                    for day_label in &props.day_labels {
                        th {
                            class: "calendar-grid-day-header",
                            {day_label.clone()}
                        }
                    }
                }
            }

            // Calendar days grid
            tbody { class: "calendar-grid-body",
                // Display all days in a grid
                for row in &*days_grid.read() {
                    tr {
                        class: "calendar-grid-week",
                        for date in row.iter().copied() {
                            td {
                                {props.render_day.call(date)}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum RelativeMonth {
    Last,
    Current,
    Next,
}

impl Display for RelativeMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelativeMonth::Last => write!(f, "last"),
            RelativeMonth::Current => write!(f, "current"),
            RelativeMonth::Next => write!(f, "next"),
        }
    }
}

#[derive(Props, Clone, Debug, PartialEq)]
struct CalendarDayProps {
    date: CalendarDate,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
fn CalendarDay(props: CalendarDayProps) -> Element {
    let CalendarDayProps { date, attributes } = props;
    let ctx: CalendarContext = use_context();
    let view_date = (ctx.view_date)();
    let day = date.day;
    let month = match date.month.cmp(&view_date.month) {
        std::cmp::Ordering::Less => RelativeMonth::Last,
        std::cmp::Ordering::Equal => RelativeMonth::Current,
        std::cmp::Ordering::Greater => RelativeMonth::Next,
    };
    let in_current_month = month == RelativeMonth::Current;
    let is_selected = (ctx.selected_date)()
        .is_some_and(|d| d.day == day && d.month == view_date.month && d.year == view_date.year);
    let is_today = date == view_date;

    // Handle day selection
    let handle_day_select = move |day: u32| {
        if !(ctx.disabled)() {
            let view_date = (ctx.view_date)();
            let date = CalendarDate::new(view_date.year, view_date.month, day);
            ctx.set_selected_date.call(Some(date));
        }
    };

    rsx! {
        button {
            class: "calendar-grid-cell",
            onclick: move |e| {
                e.prevent_default();
                if in_current_month {
                    handle_day_select(day);
                }
            },
            r#type: "button",
            tabindex: (!in_current_month).then_some("-1"),
            "data-today": is_today,
            "data-selected": is_selected,
            "data-month": "{month}",
            ..attributes,
            {day.to_string()}
        }
    }
}

// Calendar Cell component props
#[derive(Props, Clone, PartialEq)]
pub struct CalendarCellProps {
    /// The date for this cell
    date: CalendarDate,

    /// Whether this date is selected
    #[props(default)]
    is_selected: bool,

    /// Whether this date is today
    #[props(default)]
    is_today: bool,

    /// Whether this date is disabled
    #[props(default)]
    is_disabled: bool,

    /// Click handler
    #[props(default)]
    onclick: EventHandler<MouseEvent>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

// Calendar Cell component
#[component]
pub fn CalendarCell(props: CalendarCellProps) -> Element {
    let _ctx: CalendarContext = use_context();

    // Determine cell state classes
    let state_class = if props.is_selected {
        "calendar-grid-cell-selected"
    } else if props.is_today {
        "calendar-grid-cell-today"
    } else {
        ""
    };

    rsx! {
        button {
            role: "gridcell",
            class: "calendar-grid-cell {state_class}",
            "aria-selected": props.is_selected,
            "aria-disabled": props.is_disabled,
            r#type: "button",
            disabled: props.is_disabled,
            "data-selected": props.is_selected,
            "data-today": props.is_today,
            "data-disabled": props.is_disabled,
            tabindex: if props.is_selected { "0" } else { "-1" },
            onclick: props.onclick,
            ..props.attributes,

            {props.date.day.to_string()}
        }
    }
}
