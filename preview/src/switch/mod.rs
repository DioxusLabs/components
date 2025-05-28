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
pub(super) fn SwitchExample() -> Element {
    let mut checked = use_signal(|| false);

    rsx! {
        div { class: "switch-example",
            label { "Airplane Mode" }
            Switch {
                class: "switch",
                checked,
                on_checked_change: move |new_checked| {
                    checked.set(new_checked);
                    eval(&format!("console.log('Switch toggled: {}')", new_checked));
                },

                SwitchThumb { class: "switch-thumb" }
            }
        }
    }
}
