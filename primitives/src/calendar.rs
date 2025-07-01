//! Defines the [`Calendar`] component and its sub-components, which provide a calendar interface with date selection and navigation.

use crate::use_unique_id;
use dioxus_lib::prelude::*;
use std::{
    fmt::{self, Display},
    rc::Rc,
};

/// A date in the calendar, representing a specific day, month, and year.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CalendarDate {
    /// The year of the date
    pub year: i32,
    /// The month of the date (1-12)
    pub month: u32,
    /// The day of the date (1-31)
    pub day: u32,
}

impl CalendarDate {
    /// Create a new CalendarDate with the specified year, month, and day.
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    /// Create a CalendarDate representing today's date. This is a placeholder implementation.
    pub(crate) fn today() -> Self {
        // In a real implementation, we would use chrono or time crate
        // For now, let's use a placeholder date
        Self {
            year: 2024,
            month: 5,
            day: 15,
        }
    }

    /// Get the number of days in the month
    pub fn days_in_month(&self) -> u32 {
        days_in_month(self.year, self.month)
    }

    /// Get the day of the week the current month starts on
    pub fn month_start_day_of_the_week(&self) -> u32 {
        day_of_the_week(self.year, self.month, 1)
    }

    /// Get the week number of the month (0 indexed)
    pub fn week(&self) -> u32 {
        let month_start_day = self.month_start_day_of_the_week();
        (self.day - 1 + month_start_day) / 7
    }

    /// Get the day within the current month that corresponds to the 0 indexed week and 0 indexed weekday
    pub fn day_for_position(&self, week: u32, weekday: u32) -> u32 {
        // The day should be the same week and day of the week as the current date
        let new_month_start_day = day_of_the_week(self.year, self.month, 1);
        let mut day = week.saturating_sub((weekday < new_month_start_day) as u32) * 7
            + (7 + weekday - new_month_start_day) % 7
            + 1;

        // Make sure the new day is within the bounds for the new month
        day -= (day.saturating_sub(self.days_in_month())).div_ceil(7) * 7;

        day
    }

    /// Get the previous month
    pub fn prev_month(&self) -> Self {
        let mut new = *self;

        if self.month == 1 {
            new.year -= 1;
            new.month = 12;
        } else {
            new.month -= 1;
        }

        new
    }

    /// Get the next month
    pub fn next_month(&self) -> Self {
        let mut new = *self;

        if self.month == 12 {
            new.year += 1;
            new.month = 1;
        } else {
            new.month += 1;
        }

        new
    }

    /// Check if this date is the same as another date
    pub fn is_same_day(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day
    }

    /// Check if this date is in the same month as another date
    pub fn is_same_month(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month
    }

    /// Get the next week's date
    pub fn next_week(&self) -> Self {
        let mut date = *self;
        let days_in_month = self.days_in_month();
        if date.day + 7 > days_in_month {
            let day_of_the_week = date.day_of_the_week();
            date = date.next_month();
            date.day = date.day_for_position(0, day_of_the_week);
        } else {
            date.day += 7;
        }
        date
    }

    /// Get the previous week's date
    pub fn prev_week(&self) -> Self {
        let mut date = *self;
        if date.day <= 7 {
            // Then move back to the previous month
            let day_of_the_week = date.day_of_the_week();
            date = date.prev_month();
            date.day = date.day_for_position(
                (date.days_in_month() + date.month_start_day_of_the_week()) / 7,
                day_of_the_week,
            )
        } else {
            date.day -= 7;
        }
        date
    }

    /// Get the next day's date
    pub fn next_day(&self) -> Self {
        let mut date = *self;
        if date.day < self.days_in_month() {
            date.day += 1;
        } else {
            date = date.next_month();
            date.day = 1; // Reset to the first day of the next month
        }
        date
    }

    /// Get the previous day's date
    pub fn prev_day(&self) -> Self {
        let mut date = *self;
        if date.day > 1 {
            date.day -= 1;
        } else {
            date = date.prev_month();
            date.day = date.days_in_month();
        }
        date
    }

    /// Get the day of the week (0 = Sunday, 6 = Saturday)
    pub fn day_of_the_week(&self) -> u32 {
        day_of_the_week(self.year, self.month, self.day)
    }

    /// Get a human-readable ARIA label for this date
    pub fn aria_label(&self) -> String {
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
        let day_names = [
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
        ];
        let day_of_week = self.day_of_the_week();
        format!(
            "{}, {} {}, {}",
            day_names[day_of_week as usize],
            month_names[(self.month - 1) as usize],
            self.day,
            self.year
        )
    }
}

#[test]
fn test_next_day() {
    let date = CalendarDate::new(2020, 1, 31);
    let next_date = date.next_day();
    assert_eq!(next_date.year, 2020);
    assert_eq!(next_date.month, 2);
    assert_eq!(next_date.day, 1);
    let prev_date = next_date.prev_day();
    assert_eq!(prev_date.year, 2020);
    assert_eq!(prev_date.month, 1);
    assert_eq!(prev_date.day, 31);

    let date = CalendarDate::new(2020, 12, 1);
    let prev_date = date.prev_day();
    assert_eq!(prev_date.year, 2020);
    assert_eq!(prev_date.month, 11);
    assert_eq!(prev_date.day, 30);
    let next_date = date.next_day();
    assert_eq!(next_date.year, 2020);
    assert_eq!(next_date.month, 12);
    assert_eq!(next_date.day, 2);
}

#[test]
fn test_next_month() {
    let date = CalendarDate::new(2025, 1, 1); // Wednesday
    let prev_month_date = date.prev_month();
    assert_eq!(prev_month_date.year, 2024);
    assert_eq!(prev_month_date.month, 12);
    assert_eq!(prev_month_date.day, 1);
    let next_month_date = date.next_month();
    assert_eq!(next_month_date.year, 2025);
    assert_eq!(next_month_date.month, 2);
    assert_eq!(next_month_date.day, 1);

    let date = CalendarDate::new(2025, 7, 31); // Thursday
    let next_month_date = date.next_month();
    assert_eq!(next_month_date.year, 2025);
    assert_eq!(next_month_date.month, 8);
    assert_eq!(next_month_date.day, 31);
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
fn day_of_the_week(year: i32, month: u32, day: u32) -> u32 {
    let month_offsets = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let year = if month < 3 { year - 1 } else { year };
    ((year + year / 4 - year / 100 + year / 400 + month_offsets[month as usize - 1] + day as i32)
        % 7) as _
}

#[test]
fn test_day_of_the_week() {
    assert_eq!(day_of_the_week(2025, 6, 5), 4); // Thursday
}

impl fmt::Display for CalendarDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

// Calendar context for child components
#[allow(dead_code)]
#[derive(Clone)]
struct CalendarContext {
    // State
    selected_date: ReadOnlySignal<Option<CalendarDate>>,
    set_selected_date: Callback<Option<CalendarDate>>,
    focused_date: Signal<Option<CalendarDate>>,
    view_date: ReadOnlySignal<CalendarDate>,
    set_view_date: Callback<CalendarDate>,

    // Configuration
    disabled: ReadOnlySignal<bool>,
    disabled_dates: ReadOnlySignal<Vec<CalendarDate>>,
    min_date: ReadOnlySignal<Option<CalendarDate>>,
    max_date: ReadOnlySignal<Option<CalendarDate>>,
    today: CalendarDate,

    // Accessibility
    calendar_id: ReadOnlySignal<String>,
}

/// The props for the [`Calendar`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CalendarProps {
    /// The selected date
    #[props(default)]
    pub selected_date: ReadOnlySignal<Option<CalendarDate>>,

    /// Callback when selected date changes
    #[props(default)]
    pub on_date_change: Callback<Option<CalendarDate>>,

    /// The month being viewed
    pub view_date: ReadOnlySignal<CalendarDate>,

    /// The current date (used for highlighting today)
    #[props(default = CalendarDate::today())]
    pub today: CalendarDate,

    /// Callback when view date changes
    #[props(default)]
    pub on_view_change: Callback<CalendarDate>,

    /// Whether the calendar is disabled
    #[props(default)]
    pub disabled: ReadOnlySignal<bool>,

    /// Dates that should be disabled/unselectable
    #[props(default = ReadOnlySignal::new(Signal::new(Vec::new())))]
    pub disabled_dates: ReadOnlySignal<Vec<CalendarDate>>,

    /// Minimum selectable date
    #[props(default = ReadOnlySignal::new(Signal::new(None)))]
    pub min_date: ReadOnlySignal<Option<CalendarDate>>,

    /// Maximum selectable date
    #[props(default = ReadOnlySignal::new(Signal::new(None)))]
    pub max_date: ReadOnlySignal<Option<CalendarDate>>,

    /// Optional ID for the calendar
    #[props(default)]
    pub id: Option<String>,

    /// First day of the week (1 = Monday, 7 = Sunday)
    #[props(default = 1)]
    pub first_day_of_week: u32,

    /// Additional attributes to extend the calendar element
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children of the calendar element
    children: Element,
}

/// # Calendar
///
/// The [`Calendar`] component provides an accessible calendar interface with arrow key navigation, month switching, and date selection.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::calendar::{
///     Calendar, CalendarDate, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<CalendarDate>);
///     let mut view_date = use_signal(|| CalendarDate::new(2025, 6, 5));
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: CalendarDate| {
///                 tracing::info!("View changed to: {}-{}", new_view.year, new_view.month);
///                 view_date.set(new_view);
///             },
///             CalendarHeader {
///                 CalendarNavigation {
///                     CalendarPreviousMonthButton {
///                         "<"
///                     }
///                     CalendarMonthTitle {}
///                     CalendarNextMonthButton {
///                         ">"
///                     }
///                 }
///             }
///             CalendarGrid {}
///         }
///     }
/// }
/// ```
///
/// # Styling
///
/// The [`Calendar`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the calendar is disabled. Possible values are `true` or `false`.
#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    // Generate a unique ID for the calendar
    let calendar_id = match props.id {
        Some(ref id) => use_signal(|| id.clone()),
        None => use_unique_id(),
    };

    // Create context provider for child components
    let mut ctx = use_context_provider(|| CalendarContext {
        selected_date: props.selected_date,
        set_selected_date: props.on_date_change,
        focused_date: Signal::new(props.selected_date.cloned()),
        view_date: props.view_date,
        set_view_date: props.on_view_change,
        disabled: props.disabled,
        disabled_dates: props.disabled_dates,
        min_date: props.min_date,
        max_date: props.max_date,
        calendar_id: calendar_id.into(),
        today: props.today,
    });

    rsx! {
        div {
            role: "application",
            "aria-label": "Calendar",
            id: props.id,
            "data-disabled": (props.disabled)(),
            onkeydown: move |e| {
                let Some(focused_date) = (ctx.focused_date)() else {
                    return;
                };
                let mut set_focused_date = |new_date: CalendarDate| {
                    // Make sure the view date month is the same as the focused date
                    let mut view_date = (ctx.view_date)();
                    if new_date.month != view_date.month {
                        view_date.month = new_date.month;
                        view_date.year = new_date.year;
                        (ctx.set_view_date)(view_date);
                    }
                    ctx.focused_date.set(Some(new_date));
                };
                match e.key() {
                    Key::ArrowLeft => {
                        e.prevent_default();
                        set_focused_date(focused_date.prev_day());
                    }
                    Key::ArrowRight => {
                        e.prevent_default();
                        set_focused_date(focused_date.next_day());
                    }
                    Key::ArrowUp => {
                        e.prevent_default();
                        if e.modifiers().shift() {
                            let day_of_the_week = focused_date.day_of_the_week();
                            let mut prev_month = focused_date.prev_month();
                            prev_month.day = prev_month.day_for_position((prev_month.days_in_month() + prev_month.month_start_day_of_the_week()) / 7, day_of_the_week);
                            set_focused_date(prev_month);
                        } else {
                            // Otherwise, move to the previous week
                            set_focused_date(focused_date.prev_week());
                        }
                    }
                    Key::ArrowDown => {
                        e.prevent_default();
                        if e.modifiers().shift() {
                            let day_of_the_week = focused_date.day_of_the_week();
                            let mut next_month = focused_date.next_month();
                            next_month.day = next_month.day_for_position(0, day_of_the_week);
                            set_focused_date(next_month);
                        } else {
                            set_focused_date(focused_date.next_week());
                        }
                    }
                    _ => {}
                }
            },
            ..props.attributes,

            {props.children}
        }
    }
}

/// The props for the [`CalendarHeader`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CalendarHeaderProps {
    /// Optional ID for the header
    #[props(default)]
    id: Option<String>,

    /// Additional attributes to extend the header element
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children of the header element
    children: Element,
}

/// # CalendarHeader
///
/// The [`CalendarHeader`] component displays the header for the calendar. It typically contains the [`CalendarNavigation`] component
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::calendar::{
///     Calendar, CalendarDate, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<CalendarDate>);
///     let mut view_date = use_signal(|| CalendarDate::new(2025, 6, 5));
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: CalendarDate| {
///                 tracing::info!("View changed to: {}-{}", new_view.year, new_view.month);
///                 view_date.set(new_view);
///             },
///             CalendarHeader {
///                 CalendarNavigation {
///                     CalendarPreviousMonthButton {
///                         "<"
///                     }
///                     CalendarMonthTitle {}
///                     CalendarNextMonthButton {
///                         ">"
///                     }
///                 }
///             }
///             CalendarGrid {}
///         }
///     }
/// }
/// ```
#[component]
pub fn CalendarHeader(props: CalendarHeaderProps) -> Element {
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

/// The props for the [`CalendarNavigation`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CalendarNavigationProps {
    /// Optional ID for the navigation
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children of the navigation element
    #[props(default)]
    children: Element,
}

/// # CalendarNavigation
///
/// The [`CalendarNavigation`] component provides a container for navigation buttons in the calendar header.
/// It typically contains the [`CalendarPreviousMonthButton`], [`CalendarNextMonthButton`], and [`CalendarMonthTitle`] components.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::calendar::{
///     Calendar, CalendarDate, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<CalendarDate>);
///     let mut view_date = use_signal(|| CalendarDate::new(2025, 6, 5));
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: CalendarDate| {
///                 tracing::info!("View changed to: {}-{}", new_view.year, new_view.month);
///                 view_date.set(new_view);
///             },
///             CalendarHeader {
///                 CalendarNavigation {
///                     CalendarPreviousMonthButton {
///                         "<"
///                     }
///                     CalendarMonthTitle {}
///                     CalendarNextMonthButton {
///                         ">"
///                     }
///                 }
///             }
///             CalendarGrid {}
///         }
///     }
/// }
/// ```
#[component]
pub fn CalendarNavigation(props: CalendarNavigationProps) -> Element {
    rsx! {
        div { class: "calendar-navigation", ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`CalendarPreviousMonthButton`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CalendarPreviousMonthButtonProps {
    /// Additional attributes to apply to the button
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children of the button element
    children: Element,
}

/// # CalendarPreviousMonthButton
///
/// The [`CalendarPreviousMonthButton`] component provides a button to navigate to the previous month in the calendar.
///
/// This must be used inside a [`Calendar`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::calendar::{
///     Calendar, CalendarDate, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<CalendarDate>);
///     let mut view_date = use_signal(|| CalendarDate::new(2025, 6, 5));
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: CalendarDate| {
///                 tracing::info!("View changed to: {}-{}", new_view.year, new_view.month);
///                 view_date.set(new_view);
///             },
///             CalendarHeader {
///                 CalendarNavigation {
///                     CalendarPreviousMonthButton {
///                         "<"
///                     }
///                     CalendarMonthTitle {}
///                     CalendarNextMonthButton {
///                         ">"
///                     }
///                 }
///             }
///             CalendarGrid {}
///         }
///     }
/// }
/// ```
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

/// The props for the [`CalendarNextMonthButton`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CalendarNextMonthButtonProps {
    /// Additional attributes to apply to the button
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children of the button element
    children: Element,
}

/// # CalendarNextMonthButton
///
/// The [`CalendarNextMonthButton`] component provides a button to navigate to the next month in the calendar.
///
/// This must be used inside a [`Calendar`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::calendar::{
///     Calendar, CalendarDate, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<CalendarDate>);
///     let mut view_date = use_signal(|| CalendarDate::new(2025, 6, 5));
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: CalendarDate| {
///                 tracing::info!("View changed to: {}-{}", new_view.year, new_view.month);
///                 view_date.set(new_view);
///             },
///             CalendarHeader {
///                 CalendarNavigation {
///                     CalendarPreviousMonthButton {
///                         "<"
///                     }
///                     CalendarMonthTitle {}
///                     CalendarNextMonthButton {
///                         ">"
///                     }
///                 }
///             }
///             CalendarGrid {}
///         }
///     }
/// }
/// ```
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

/// The props for the [`CalendarMonthTitle`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CalendarMonthTitleProps {
    /// Additional attributes to apply to the title element
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

/// # CalendarMonthTitle
///
/// The [`CalendarMonthTitle`] component displays the title of the current month in the calendar. It will contain
/// the month and year information as text in the children.
///
/// This must be used inside a [`Calendar`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::calendar::{
///     Calendar, CalendarDate, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<CalendarDate>);
///     let mut view_date = use_signal(|| CalendarDate::new(2025, 6, 5));
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: CalendarDate| {
///                 tracing::info!("View changed to: {}-{}", new_view.year, new_view.month);
///                 view_date.set(new_view);
///             },
///             CalendarHeader {
///                 CalendarNavigation {
///                     CalendarPreviousMonthButton {
///                         "<"
///                     }
///                     CalendarMonthTitle {}
///                     CalendarNextMonthButton {
///                         ">"
///                     }
///                 }
///             }
///             CalendarGrid {}
///         }
///     }
/// }
/// ```
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

/// The props for the [`CalendarGrid`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CalendarGridProps {
    /// Optional ID for the grid
    #[props(default)]
    pub id: Option<String>,

    /// Whether to show week numbers
    #[props(default)]
    pub show_week_numbers: bool,

    /// Day labels (Sun, Mon, etc.)
    #[props(default = vec!["Su".to_string(), "Mo".to_string(), "Tu".to_string(), "We".to_string(), "Th".to_string(), "Fr".to_string(), "Sa".to_string()])]
    pub day_labels: Vec<String>,

    /// The callback that will be used to render each day in the grid
    #[props(default = Callback::new(|date: CalendarDate| {
        rsx! { CalendarDay { date } }
    }))]
    pub render_day: Callback<CalendarDate, Element>,

    /// Additional attributes to apply to the grid element
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

/// # CalendarGrid
///
/// The [`CalendarGrid`] component displays the grid of days for the current month in the calendar.
///
/// This must be used inside a [`Calendar`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::calendar::{
///     Calendar, CalendarDate, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<CalendarDate>);
///     let mut view_date = use_signal(|| CalendarDate::new(2025, 6, 5));
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: CalendarDate| {
///                 tracing::info!("View changed to: {}-{}", new_view.year, new_view.month);
///                 view_date.set(new_view);
///             },
///             CalendarHeader {
///                 CalendarNavigation {
///                     CalendarPreviousMonthButton {
///                         "<"
///                     }
///                     CalendarMonthTitle {}
///                     CalendarNextMonthButton {
///                         ">"
///                     }
///                 }
///             }
///             CalendarGrid {}
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`CalendarGrid`] component renders days in a grid that can be styled using CSS. They define the following data attributes:
/// - `data-today`: If the date is today. Possible values are `true` or `false`
/// - `data-selected`: If the date is selected. Possible values are `true` or `false`
/// - `data-month`: The relative month of the date. Possible values are `last`, `current`, or `next`
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

        let first_day_offset = view_date.month_start_day_of_the_week();

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
                grid.push(CalendarDate::new(
                    next_month.year,
                    next_month.month,
                    day as _,
                ));
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
            thead { aria_hidden: "true",
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
                        role: "row",
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
    let mut ctx: CalendarContext = use_context();
    let view_date = (ctx.view_date)();
    let day = date.day;
    let month = match date.month.cmp(&view_date.month) {
        std::cmp::Ordering::Less => RelativeMonth::Last,
        std::cmp::Ordering::Equal => RelativeMonth::Current,
        std::cmp::Ordering::Greater => RelativeMonth::Next,
    };
    let in_current_month = month == RelativeMonth::Current;
    let is_selected = move || (ctx.selected_date)().is_some_and(|d| d == date);
    let is_focused = move || (ctx.focused_date)().is_some_and(|d| d == date);
    let is_today = date == ctx.today;

    // Handle day selection
    let mut handle_day_select = move |day: u32| {
        if !(ctx.disabled)() {
            let view_date = (ctx.view_date)();
            let date = CalendarDate::new(view_date.year, view_date.month, day);
            ctx.set_selected_date.call((!is_selected()).then_some(date));
            ctx.focused_date.set(Some(date));
        }
    };

    let mut day_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    use_effect(move || {
        if let Some(day) = day_ref() {
            if is_focused() {
                spawn(async move {
                    _ = day.set_focus(true).await;
                });
            }
        }
    });

    let focusable_date = (ctx.focused_date)()
        .or_else(&*ctx.selected_date)
        .unwrap_or_else(&*ctx.view_date);

    rsx! {
        button {
            class: "calendar-grid-cell",
            r#type: "button",
            tabindex: if date == focusable_date {
                "0"
            } else {
                "-1"
            },
            aria_label: props.date.aria_label(),
            "data-today": is_today,
            "data-selected": is_selected(),
            "data-month": "{month}",
            onclick: move |e| {
                e.prevent_default();
                if in_current_month {
                    handle_day_select(day);
                }
            },
            onfocus: move |_| {
                if in_current_month {
                    ctx.focused_date.set(Some(date));
                }
            },
            onmounted: move |e| day_ref.set(Some(e.data())),
            ..attributes,
            {day.to_string()}
        }
    }
}
