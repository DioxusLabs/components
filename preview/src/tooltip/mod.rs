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
pub(super) fn TooltipExample() -> Element {
    rsx! {
        div {
            class: "tooltip-example",
            style: "padding: 50px; display: flex; gap: 20px;",
            // Basic tooltip
            Tooltip { class: "tooltip",
                TooltipTrigger { class: "tooltip-trigger",
                    button { "Hover me" }
                }
                TooltipContent { class: "tooltip-content", "This is a basic tooltip" }
            }
            // Tooltip with different position
            Tooltip { class: "tooltip",
                TooltipTrigger { class: "tooltip-trigger",
                    button { "Right tooltip" }
                }
                TooltipContent { class: "tooltip-content", side: TooltipSide::Right,
                    "This tooltip appears on the right"
                }
            }
            // Tooltip with HTML content
            Tooltip { class: "tooltip",
                TooltipTrigger { class: "tooltip-trigger",
                    button { "Rich content" }
                }
                TooltipContent { class: "tooltip-content", style: "width: 200px;",
                    h4 { style: "margin-top: 0; margin-bottom: 8px;", "Tooltip title" }
                    p { style: "margin: 0;", "This tooltip contains rich HTML content with styling." }
                }
            }
        }
    }
}
