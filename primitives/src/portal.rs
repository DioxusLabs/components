use dioxus_lib::{prelude::*, warnings::Warning};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PortalId(usize);

#[derive(Clone, Copy, PartialEq)]
struct PortalCtx {
    portals: Signal<HashMap<usize, Signal<Element>>>,
}

/// Create a portal.
pub fn use_portal() -> PortalId {
    static NEXT_ID: GlobalSignal<usize> = Signal::global(|| 0);

    let (sig, id) = use_hook(|| {
        let mut next_id = NEXT_ID.write();
        let id = *next_id;
        *next_id += 1;

        let mut ctx = match try_consume_context::<PortalCtx>() {
            Some(ctx) => ctx,
            None => {
                let portals = Signal::new_in_scope(HashMap::new(), ScopeId::ROOT);
                let ctx = PortalCtx { portals };
                provide_root_context(ctx)
            }
        };

        let sig = Signal::new_in_scope(Ok(VNode::placeholder()), ScopeId::ROOT);
        ctx.portals.write().insert(id, sig);

        (sig, PortalId(id))
    });

    // Cleanup the portal.
    use_drop(move || {
        let mut ctx = consume_context::<PortalCtx>();
        ctx.portals.write().remove(&id.0);
        sig.manually_drop();
    });

    id
}

#[component]
pub fn PortalIn(portal: PortalId, children: Element) -> Element {
    if let Some(mut ctx) = try_use_context::<PortalCtx>() {
        dioxus_lib::signals::warnings::signal_write_in_component_body::allow(|| {
            let mut portals = ctx.portals.write();
            if let Some(portal) = portals.get_mut(&portal.0) {
                portal.set(children);
            }
        });
    }

    rsx! {}
}

#[component]
pub fn PortalOut(portal: PortalId) -> Element {
    if let Some(ctx) = try_use_context::<PortalCtx>() {
        if let Some(children) = ctx.portals.peek().get(&portal.0) {
            return rsx! {
                {children}
            };
        }
    }

    rsx! {}
}
