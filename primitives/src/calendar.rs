//! Defines the [`Calendar`] component and its sub-components, which provide a calendar interface with date selection and navigation.

use dioxus::prelude::*;
use std::{
    fmt::{self, Display},
    rc::Rc,
};

use chrono::{Datelike, Days, Local, Month, Months, NaiveDate, Weekday, WeekdaySet};

/// The context provided by the [`Calendar`] component to its children.
#[derive(Copy, Clone)]
pub struct CalendarContext {
    // State
    selected_date: ReadOnlySignal<Option<NaiveDate>>,
    set_selected_date: Callback<Option<NaiveDate>>,
    focused_date: Signal<Option<NaiveDate>>,
    view_date: ReadOnlySignal<NaiveDate>,
    set_view_date: Callback<NaiveDate>,
    format_weekday: Callback<Weekday, String>,

    // Configuration
    disabled: ReadOnlySignal<bool>,
    today: NaiveDate,
    first_day_of_week: Weekday,
}

impl CalendarContext {
    /// Get the currently selected date
    pub fn selected_date(&self) -> Option<NaiveDate> {
        self.selected_date.cloned()
    }

    /// Set the selected date
    pub fn set_selected_date(&self, date: Option<NaiveDate>) {
        (self.set_selected_date)(date);
    }

    /// Get the currently focused date
    pub fn focused_date(&self) -> Option<NaiveDate> {
        self.focused_date.cloned()
    }

    /// Set the focused date
    pub fn set_focused_date(&mut self, date: Option<NaiveDate>) {
        self.focused_date.set(date);
    }

    /// Get the current view date
    pub fn view_date(&self) -> NaiveDate {
        self.view_date.cloned()
    }

    /// Set the view date
    pub fn set_view_date(&self, date: NaiveDate) {
        (self.set_view_date)(date);
    }

    /// Check if the calendar is disabled
    pub fn is_disabled(&self) -> bool {
        self.disabled.cloned()
    }
}

/// The props for the [`Calendar`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CalendarProps {
    /// The selected date
    #[props(default)]
    pub selected_date: ReadOnlySignal<Option<NaiveDate>>,

    /// Callback when selected date changes
    #[props(default)]
    pub on_date_change: Callback<Option<NaiveDate>>,

    /// callback when localizing day of the week
    #[props(default = Callback::new(|weekday| format!("{weekday}")))]
    pub on_format_weekday: Callback<Weekday, String>,

    /// The month being viewed
    pub view_date: ReadOnlySignal<NaiveDate>,

    /// The current date (used for highlighting today)
    #[props(default = Local::now().date_naive())]
    pub today: NaiveDate,

    /// Callback when view date changes
    #[props(default)]
    pub on_view_change: Callback<NaiveDate>,

    /// Whether the calendar is disabled
    #[props(default)]
    pub disabled: ReadOnlySignal<bool>,

    /// First day of the week
    #[props(default = Weekday::Sun)]
    pub first_day_of_week: Weekday,

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
///     Calendar, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// use chrono::{Datelike, NaiveDate, Utc};
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<NaiveDate>);
///     let mut view_date = use_signal(|| Utc::now().date_naive());
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: NaiveDate| {
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
    // Create context provider for child components
    let mut ctx = use_context_provider(|| CalendarContext {
        selected_date: props.selected_date,
        set_selected_date: props.on_date_change,
        focused_date: Signal::new(props.selected_date.cloned()),
        view_date: props.view_date,
        set_view_date: props.on_view_change,
        format_weekday: props.on_format_weekday,
        disabled: props.disabled,
        today: props.today,
        first_day_of_week: props.first_day_of_week,
    });

    rsx! {
        div {
            role: "application",
            "aria-label": "Calendar",
            "data-disabled": (props.disabled)(),
            onkeydown: move |e| {
                let Some(focused_date) = (ctx.focused_date)() else {
                    return;
                };
                let mut set_focused_date = |new_date: Option<NaiveDate>| {
                    // Make sure the view date month is the same as the focused date
                    let mut view_date = (ctx.view_date)();
                    if let Some(date) = new_date {
                        if date.month() != view_date.month() {
                            view_date = date.with_day(view_date.day()).unwrap();
                            (ctx.set_view_date)(view_date);
                        }
                    }
                    ctx.focused_date.set(new_date);
                };
                match e.key() {
                    Key::ArrowLeft => {
                        e.prevent_default();
                        set_focused_date(focused_date.pred_opt());
                    }
                    Key::ArrowRight => {
                        e.prevent_default();
                        set_focused_date(focused_date.succ_opt());
                    }
                    Key::ArrowUp => {
                        e.prevent_default();
                        if e.modifiers().shift() {
                            match focused_date.checked_sub_months(Months::new(1)) {
                                Some(prev_month) => set_focused_date(prev_month.with_day(1)),
                                None => set_focused_date(None)
                            }
                        } else {
                            // Otherwise, move to the previous week
                            set_focused_date(focused_date.checked_sub_days(Days::new(7)));
                        }
                    }
                    Key::ArrowDown => {
                        e.prevent_default();
                        if e.modifiers().shift() {
                            match focused_date.checked_add_months(Months::new(1)) {
                                Some(next_month) => set_focused_date(next_month.with_day(1)),
                                None => set_focused_date(None)
                            }
                        } else {
                            set_focused_date(focused_date.checked_add_days(Days::new(7)));
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
///     Calendar, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// use chrono::{Datelike, NaiveDate, Utc};
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<NaiveDate>);
///     let mut view_date = use_signal(|| Utc::now().date_naive());
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: NaiveDate| {
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
///     Calendar, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// use chrono::{Datelike, NaiveDate, Utc};
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<NaiveDate>);
///     let mut view_date = use_signal(|| Utc::now().date_naive());
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: NaiveDate| {
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
///     Calendar, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// use chrono::{Datelike, NaiveDate, Utc};
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<NaiveDate>);
///     let mut view_date = use_signal(|| Utc::now().date_naive());
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: NaiveDate| {
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
    let ctx: CalendarContext = use_context();
    // disable previous button when we reach the limit
    let mut button_disabled = use_signal(|| false);
    // Handle navigation to previous month
    let handle_prev_month = move |e: Event<MouseData>| {
        e.prevent_default();
        let current_view = (ctx.view_date)();
        match current_view.checked_sub_months(Months::new(1)) {
            Some(prev_month) => ctx.set_view_date.call(prev_month),
            None => button_disabled.set(true),
        }
    };

    rsx! {
        button {
            class: "calendar-nav-prev",
            aria_label: "Previous month",
            r#type: "button",
            onclick: handle_prev_month,
            disabled: (ctx.disabled)() || button_disabled(),
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
///     Calendar, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// use chrono::{Datelike, NaiveDate, Utc};
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<NaiveDate>);
///     let mut view_date = use_signal(|| Utc::now().date_naive());
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: NaiveDate| {
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
    let ctx: CalendarContext = use_context();
    // disable next button when we reach the limit
    let mut button_disabled = use_signal(|| false);
    // Handle navigation to next month
    let handle_next_month = move |e: Event<MouseData>| {
        e.prevent_default();
        let current_view = (ctx.view_date)();
        match current_view.checked_add_months(Months::new(1)) {
            Some(next_month) => ctx.set_view_date.call(next_month),
            None => button_disabled.set(true),
        }
    };

    rsx! {
        button {
            class: "calendar-nav-next",
            aria_label: "Next month",
            r#type: "button",
            onclick: handle_next_month,
            disabled: (ctx.disabled)() || button_disabled(),
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
///     Calendar, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// use chrono::{Datelike, NaiveDate, Utc};
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<NaiveDate>);
///     let mut view_date = use_signal(|| Utc::now().date_naive());
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: NaiveDate| {
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
    let ctx: CalendarContext = use_context();
    // Format the current month and year
    let month_year = use_memo(move || {
        let view_date = (ctx.view_date)();
        let month_name = month_name(&view_date);
        format!("{} {}", month_name, view_date.year())
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
    #[props(default = Callback::new(|date: NaiveDate| {
        rsx! { CalendarDay { date } }
    }))]
    pub render_day: Callback<NaiveDate, Element>,

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
///     Calendar, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton
/// };
/// use chrono::{Datelike, NaiveDate, Utc};
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_date = use_signal(|| None::<NaiveDate>);
///     let mut view_date = use_signal(|| Utc::now().date_naive());
///     rsx! {
///         Calendar {
///             selected_date: selected_date(),
///             on_date_change: move |date| {
///                 tracing::info!("Selected date: {:?}", date);
///                 selected_date.set(date);
///             },
///             view_date: view_date(),
///             on_view_change: move |new_view: NaiveDate| {
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
    let ctx: CalendarContext = use_context();

    // We'll use the view_date from context in the memo below

    // Generate a grid of days with proper layout
    // Use the view_date as a dependency to ensure the grid updates when the view changes
    let days_grid = use_memo(move || {
        // Get the current view date from context
        let view_date = (ctx.view_date)();
        let num_days_in_month = view_date.num_days_in_month();

        let weekday = view_date.with_day(1).unwrap().weekday();
        let first_day_offset = weekday.days_since(ctx.first_day_of_week);

        // Create a grid with empty cells for padding and actual days
        let mut grid = Vec::new();

        // Add empty cells for days before the first day of the month
        if let Some(previous_month) = view_date.checked_sub_months(Months::new(1)) {
            for index in 1..=first_day_offset {
                let day = previous_month.num_days_in_month() as u32 + index - first_day_offset;
                grid.push(
                    previous_month
                        .with_day(day)
                        .expect("invalid or out-of-range date"),
                );
            }
        }

        // Add days of the month
        for day in 1..=num_days_in_month {
            grid.push(
                view_date
                    .with_day(day as u32)
                    .expect("invalid or out-of-range date"),
            );
        }

        // Add empty cells to complete the grid (for a clean layout)
        if let Some(next_month) = view_date.checked_add_months(Months::new(1)) {
            let remainder = grid.len() % 7;
            if remainder > 0 {
                for day in 1..=(7 - remainder) {
                    grid.push(
                        next_month
                            .with_day(day as u32)
                            .expect("invalid or out-of-range date"),
                    );
                }
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
                    for weekday in WeekdaySet::ALL.iter(ctx.first_day_of_week) {
                        th {
                            class: "calendar-grid-day-header",
                            {ctx.format_weekday.call(weekday)}
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

/// Get a human-readable Month for input date
fn month_name(date: &NaiveDate) -> &str {
    let month = Month::try_from(date.month() as u8).expect("Month out of range");
    month.name()
}

/// Get a human-readable ARIA label for input date
fn aria_label(date: &NaiveDate) -> String {
    let month_name = month_name(date);
    let day_name = date.weekday();
    format!(
        "{}, {} {}, {}",
        day_name,
        month_name,
        date.day(),
        date.year()
    )
}

#[derive(Props, Clone, Debug, PartialEq)]
struct CalendarDayProps {
    date: NaiveDate,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
fn CalendarDay(props: CalendarDayProps) -> Element {
    let CalendarDayProps { date, attributes } = props;
    let mut ctx: CalendarContext = use_context();
    let view_date = (ctx.view_date)();
    let day = date.day();
    let month = match date.month().cmp(&view_date.month()) {
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
            let date = view_date.with_day(day).unwrap();
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

    let view_date = (ctx.view_date)();
    let focusable_date = (ctx.focused_date)()
        .filter(|d| d.month0() == view_date.month0())
        .or_else(|| {
            ctx.selected_date
                .cloned()
                .filter(|d| d.month0() == view_date.month0())
        })
        .unwrap_or(view_date);

    rsx! {
        button {
            class: "calendar-grid-cell",
            r#type: "button",
            tabindex: if date == focusable_date {
                "0"
            } else {
                "-1"
            },
            aria_label: aria_label(&props.date),
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
