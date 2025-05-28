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
pub(super) fn TabsExample() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/tabs/style.css") }
        Tabs { class: "tabs", default_value: "tab1".to_string(),

            div { class: "tabs-list",
                TabTrigger {
                    class: "tabs-trigger",
                    value: "tab1".to_string(),
                    index: 0usize,
                    "Tab 1"
                }
                TabTrigger {
                    class: "tabs-trigger",
                    value: "tab2".to_string(),
                    index: 1usize,
                    "Tab 2"
                }
                TabTrigger {
                    class: "tabs-trigger",
                    value: "tab3".to_string(),
                    index: 2usize,
                    "Tab 3"
                }
            }

            TabContent { class: "tabs-content", value: "tab1".to_string(), "Tab 1 Content" }
            TabContent { class: "tabs-content", value: "tab2".to_string(), "Tab 2 Content" }
            TabContent { class: "tabs-content", value: "tab3".to_string(), "Tab 3 Content" }
        }
    }
}
