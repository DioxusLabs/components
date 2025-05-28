use dioxus::{document::eval, prelude::*};
use dioxus_primitives::{
    accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger},
    aspect_ratio::AspectRatio,
    avatar::{Avatar, AvatarFallback, AvatarImage},
    calendar::{Calendar, CalendarDate, CalendarGrid, CalendarHeader, CalendarNavigation},
    checkbox::{Checkbox, CheckboxIndicator},
    collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
    context_menu::{ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger},
    dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger},
    hover_card::{HoverCard, HoverCardAlign, HoverCardContent, HoverCardSide, HoverCardTrigger},
    menubar::{Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger},
    progress::{Progress, ProgressIndicator},
    radio_group::{RadioGroup, RadioItem},
    scroll_area::{ScrollArea, ScrollDirection},
    select::{Select, SelectGroup, SelectOption},
    separator::Separator,
    slider::{Slider, SliderRange, SliderThumb, SliderTrack, SliderValue},
    switch::{Switch, SwitchThumb},
    tabs::{TabContent, TabTrigger, Tabs},
    toast::{ToastOptions, ToastProvider, use_toast},
    toggle_group::{ToggleGroup, ToggleItem},
    toolbar::{Toolbar, ToolbarButton, ToolbarSeparator},
    tooltip::{Tooltip, TooltipContent, TooltipSide, TooltipTrigger},
};



#[component]
pub(super) fn CalendarExample() -> Element {
    let mut selected_date = use_signal(|| None::<CalendarDate>);
    let mut view_date = use_signal(|| CalendarDate::new(2024, 5, 15));

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/calendar/style.css") }

        div { class: "calendar-example", style: "padding: 20px;",
            // Basic calendar
            div { class: "calendar",
                Calendar {
                    selected_date: selected_date(),
                    on_date_change: move |date| {
                        println!("Selected date: {:?}", date);
                        selected_date.set(date);
                    },
                    view_date: view_date(),
                    on_view_change: move |new_view: CalendarDate| {
                        println!("View changed to: {}-{}", new_view.year, new_view.month);
                        view_date.set(new_view);
                    },

                    CalendarHeader { CalendarNavigation {} }

                    CalendarGrid {}
                }
            }

            // Display selected date
            div { class: "selected-date", style: "margin-top: 20px;",
                if let Some(date) = selected_date() {
                    p { style: "font-weight: bold;", "Selected date: {date}" }
                } else {
                    p { style: "color: #666;", "No date selected" }
                }
            }
        }
    }
}
