//! Defines the [`Calendar`] component and its sub-components, which provide a calendar interface with date selection and navigation.

use dioxus::prelude::*;
use std::{
    fmt::{self, Display},
    rc::Rc,
};

use time::{ext::NumericalDuration, macros::date, Date, Month, UtcDateTime, Weekday};

// A collection of [`Weekday`]s stored as a single byte
// Implemented as a bitmask where bits 1-7 correspond to Monday-Sunday
#[derive(Clone, Copy)]
struct WeekdaySet(u8); // the 8-th bit is always 0

impl WeekdaySet {
    // Get the first day in the collection, starting from Monday
    // Returns `None` if the collection is empty
    const fn first(self) -> Option<Weekday> {
        if self.is_empty() {
            return None;
        }

        // Find the first non-zero bit
        Some(Weekday::Monday.nth_next(self.0.trailing_zeros() as u8))
    }

    // Create a `WeekdaySet` from a single [`Weekday`]
    const fn single(weekday: Weekday) -> Self {
        Self(1 << weekday.number_days_from_monday())
    }

    // Iterate over the [`Weekday`]s in the collection starting from a given day
    // Wraps around from Sunday to Monday if necessary
    const fn iter(self, start: Weekday) -> WeekdaySetIter {
        WeekdaySetIter { days: self, start }
    }

    // Returns `true` if the collection is empty
    const fn is_empty(self) -> bool {
        self.0 == 0
    }

    // Split the collection in two at the given day. Returns a tuple `(before, after)`
    // `before` contains all days starting from Monday up to but NOT including `weekday`
    // `after` contains all days starting from `weekday` up to and including Sunday
    const fn split_at(self, weekday: Weekday) -> (Self, Self) {
        let days_after = 0b1000_0000 - Self::single(weekday).0;
        let days_before = days_after ^ 0b0111_1111;
        (Self(self.0 & days_before), Self(self.0 & days_after))
    }

    // Returns `true` if the collection contains the given day
    const fn contains(self, day: Weekday) -> bool {
        self.0 & Self::single(day).0 != 0
    }

    // Removes a day from the collection
    // Returns `true` if the collection did contain the day
    fn remove(&mut self, day: Weekday) -> bool {
        if self.contains(day) {
            self.0 &= !Self::single(day).0;
            return true;
        }

        false
    }
}

// An iterator over a collection of weekdays, starting from a given day
struct WeekdaySetIter {
    days: WeekdaySet,
    start: Weekday,
}

impl Iterator for WeekdaySetIter {
    type Item = Weekday;

    fn next(&mut self) -> Option<Self::Item> {
        if self.days.is_empty() {
            return None;
        }

        let (before, after) = self.days.split_at(self.start);
        let days = if after.is_empty() { before } else { after };

        let next = days.first().expect("the collection is not empty");
        self.days.remove(next);
        Some(next)
    }
}

pub(crate) fn weekday_abbreviation(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Monday => "Mo",
        Weekday::Tuesday => "Tu",
        Weekday::Wednesday => "We",
        Weekday::Thursday => "Th",
        Weekday::Friday => "Fr",
        Weekday::Saturday => "Sa",
        Weekday::Sunday => "Su",
    }
}

// The number of days since the first weekday of current date
fn days_since(date: Date, weekday: Weekday) -> i64 {
    let lhs = date.replace_day(1).unwrap().weekday() as i64;
    let rhs = weekday as i64;
    if lhs < rhs {
        7 + lhs - rhs
    } else {
        lhs - rhs
    }
}

fn next_month(date: Date) -> Option<Date> {
    let next_month = date.month().next();
    let last_day = next_month.length(date.year());
    // Clamp the day to the length of the next month
    let current_day = date.day();
    let new_day = current_day.min(last_day);
    Date::from_calendar_date(
        date.year() + if next_month == Month::January { 1 } else { 0 },
        next_month,
        new_day,
    )
    .ok()
}

fn previous_month(date: Date) -> Option<Date> {
    let previous_month = date.month().previous();
    let last_day = previous_month.length(date.year());
    // Clamp the day to the length of the previous month
    let current_day = date.day();
    let new_day = current_day.min(last_day);
    Date::from_calendar_date(
        date.year()
            + if previous_month == Month::December {
                -1
            } else {
                0
            },
        previous_month,
        new_day,
    )
    .ok()
}

fn replace_month(date: Date, month: Month) -> Date {
    let year = date.year();
    let num_days = month.length(year);
    Date::from_calendar_date(year, month, std::cmp::min(date.day(), num_days))
        .expect("invalid or out-of-range date")
}

/// Calendar date range
#[derive(Copy, Clone, Debug)]
pub struct DateRange {
    /// The start date of the range
    pub start: Date,
    /// The end date of the range
    pub end: Date,
}

impl DateRange {
    /// Create a new date range
    pub fn new(start: Date, end: Date) -> Self {
        if start <= end {
            Self { start, end }
        } else {
            Self {
                start: end,
                end: start,
            }
        }
    }

    fn contains(&self, date: Date) -> bool {
        self.start <= date && date <= self.end
    }

    fn contained_in_interval(&self, date: Date) -> bool {
        self.start < date && date < self.end
    }

    fn clamp(&self, date: Date) -> Date {
        date.clamp(self.start, self.end)
    }
}

impl Display for DateRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}

/// The base context provided by the [`Calendar`] and the [`RangeCalendar`] component to its children.
#[derive(Copy, Clone)]
pub struct BaseCalendarContext {
    // State
    focused_date: Signal<Option<Date>>,
    view_date: ReadSignal<Date>,
    available_range: Signal<Option<DateRange>>,
    set_view_date: Callback<Date>,
    format_weekday: Callback<Weekday, String>,
    format_month: Callback<Month, String>,

    // Configuration
    disabled: ReadSignal<bool>,
    today: Date,
    first_day_of_week: Weekday,
    min_date: Date,
    max_date: Date,
    disabled_ranges: ReadSignal<Vec<DateRange>>,
}

impl BaseCalendarContext {
    /// Get the currently focused date
    pub fn focused_date(&self) -> Option<Date> {
        self.focused_date.cloned()
    }

    /// Set the focused date
    pub fn set_focused_date(&mut self, date: Option<Date>) {
        self.focused_date.set(date);
    }

    /// Get the current view date
    pub fn view_date(&self) -> Date {
        self.view_date.cloned()
    }

    /// Set the view date
    pub fn set_view_date(&self, date: Date) {
        (self.set_view_date)(date.clamp(self.min_date, self.max_date));
    }

    /// Check if the calendar is disabled
    pub fn is_disabled(&self) -> bool {
        self.disabled.cloned()
    }

    /// Check if the selected date is unavailable
    pub fn is_unavailable(&self, date: Date) -> bool {
        for range in (self.disabled_ranges)() {
            if range.contains(date) {
                return true;
            }
        }
        false
    }

    /// Check if a date is focused
    pub fn is_focused(&self, date: Date) -> bool {
        self.focused_date().is_some_and(|d| d == date)
    }

    /// Return available date range by given date
    pub fn available_range(&self, anchor_date: Option<Date>) -> Option<DateRange> {
        let date = anchor_date?;

        let ranges = (self.disabled_ranges)();

        for (index, range) in ranges.iter().enumerate() {
            if range.end < date {
                continue;
            }

            let start = if index == 0 {
                self.min_date
            } else {
                let date = ranges[index - 1].end;
                date.next_day().unwrap_or(date)
            };
            let end = range.start.previous_day().unwrap_or(range.start);

            return Some(DateRange::new(start, end));
        }

        ranges
            .last()
            .map(|r| DateRange::new(r.end.next_day().unwrap_or(r.end), self.max_date))
    }
}

/// The context provided by the [`Calendar`] component to its children.
#[derive(Copy, Clone)]
pub struct CalendarContext {
    selected_date: ReadSignal<Option<Date>>,
    set_selected_date: Callback<Option<Date>>,
}

impl CalendarContext {
    /// Get the currently selected date
    pub fn selected_date(&self) -> Option<Date> {
        self.selected_date.cloned()
    }

    /// Set the selected date
    pub fn set_selected_date(&self, date: Option<Date>) {
        (self.set_selected_date)(date);
    }
}

/// The props for the [`Calendar`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CalendarProps {
    /// The selected date
    #[props(default)]
    pub selected_date: ReadSignal<Option<Date>>,

    /// Callback when selected date changes
    #[props(default)]
    pub on_date_change: Callback<Option<Date>>,

    /// Callback when display weekday
    #[props(default = Callback::new(|weekday: Weekday| weekday_abbreviation(weekday).to_string()))]
    pub on_format_weekday: Callback<Weekday, String>,

    /// Callback when display month
    #[props(default = Callback::new(|month: Month| month.to_string()))]
    pub on_format_month: Callback<Month, String>,

    /// The month being viewed
    #[props(default = ReadSignal::new(Signal::new(UtcDateTime::now().date())))]
    pub view_date: ReadSignal<Date>,

    /// The current date (used for highlighting today)
    #[props(default = UtcDateTime::now().date())]
    pub today: Date,

    /// Callback when view date changes
    #[props(default)]
    pub on_view_change: Callback<Date>,

    /// Whether the calendar is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// First day of the week
    #[props(default = Weekday::Sunday)]
    pub first_day_of_week: Weekday,

    /// Lower limit of the range of available dates
    #[props(default = date!(1925-01-01))]
    pub min_date: Date,

    /// Upper limit of the range of available dates
    #[props(default = date!(2050-12-31))]
    pub max_date: Date,

    /// Unavailable dates
    #[props(default)]
    pub disabled_ranges: ReadSignal<Vec<DateRange>>,

    /// Additional attributes to extend the calendar element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the calendar element
    pub children: Element,
}

/// # Calendar
///
/// The [`Calendar`] component provides an accessible calendar interface with arrow key navigation, month switching, and date selection.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::calendar::{
///     Calendar, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// use time::{Date, Month, UtcDateTime};
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<Date>);
///     let mut view_date = use_signal(|| UtcDateTime::now().date());
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: Date| {
///                 tracing::info!("View changed to: {}-{}", new_view.year(), new_view.month());
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
    // Create base context provider for child components
    let mut base_ctx = use_context_provider(|| BaseCalendarContext {
        focused_date: Signal::new(props.selected_date.cloned()),
        view_date: props.view_date,
        set_view_date: props.on_view_change,
        available_range: Signal::new(None::<DateRange>),
        format_weekday: props.on_format_weekday,
        format_month: props.on_format_month,
        disabled: props.disabled,
        today: props.today,
        first_day_of_week: props.first_day_of_week,
        min_date: props.min_date,
        max_date: props.max_date,
        disabled_ranges: props.disabled_ranges,
    });
    // Create Calendar context provider for child components
    use_context_provider(|| CalendarContext {
        selected_date: props.selected_date,
        set_selected_date: props.on_date_change,
    });

    rsx! {
        div {
            role: "application",
            aria_label: "Calendar",
            "data-disabled": (props.disabled)(),
            onkeydown: move |e| {
                let Some(focused_date) = (base_ctx.focused_date)() else {
                    return;
                };
                let mut set_focused_date = |new_date: Option<Date>| {
                    // Make sure the view date month is the same as the focused date
                    let mut view_date = (base_ctx.view_date)();
                    if let Some(date) = new_date {
                        if date.month() != view_date.month() {
                            view_date = date.replace_day(1).unwrap();
                            (base_ctx.set_view_date)(view_date);
                        }
                    }

                    match new_date {
                        Some(date) => {
                            if base_ctx.min_date <= date && date <= base_ctx.max_date {
                                base_ctx.focused_date.set(new_date);
                            }
                        },
                        None => base_ctx.focused_date.set(None)
                    }
                };
                match e.key() {
                    Key::ArrowLeft => {
                        e.prevent_default();
                        set_focused_date(focused_date.previous_day());
                    }
                    Key::ArrowRight => {
                        e.prevent_default();
                        set_focused_date(focused_date.next_day());
                    }
                    Key::ArrowUp => {
                        e.prevent_default();
                        if e.modifiers().shift() {
                            if let Some(date) = previous_month(focused_date) {
                                set_focused_date(Some(date));
                            }
                        } else {
                            // Otherwise, move to the previous week
                            set_focused_date(Some(focused_date.saturating_sub(7.days())));
                        }
                    }
                    Key::ArrowDown => {
                        e.prevent_default();
                        if e.modifiers().shift() {
                            if let Some(date) = next_month(focused_date) {
                                set_focused_date(Some(date));
                            }
                        } else {
                            // Otherwise, move to the next week
                            set_focused_date(Some(focused_date.saturating_add(7.days())));
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

/// The context provided by the [`RangeCalendar`] component to its children.
#[derive(Copy, Clone)]
pub struct RangeCalendarContext {
    // The date that the user clicked on to begin range selection
    anchor_date: Signal<Option<Date>>,
    // Currently highlighted date range
    highlighted_range: Signal<Option<DateRange>>,
    set_selected_range: Callback<Option<DateRange>>,
}

impl RangeCalendarContext {
    /// Set the selected date
    pub fn set_selected_date(&mut self, date: Option<Date>) {
        match (self.anchor_date)() {
            Some(anchor) => {
                if let Some(date) = date {
                    self.anchor_date.set(None);

                    let range = DateRange::new(date, anchor);
                    self.set_selected_range.call(Some(range));
                    self.highlighted_range.set(Some(range));
                }
            }
            None => {
                self.anchor_date.set(date);

                let range = date.map(|d| DateRange::new(d, d));
                self.highlighted_range.set(range);
            }
        }
    }

    /// Set the selected date range by hovered date
    pub fn set_hovered_date(&mut self, date: Date) {
        if let Some(anchor) = (self.anchor_date)() {
            let range = DateRange::new(anchor, date);
            self.highlighted_range.set(Some(range));
        }
    }

    /// Set previous selected range
    pub fn reset_selection(&mut self, range: Option<DateRange>) {
        self.anchor_date.set(None);
        self.highlighted_range.set(range);
    }
}

/// The props for the [`RangeCalendar`] component.
#[derive(Props, Clone, PartialEq)]
pub struct RangeCalendarProps {
    /// The selected range
    #[props(default)]
    pub selected_range: ReadSignal<Option<DateRange>>,

    /// Callback when selected date range changes
    #[props(default)]
    pub on_range_change: Callback<Option<DateRange>>,

    /// Callback when display weekday
    #[props(default = Callback::new(|weekday: Weekday| weekday_abbreviation(weekday).to_string()))]
    pub on_format_weekday: Callback<Weekday, String>,

    /// Callback when display month
    #[props(default = Callback::new(|month: Month| month.to_string()))]
    pub on_format_month: Callback<Month, String>,

    /// The month being viewed
    #[props(default = ReadSignal::new(Signal::new(UtcDateTime::now().date())))]
    pub view_date: ReadSignal<Date>,

    /// The current date (used for highlighting today)
    #[props(default = UtcDateTime::now().date())]
    pub today: Date,

    /// Callback when view date changes
    #[props(default)]
    pub on_view_change: Callback<Date>,

    /// Whether the calendar is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// First day of the week
    #[props(default = Weekday::Sunday)]
    pub first_day_of_week: Weekday,

    /// Lower limit of the range of available dates
    #[props(default = date!(1925-01-01))]
    pub min_date: Date,

    /// Upper limit of the range of available dates
    #[props(default = date!(2050-12-31))]
    pub max_date: Date,

    /// Unavailable dates
    #[props(default)]
    pub disabled_ranges: ReadSignal<Vec<DateRange>>,

    /// Additional attributes to extend the calendar element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the calendar element
    pub children: Element,
}

/// # RangeCalendar
///
/// The [`RangeCalendar`] component provides an accessible calendar interface with arrow key navigation, month switching, and date selection.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::calendar::*;
/// use time::{Date, Month, UtcDateTime};
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_range = use_signal(|| None::<DateRange>);
///     let mut view_date = use_signal(|| UtcDateTime::now().date());
///     rsx! {
///         RangeCalendar {
///             selected_range: selected_range(),
///             on_range_change: move |range| {
///                 tracing::info!("Selected range: {:?}", range);
///                 selected_range.set(range);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: Date| {
///                 tracing::info!("View changed to: {}-{}", new_view.year(), new_view.month());
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
/// The [`RangeCalendar`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the calendar is disabled. Possible values are `true` or `false`.
#[component]
pub fn RangeCalendar(props: RangeCalendarProps) -> Element {
    let focused_date = use_signal(|| {
        let range = (props.selected_range)();
        range.map(|r| r.end)
    });
    let anchor_date = use_signal(|| None::<Date>);
    let highlighted_range = use_signal(|| (props.selected_range)());
    let available_range = use_signal(|| None::<DateRange>);

    // Create base context provider for child components
    let mut base_ctx = use_context_provider(|| BaseCalendarContext {
        focused_date,
        view_date: props.view_date,
        set_view_date: props.on_view_change,
        available_range,
        format_weekday: props.on_format_weekday,
        format_month: props.on_format_month,
        disabled: props.disabled,
        today: props.today,
        first_day_of_week: props.first_day_of_week,
        min_date: props.min_date,
        max_date: props.max_date,
        disabled_ranges: props.disabled_ranges,
    });

    // Create RangeCalendar context provider for child components
    let mut ctx = use_context_provider(|| RangeCalendarContext {
        anchor_date,
        highlighted_range,
        set_selected_range: props.on_range_change,
    });

    rsx! {
        div {
            role: "application",
            aria_label: "Calendar",
            "data-disabled": (props.disabled)(),
            onkeydown: move |e| {
                let Some(mut focused_date) = (base_ctx.focused_date)() else {
                    return;
                };

                // force hover day as focus
                if let (Some(range), Some(date)) = ((ctx.highlighted_range)(), (ctx.anchor_date)()) {
                      if date != range.start {
                            focused_date = range.start
                        } else {
                            focused_date = range.end
                        }
                };

                let mut set_focused_date = |new_date: Option<Date>| {
                    // Make sure the view date month is the same as the focused date
                    let mut view_date = (base_ctx.view_date)();
                    if let Some(date) = new_date {
                        if date.month() != view_date.month() {
                            view_date = date.replace_day(1).unwrap();
                            (base_ctx.set_view_date)(view_date);
                        }
                    }

                    match new_date {
                        Some(date) => {
                            if base_ctx.min_date <= date && date <= base_ctx.max_date {
                                base_ctx.focused_date.set(new_date);
                                let date = match (base_ctx.available_range)() {
                                    Some(range) => range.clamp(date),
                                    None => date,
                                };
                                ctx.set_hovered_date(date);
                            }
                        },
                        None => base_ctx.focused_date.set(None)
                    }
                };
                match e.key() {
                    Key::ArrowLeft => {
                        e.prevent_default();
                        set_focused_date(focused_date.previous_day());
                    }
                    Key::ArrowRight => {
                        e.prevent_default();
                        set_focused_date(focused_date.next_day());
                    }
                    Key::ArrowUp => {
                        e.prevent_default();
                        if e.modifiers().shift() {
                            if let Some(date) = previous_month(focused_date) {
                                set_focused_date(Some(date));
                            }
                        } else {
                            // Otherwise, move to the previous week
                            set_focused_date(Some(focused_date.saturating_sub(7.days())));
                        }
                    }
                    Key::ArrowDown => {
                        e.prevent_default();
                        if e.modifiers().shift() {
                            if let Some(date) = next_month(focused_date) {
                                set_focused_date(Some(date));
                            }
                        } else {
                            // Otherwise, move to the next week
                            set_focused_date(Some(focused_date.saturating_add(7.days())));
                        }
                    }
                    Key::Escape => {
                        ctx.reset_selection((props.selected_range)());
                        base_ctx.available_range.set(None);
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
    pub id: Option<String>,

    /// Additional attributes to extend the header element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the header element
    pub children: Element,
}

/// # CalendarHeader
///
/// The [`CalendarHeader`] component displays the header for the calendar. It typically contains the [`CalendarNavigation`] component
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::calendar::{
///     Calendar, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// use time::{Date, Month, UtcDateTime};
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<Date>);
///     let mut view_date = use_signal(|| UtcDateTime::now().date());
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: Date| {
///                 tracing::info!("View changed to: {}-{}", new_view.year(), new_view.month());
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
    pub attributes: Vec<Attribute>,

    /// The children of the navigation element
    #[props(default)]
    pub children: Element,
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
///     Calendar, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// use time::{Date, Month, UtcDateTime};
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<Date>);
///     let mut view_date = use_signal(|| UtcDateTime::now().date());
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: Date| {
///                 tracing::info!("View changed to: {}-{}", new_view.year(), new_view.month());
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
    pub attributes: Vec<Attribute>,

    /// The children of the button element
    pub children: Element,
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
///     Calendar, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// use time::{Date, Month, UtcDateTime};
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<Date>);
///     let mut view_date = use_signal(|| UtcDateTime::now().date());
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: Date| {
///                 tracing::info!("View changed to: {}-{}", new_view.year(), new_view.month());
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
    let ctx: BaseCalendarContext = use_context();
    // disable previous button when we reach the limit
    let button_disabled = use_memo(move || {
        // Get the current view date from context
        let view_date = (ctx.view_date)();
        match previous_month(view_date) {
            Some(date) => ctx.min_date.replace_day(1).unwrap() > date,
            None => true,
        }
    });
    // disable previous button when the current selection range does not include the previous month
    let navigate_disabled = use_memo(move || {
        // Get the current view date from context
        let view_date = (ctx.view_date)();
        (ctx.available_range)().is_some_and(|range| range.start.month() == view_date.month())
    });

    // Handle navigation to previous month
    let handle_prev_month = move |e: Event<MouseData>| {
        e.prevent_default();
        let current_view = (ctx.view_date)();
        if let Some(date) = previous_month(current_view) {
            ctx.set_view_date.call(date)
        }
    };

    rsx! {
        button {
            class: "calendar-nav-prev",
            aria_label: "Previous month",
            type: "button",
            onclick: handle_prev_month,
            disabled: (ctx.disabled)() || button_disabled() || navigate_disabled(),
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
    pub attributes: Vec<Attribute>,

    /// The children of the button element
    pub children: Element,
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
///     Calendar, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// use time::{Date, Month, UtcDateTime};
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<Date>);
///     let mut view_date = use_signal(|| UtcDateTime::now().date());
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: Date| {
///                 tracing::info!("View changed to: {}-{}", new_view.year(), new_view.month());
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
    let ctx: BaseCalendarContext = use_context();
    // disable next button when we reach the limit
    let button_disabled = use_memo(move || {
        // Get the current view date from context
        let view_date = (ctx.view_date)();
        match next_month(view_date) {
            Some(date) => {
                let last_day = ctx.max_date.month().length(ctx.max_date.year());
                ctx.max_date.replace_day(last_day).unwrap() < date
            }
            None => true,
        }
    });
    // disable next button when the current selection range does not include the next month
    let navigate_disabled = use_memo(move || {
        // Get the current view date from context
        let view_date = (ctx.view_date)();
        (ctx.available_range)().is_some_and(|range| range.end.month() == view_date.month())
    });

    // Handle navigation to next month
    let handle_next_month = move |e: Event<MouseData>| {
        e.prevent_default();
        let current_view = (ctx.view_date)();
        if let Some(date) = next_month(current_view) {
            ctx.set_view_date.call(date)
        }
    };

    rsx! {
        button {
            class: "calendar-nav-next",
            aria_label: "Next month",
            type: "button",
            onclick: handle_next_month,
            disabled: (ctx.disabled)() || button_disabled() || navigate_disabled(),
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
    pub attributes: Vec<Attribute>,
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
///     Calendar, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// use time::{Date, Month, UtcDateTime};
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<Date>);
///     let mut view_date = use_signal(|| UtcDateTime::now().date());
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: Date| {
///                 tracing::info!("View changed to: {}-{}", new_view.year(), new_view.month());
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
    let ctx: BaseCalendarContext = use_context();
    // Format the current month and year
    let month_year = use_memo(move || {
        let view_date = (ctx.view_date)();
        format!("{} {}", view_date.month(), view_date.year())
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

    /// The callback that will be used to render each day in the grid
    #[props(default = Callback::new(|date: Date| {
        rsx! { CalendarDay { date } }
    }))]
    pub render_day: Callback<Date, Element>,

    /// Additional attributes to apply to the grid element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
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
///     Calendar, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// use time::{Date, Month, UtcDateTime};
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<Date>);
///     let mut view_date = use_signal(|| UtcDateTime::now().date());
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: Date| {
///                 tracing::info!("View changed to: {}-{}", new_view.year(), new_view.month());
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
    let ctx: BaseCalendarContext = use_context();

    // We'll use the view_date from context in the memo below

    // Generate a grid of days with proper layout
    // Use the view_date as a dependency to ensure the grid updates when the view changes
    let days_grid = use_memo(move || {
        // Get the current view date from context
        let view_date = (ctx.view_date)();

        // Create a grid with empty cells for padding and actual days
        let mut grid = Vec::new();

        // Add empty cells for days before the first day of the month
        let previous_month = view_date
            .replace_day(1)
            .expect("invalid or out-of-range date");
        let num_days = days_since(view_date, ctx.first_day_of_week);
        let mut date = previous_month.saturating_sub(num_days.days());
        for _ in 1..=num_days {
            grid.push(date);
            date = date.next_day().expect("invalid or out-of-range date");
        }

        let mut date = view_date;
        // Add days of the month
        let num_days_in_month = view_date.month().length(view_date.year());
        for day in 1..=num_days_in_month {
            date = view_date
                .replace_day(day)
                .expect("invalid or out-of-range date");
            grid.push(date);
        }

        // Add empty cells to complete the grid (for a clean layout)
        let remainder = grid.len() % 7;
        if remainder > 0 {
            for _ in 1..=(7 - remainder) {
                date = date.next_day().expect("invalid or out-of-range date");
                grid.push(date);
            }
        }

        // Turn the flat grid into a 2D grid (7 columns)
        grid.chunks(7)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>()
    });

    let weekday_headers = use_memo(move || {
        WeekdaySet(0b111_1111) // `WeekdaySet` containing all seven `Weekday`s
            .iter(ctx.first_day_of_week)
            .map(|weekday| (weekday, ctx.format_weekday.call(weekday)))
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
                    for (weekday, label) in weekday_headers() {
                        th {
                            key: "{weekday:?}",  // Add key for efficient diffing
                            class: "calendar-grid-day-header",
                            {label}
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

/// The props for the [`CalendarSelectMonth`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CalendarSelectMonthProps {
    /// Additional attributes to extend the select month element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// # CalendarSelectMonth
///
/// The [`CalendarSelectMonth`] component provides a drop-down list for selecting the current month.
///
/// This must be used inside a [`Calendar`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::calendar::{
///     Calendar, CalendarGrid, CalendarHeader, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton, CalendarSelectMonth
/// };
/// use time::{Date, Month, UtcDateTime};
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<Date>);
///     let mut view_date = use_signal(|| UtcDateTime::now().date());
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: Date| {
///                 tracing::info!("View changed to: {}-{}", new_view.year(), new_view.month());
///                 view_date.set(new_view);
///             },
///             CalendarHeader {
///                 CalendarNavigation {
///                     CalendarPreviousMonthButton {
///                         "<"
///                     }
///                     CalendarSelectMonth {}
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
pub fn CalendarSelectMonth(props: CalendarSelectMonthProps) -> Element {
    let calendar: BaseCalendarContext = use_context();
    let view_date = calendar.view_date();
    let month = view_date.month();

    let months = use_memo(move || {
        // Get the current view date from context
        let view_date = (calendar.view_date)();
        let mut min_month = Month::January;
        if replace_month(view_date, min_month) < calendar.min_date {
            min_month = calendar.min_date.month();
        }
        let mut max_month = Month::December;
        if replace_month(view_date, max_month) > calendar.max_date {
            max_month = calendar.max_date.month();
        }

        let mut month = min_month;
        let mut months = Vec::new();
        loop {
            months.push(month);

            if month == max_month {
                return months;
            }
            month = month.next();
        }
    });

    rsx! {
        span { class: "calendar-month-select-container",
            select {
                aria_label: "Month",
                onchange: move |e| {
                    let mut view_date = calendar.view_date();
                    let number = e.value().parse().unwrap_or(view_date.month() as u8);
                    let cur_month = Month::try_from(number).expect("Month out-of-range");
                    view_date = view_date.replace_month(cur_month).unwrap_or(view_date);
                    calendar.set_view_date(view_date);
                },
                ..props.attributes,
                for month in months() {
                    option {
                        value: month as u8,
                        selected: calendar.view_date().month() == month,
                        {calendar.format_month.call(month)}
                    }
                }
            }
            span { class: "calendar-month-select-value",
                {calendar.format_month.call(month)}
                svg {
                    class: "select-expand-icon",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    polyline { points: "6 9 12 15 18 9" }
                }
            }
        }
    }
}

/// The props for the [`CalendarSelectYear`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CalendarSelectYearProps {
    /// Additional attributes to extend the select year element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// # CalendarSelectYear
///
/// The [`CalendarSelectYear`] component provides a drop-down list for selecting the current year.
///
/// This must be used inside a [`Calendar`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::calendar::{
///     Calendar, CalendarGrid, CalendarHeader, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton, CalendarSelectYear
/// };
/// use time::{Date, Month, UtcDateTime};
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<Date>);
///     let mut view_date = use_signal(|| UtcDateTime::now().date());
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: Date| {
///                 tracing::info!("View changed to: {}-{}", new_view.year(), new_view.month());
///                 view_date.set(new_view);
///             },
///             CalendarHeader {
///                 CalendarNavigation {
///                     CalendarPreviousMonthButton {
///                         "<"
///                     }
///                     CalendarSelectYear {}
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
pub fn CalendarSelectYear(props: CalendarSelectYearProps) -> Element {
    let calendar: BaseCalendarContext = use_context();
    let view_date = calendar.view_date();
    let year = view_date.year();

    let years = use_memo(move || {
        // Get the current view date from context
        let view_date = (calendar.view_date)();
        let month = view_date.month();
        let mut min_year = calendar.min_date.year();
        if replace_month(calendar.min_date, month) < calendar.min_date {
            min_year += 1;
        }
        let mut max_year = calendar.max_date.year();
        if replace_month(calendar.max_date, month) > calendar.max_date {
            max_year -= 1;
        }

        min_year..=max_year
    });

    rsx! {
        span { class: "calendar-year-select-container",
            select {
                aria_label: "Year",
                onchange: move |e| {
                    let mut view_date = calendar.view_date();
                    let year = e.value().parse().unwrap_or(view_date.year());
                    view_date = view_date.replace_year(year).unwrap_or(view_date);
                    calendar.set_view_date(view_date);
                },
                ..props.attributes,
                for year in years() {
                    option {
                        value: year,
                        selected: calendar.view_date().year() == year,
                        "{year}"
                    }
                }
            }
            span { class: "calendar-year-select-value",
                "{year}"
                svg {
                    class: "select-expand-icon",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    polyline { points: "6 9 12 15 18 9" }
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

impl RelativeMonth {
    fn current_month(&self) -> bool {
        *self == RelativeMonth::Current
    }
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

/// Get a human-readable ARIA label for input date
fn aria_label(date: &Date) -> String {
    format!(
        "{}, {} {}, {}",
        date.weekday(),
        date.month(),
        date.day(),
        date.year()
    )
}

/// The props for the [`CalendarDay`] component.
#[derive(Props, Clone, Debug, PartialEq)]
pub struct CalendarDayProps {
    date: Date,
    /// Additional attributes to extend the calendar day element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// # CalendarDay
///
/// The [`CalendarDay`] component provides an accessible calendar interface for a date
///
/// This must be used inside a [`CalendarGrid`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::calendar::*;
/// use time::{Date, Month, UtcDateTime};
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_range = use_signal(|| None::<DateRange>);
///     let mut view_date = use_signal(|| UtcDateTime::now().date());
///     rsx! {
///         RangeCalendar {
///             selected_range: selected_range(),
///             on_range_change: move |range| {
///                 tracing::info!("Selected range: {:?}", range);
///                 selected_range.set(range);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: Date| {
///                 tracing::info!("View changed to: {}-{}", new_view.year(), new_view.month());
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
/// The [`CalendarDay`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the calendar is disabled. Possible values are `true` or `false`.
/// - `data-unavailable`: Indicates if the date is unavailable. Possible values are `true` or `false`.
/// - `data-today`: Indicates if the cell is today. Possible values are `true` or `false`.
/// - `data-month`: The relative month of the date. Possible values are `last`,
/// - `data-selected`: Indicates if the cell is selected. Possible values are `true` or `false`.
/// - `data-selection-start`: Indicates if cell is the first date in a range selection. Possible values are `true` or `false`.
/// - `data-selection-between`: Indicates if a date interval contains a cell. Possible values are `true` or `false`.
/// - `data-selection-end`: Indicates if cell is the last date in a range selection. Possible values are `true` or `false`.
#[component]
pub fn CalendarDay(props: CalendarDayProps) -> Element {
    let single_context = try_use_context::<CalendarContext>().is_some();

    if single_context {
        rsx! {
            SingleCalendarDay { date: props.date, attributes: props.attributes.clone() }
        }
    } else {
        rsx! {
            RangeCalendarDay { date: props.date, attributes: props.attributes.clone() }
        }
    }
}

fn relative_calendar_month(
    date: Date,
    base_ctx: &BaseCalendarContext,
    view_date: Date,
) -> RelativeMonth {
    if date < base_ctx.min_date {
        RelativeMonth::Last
    } else if date > base_ctx.max_date {
        RelativeMonth::Next
    } else {
        let lhs = date.month() as u8;
        let rhs = view_date.month() as u8;
        match lhs.cmp(&rhs) {
            std::cmp::Ordering::Less => RelativeMonth::Last,
            std::cmp::Ordering::Equal => RelativeMonth::Current,
            std::cmp::Ordering::Greater => RelativeMonth::Next,
        }
    }
}

fn is_between(date: Date, range: Option<DateRange>) -> bool {
    range.is_some_and(|r| r.contained_in_interval(date))
}

fn is_start(date: Date, range: Option<DateRange>) -> bool {
    range.is_some_and(|r| r.start == date && date != r.end)
}

fn is_end(date: Date, range: Option<DateRange>) -> bool {
    range.is_some_and(|r| r.end == date && date != r.start)
}

fn use_day_mounted_ref(
    mut is_focused: impl FnMut() -> bool + 'static,
) -> impl FnMut(MountedEvent) + 'static {
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
    move |e| day_ref.set(Some(e.data()))
}

#[component]
fn SingleCalendarDay(props: CalendarDayProps) -> Element {
    let CalendarDayProps { date, attributes } = props;
    let mut base_ctx: BaseCalendarContext = use_context();
    let day = date.day();
    let view_date = (base_ctx.view_date)();
    let month = relative_calendar_month(date, &base_ctx, view_date);
    let in_current_month = month.current_month();
    let is_focused = move || base_ctx.is_focused(date);
    let is_today = date == base_ctx.today;
    let is_unavailable = base_ctx.is_unavailable(date);

    let is_disabled = move || {
        if (base_ctx.disabled)() {
            return true;
        }

        is_unavailable
    };
    let onmounted = use_day_mounted_ref(is_focused);

    let ctx: CalendarContext = use_context();
    let is_selected = move || (ctx.selected_date)().is_some_and(|d| d == date);

    // Handle day selection
    let mut handle_day_select = move |day: u8| {
        if (base_ctx.disabled)() || is_unavailable {
            return;
        }
        let view_date = (base_ctx.view_date)();
        let date = view_date.replace_day(day).unwrap();
        ctx.set_selected_date.call((!is_selected()).then_some(date));
        base_ctx.focused_date.set(Some(date));
    };

    let focusable_date = (base_ctx.focused_date)()
        .filter(|d| d.month() == view_date.month())
        .or_else(|| {
            ctx.selected_date
                .cloned()
                .filter(|d| d.month() == view_date.month())
        })
        .unwrap_or(view_date);

    rsx! {
        button {
            class: "calendar-grid-cell",
            type: "button",
            tabindex: if date == focusable_date {
                "0"
            } else {
                "-1"
            },
            aria_label: aria_label(&props.date),
            "data-today": is_today,
            "data-selected": is_selected(),
            "data-unavailable": if is_unavailable { true },
            "data-disabled": is_disabled(),
            "data-month": "{month}",
            onclick: move |e| {
                e.prevent_default();
                if in_current_month {
                    handle_day_select(day);
                }
            },
            onfocus: move |_| {
                if in_current_month {
                    base_ctx.focused_date.set(Some(date));
                }
            },
            onmounted,
            ..attributes,
            {day.to_string()}
        }
    }
}

#[component]
fn RangeCalendarDay(props: CalendarDayProps) -> Element {
    let CalendarDayProps { date, attributes } = props;
    let mut base_ctx: BaseCalendarContext = use_context();
    let day = date.day();
    let view_date = (base_ctx.view_date)();
    let month = relative_calendar_month(date, &base_ctx, view_date);
    let in_current_month = month.current_month();
    let is_focused = move || base_ctx.is_focused(date);
    let is_today = date == base_ctx.today;
    let is_unavailable = base_ctx.is_unavailable(date);

    let is_disabled = move || {
        if (base_ctx.disabled)() {
            return true;
        }

        is_unavailable
    };
    let onmounted = use_day_mounted_ref(is_focused);

    let mut ctx: RangeCalendarContext = use_context();
    let is_selected = move || (ctx.highlighted_range)().is_some_and(|r| r.contains(date));
    let is_between = move || is_between(date, ctx.highlighted_range.cloned());
    let is_start = move || is_start(date, ctx.highlighted_range.cloned());
    let is_end = move || is_end(date, ctx.highlighted_range.cloned());

    // Handle day selection
    let mut handle_day_select = move |day: u8| {
        if is_disabled() || is_unavailable {
            return;
        }

        let view_date = (base_ctx.view_date)();
        let date = view_date.replace_day(day).ok();
        ctx.set_selected_date(date);
        base_ctx.focused_date.set(date);

        let anchor_date = (ctx.anchor_date)();
        base_ctx
            .available_range
            .set(base_ctx.available_range(anchor_date));
    };

    let focusable_date = (base_ctx.focused_date)()
        .filter(|d| d.month() == view_date.month())
        .or_else(|| {
            ctx.anchor_date
                .cloned()
                .filter(|d| d.month() == view_date.month())
        })
        .unwrap_or(view_date);

    rsx! {
        button {
            class: "calendar-grid-cell",
            type: "button",
            tabindex: if date == focusable_date {
                "0"
            } else {
                "-1"
            },
            aria_label: aria_label(&props.date),
            "data-disabled": is_disabled(),
            "data-today": if is_today { true },
            "data-selected": is_selected(),
            "data-unavailable": if is_unavailable { true },
            "data-selection-start": if is_start() { true },
            "data-selection-between": if is_between() { true },
            "data-selection-end": if is_end() { true },
            "data-month": "{month}",
            onclick: move |e| {
                e.prevent_default();
                if in_current_month {
                    handle_day_select(day);
                }
            },
            onfocus: move |_| {
                if in_current_month {
                    base_ctx.focused_date.set(Some(date));
                }
            },
            onmouseover: move |_| {
                if in_current_month {
                    let date = (base_ctx.available_range)().map_or(date, |range| range.clamp(date));
                    ctx.set_hovered_date(date);
                }
            },
            onmounted,
            ..attributes,
            {day.to_string()}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

    #[test]
    fn test_weekday_set() {
        let mut weekdays = WeekdaySet::single(Weekday::Monday);
        // Test contains
        assert!(weekdays.contains(Weekday::Monday));
        assert!(!weekdays.contains(Weekday::Tuesday));

        // Test remove
        assert!(weekdays.remove(Weekday::Monday));
        assert!(!weekdays.contains(Weekday::Monday));
        assert!(!weekdays.remove(Weekday::Monday)); // Already removed

        let all_days = WeekdaySet(0b111_1111); // All days
        let empty_set = WeekdaySet(0b000_0000); // Empty
        let single_set = WeekdaySet::single(Weekday::Friday); // Single day
        let part_size_set = WeekdaySet(0b010_1010); // Tu, Th, Sa

        // Test iterator
        let days: Vec<_> = all_days.iter(Weekday::Sunday).collect();
        assert_eq!(days.len(), 7);
        assert_eq!(days[0], Weekday::Sunday);

        let mut iter = all_days.iter(Weekday::Wednesday);
        assert_eq!(iter.next(), Some(Weekday::Wednesday));
        assert_eq!(iter.next(), Some(Weekday::Thursday));

        // Test first
        assert_eq!(empty_set.first(), None);
        assert_eq!(single_set.first(), Some(Weekday::Friday));
        assert_eq!(part_size_set.first(), Some(Weekday::Tuesday));
        assert_eq!(all_days.first(), Some(Weekday::Monday));

        // Test is_empty
        assert!(empty_set.is_empty());
        assert!(!part_size_set.is_empty());
        assert!(!single_set.is_empty());
        assert!(!all_days.is_empty());
    }

    #[test]
    fn test_days_since() {
        // Test days since calculation
        let date = date!(2024 - 01 - 01); // Monday
        assert_eq!(days_since(date, Weekday::Monday), 0);
        assert_eq!(days_since(date, Weekday::Sunday), 1);
        assert_eq!(days_since(date, Weekday::Tuesday), 6);
    }

    #[test]
    fn test_month_navigation() {
        let date = date!(2024 - 01 - 15);

        // Test next month
        let next = next_month(date);
        assert!(next.is_some());
        assert_eq!(next.unwrap().month(), Month::February);
        assert_eq!(next.unwrap().year(), 2024);
        assert_eq!(next.unwrap().day(), 15);

        // Test previous month
        let prev = previous_month(date);
        assert!(prev.is_some());
        assert_eq!(prev.unwrap().month(), Month::December);
        assert_eq!(prev.unwrap().year(), 2023);
        assert_eq!(prev.unwrap().day(), 15);
    }

    #[test]
    fn test_calendar_grid_weeks() {
        // Helper function to generate grid (mimics what CalendarGrid does)
        fn generate_test_grid(view_date: Date, first_day_of_week: Weekday) -> Vec<Vec<Date>> {
            let mut grid = Vec::new();

            let first_of_month = view_date.replace_day(1).unwrap();
            let start_offset = days_since(view_date, first_day_of_week) as usize;

            // Add previous month's trailing days
            if start_offset > 0 {
                if let Some(mut date) = first_of_month.previous_day() {
                    for _ in 1..start_offset {
                        date = date.previous_day().unwrap_or(date);
                    }
                    for _ in 0..start_offset {
                        grid.push(date);
                        date = date.next_day().unwrap_or(date);
                    }
                }
            }

            // Add current month's days
            let days_in_month = view_date.month().length(view_date.year());
            for day in 1..=days_in_month {
                if let Ok(date) = view_date.replace_day(day) {
                    grid.push(date);
                }
            }

            // Add next month's days to complete the week
            let remainder = grid.len() % 7;
            if remainder > 0 {
                if let Ok(last_day) = view_date.replace_day(days_in_month) {
                    if let Some(mut date) = last_day.next_day() {
                        for _ in 0..(7 - remainder) {
                            grid.push(date);
                            date = date.next_day().unwrap_or(date);
                        }
                    }
                }
            }

            grid.chunks(7).map(|week| week.to_vec()).collect()
        }

        // Test February 2021: starts on Monday, 28 days
        // When first day of week is Monday, should fit in exactly 4 weeks
        let feb_2021 = date!(2021 - 02 - 15);
        let feb_grid = generate_test_grid(feb_2021, Weekday::Monday);
        assert_eq!(
            feb_grid.len(),
            4,
            "February 2021 should have exactly 4 weeks"
        );

        // Count days from February in the grid
        let feb_days: Vec<_> = feb_grid
            .iter()
            .flatten()
            .filter(|d| d.month() == Month::February && d.year() == 2021)
            .collect();
        assert_eq!(
            feb_days.len(),
            28,
            "Should have all 28 days of February 2021"
        );

        // Test May 2024: starts on Wednesday, 31 days
        // When first day of week is Sunday, should need 5 weeks
        let may_2024 = date!(2024 - 05 - 15);
        let may_grid = generate_test_grid(may_2024, Weekday::Sunday);
        assert_eq!(
            may_grid.len(),
            5,
            "May 2024 should have exactly 5 weeks when starting from Sunday"
        );

        // Count days from May in the grid
        let may_days: Vec<_> = may_grid
            .iter()
            .flatten()
            .filter(|d| d.month() == Month::May && d.year() == 2024)
            .collect();
        assert_eq!(may_days.len(), 31, "Should have all 31 days of May 2024");

        // Test that we don't generate empty trailing weeks
        // December 2018: starts on Saturday, 31 days (when week starts on Sunday)
        // Should need exactly 6 weeks (30 days in November + 31 in December + 5 in January = 66/7 = 6)
        let dec_2018 = date!(2018 - 12 - 15);
        let dec_grid = generate_test_grid(dec_2018, Weekday::Sunday);
        assert_eq!(
            dec_grid.len(),
            6,
            "December 2018 should have exactly 6 weeks"
        );

        // Verify no week is completely empty
        for week in &dec_grid {
            assert!(!week.is_empty(), "No week should be empty");
            assert_eq!(week.len(), 7, "Each week should have exactly 7 days");
        }
    }
}
