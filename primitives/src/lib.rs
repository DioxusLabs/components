use dioxus_lib::{prelude::*, warnings::Warning};
use std::collections::HashMap;

pub mod accordion;
pub mod aspect_ratio;
pub mod separator;

/// Generate a runtime-unique id.
fn use_unique_id() -> Signal<String> {
    static NEXT_ID: GlobalSignal<usize> = Signal::global(|| 0);

    use_signal(|| {
        let id = *NEXT_ID.peek();
        *NEXT_ID.write() += 1;
        format!("dxc-{id}")
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PortalId(usize);

#[derive(Clone, Copy, PartialEq, Default)]
struct PortalCtx {
    portals: Signal<HashMap<usize, Element>>,
}

/// Create a portal.
pub fn use_portal() -> PortalId {
    static NEXT_ID: GlobalSignal<usize> = Signal::global(|| 0);

    let id = use_hook(|| {
        let id = *NEXT_ID.peek();
        *NEXT_ID.write() += 1;

        let mut ctx = match try_consume_context::<PortalCtx>() {
            Some(ctx) => ctx,
            None => {
                let ctx = PortalCtx::default();
                provide_root_context(ctx)
            }
        };

        ctx.portals.write().insert(id, Ok(VNode::placeholder()));

        PortalId(id)
    });

    // Cleanup the portal.
    use_drop(move || {
        let mut ctx = consume_context::<PortalCtx>();
        ctx.portals.write().remove(&id.0);
    });

    id
}

#[component]
pub fn PortalIn(portal: PortalId, children: Element) -> Element {
    if let Some(mut ctx) = try_use_context::<PortalCtx>() {
        dioxus_lib::signals::warnings::signal_write_in_component_body::allow(|| {
            ctx.portals.write().insert(portal.0, children);
        });
    }

    rsx! {}
}

#[component]
pub fn PortalOut(portal: PortalId) -> Element {
    if let Some(ctx) = try_use_context::<PortalCtx>() {
        if let Some(children) = ctx.portals.read().get(&portal.0) {
            return rsx! {
                {children}
            };
        }
    }

    rsx! {}
}
