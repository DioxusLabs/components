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
pub(super) fn DropdownMenuExample() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./src/dropdown_menu/style.css") }
        DropdownMenu { class: "dropdown-menu", default_open: false,

            DropdownMenuTrigger { class: "dropdown-menu-trigger", "Open Menu" }

            DropdownMenuContent { class: "dropdown-menu-content",

                DropdownMenuItem {
                    class: "dropdown-menu-item",
                    value: "item1".to_string(),
                    index: 0usize,
                    on_select: move |value| {
                        eval(&format!("console.log('Selected: {}')", value));
                    },
                    "Item 1"
                }

                DropdownMenuItem {
                    class: "dropdown-menu-item",
                    value: "item2".to_string(),
                    index: 1usize,
                    on_select: move |value| {
                        eval(&format!("console.log('Selected: {}')", value));
                    },
                    "Item 2"
                }

                DropdownMenuItem {
                    class: "dropdown-menu-item",
                    value: "item3".to_string(),
                    index: 2usize,
                    on_select: move |value| {
                        eval(&format!("console.log('Selected: {}')", value));
                    },
                    "Item 3"
                }
            }
        }
    }
}