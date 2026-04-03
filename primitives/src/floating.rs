//! Floating UI positioning hook for Dioxus components.
//!
//! Bridges the pure-Rust `floating-ui-core` engine with Dioxus by measuring
//! element rects via `document::eval` and applying the computed position as
//! inline styles.

use dioxus::prelude::*;

use floating_ui_core::compute_position::{compute_position, ComputePositionConfig};
use floating_ui_core::middleware::flip::Flip;
use floating_ui_core::middleware::offset::Offset;
use floating_ui_core::middleware::shift::Shift;
use floating_ui_core::middleware::Middleware;
use floating_ui_core::premeasured::{PreMeasured, PreMeasuredPlatform};
use floating_ui_core::types::*;

use crate::{ContentAlign, ContentSide};

/// The computed position of a floating element.
#[derive(Clone, Copy, Debug)]
pub struct FloatingPosition {
    /// The x coordinate (CSS `left` in pixels, viewport-relative).
    pub x: f64,
    /// The y coordinate (CSS `top` in pixels, viewport-relative).
    pub y: f64,
    /// The actual placement after middleware (may differ from requested due to flip).
    pub placement: Placement,
}

/// Convert `ContentSide` + `ContentAlign` to a floating-ui `Placement`.
pub(crate) fn to_placement(side: ContentSide, align: ContentAlign) -> Placement {
    let s = match side {
        ContentSide::Top => Side::Top,
        ContentSide::Right => Side::Right,
        ContentSide::Bottom => Side::Bottom,
        ContentSide::Left => Side::Left,
    };
    let a = match align {
        ContentAlign::Start => Some(Alignment::Start),
        ContentAlign::Center => None,
        ContentAlign::End => Some(Alignment::End),
    };
    Placement::from_side_alignment(s, a)
}

/// Convert a floating-ui `Placement` back to `ContentSide`.
pub(crate) fn placement_to_side(placement: Placement) -> ContentSide {
    match placement.side() {
        Side::Top => ContentSide::Top,
        Side::Right => ContentSide::Right,
        Side::Bottom => ContentSide::Bottom,
        Side::Left => ContentSide::Left,
    }
}

/// Convert a floating-ui `Placement` back to `ContentAlign`.
pub(crate) fn placement_to_align(placement: Placement) -> ContentAlign {
    match placement.alignment() {
        Some(Alignment::Start) => ContentAlign::Start,
        Some(Alignment::End) => ContentAlign::End,
        None => ContentAlign::Center,
    }
}

/// JavaScript that measures both elements and the viewport, then listens for
/// scroll/resize to re-measure. Sends `[ref_x, ref_y, ref_w, ref_h, float_w, float_h, vp_w, vp_h]`.
///
/// Protocol:
/// 1. Receives `triggerId` (string)
/// 2. Receives `contentId` (string)
/// 3. Sends measurement arrays whenever layout changes
/// 4. Receives any value as a cleanup/shutdown signal
const MEASURE_JS: &str = r#"
    let triggerId = await dioxus.recv();
    let contentId = await dioxus.recv();

    function measure() {
        let trigger = document.getElementById(triggerId);
        let content = document.getElementById(contentId);
        if (trigger && content) {
            let tRect = trigger.getBoundingClientRect();
            let cRect = content.getBoundingClientRect();
            dioxus.send([
                tRect.x, tRect.y, tRect.width, tRect.height,
                cRect.width, cRect.height,
                window.innerWidth, window.innerHeight
            ]);
            return true;
        }
        return false;
    }

    function onUpdate() { measure(); }

    function init() {
        if (measure()) {
            window.addEventListener('scroll', onUpdate, true);
            window.addEventListener('resize', onUpdate);
        } else {
            requestAnimationFrame(init);
        }
    }
    requestAnimationFrame(init);

    await dioxus.recv();

    window.removeEventListener('scroll', onUpdate, true);
    window.removeEventListener('resize', onUpdate);
"#;

/// No-op JS that waits for a cleanup signal and exits.
const NOOP_JS: &str = "await dioxus.recv();";

/// Hook that computes and continuously updates floating element positioning.
///
/// Uses `document::eval` to measure element rects in the browser, then runs
/// `floating-ui-core`'s positioning algorithm with offset, flip, and shift
/// middleware.
///
/// Returns a signal containing the computed position, or `None` if the
/// elements haven't been measured yet or the floating element is closed.
pub(crate) fn use_floating(
    trigger_id: impl Readable<Target = String> + Copy + 'static,
    content_id: impl Readable<Target = String> + Copy + 'static,
    side: ContentSide,
    align: ContentAlign,
    offset_value: f64,
    open: impl Readable<Target = bool> + Copy + 'static,
) -> Signal<Option<FloatingPosition>> {
    let mut position = use_signal(|| None::<FloatingPosition>);
    let placement = to_placement(side, align);

    crate::use_effect_with_cleanup(move || {
        let is_open = open.cloned();

        if !is_open {
            position.set(None);
        }

        // Always create an eval so the cleanup closure has the same type.
        // When not open, use a no-op that just waits for the cleanup signal.
        let mut eval = dioxus::document::eval(if is_open { MEASURE_JS } else { NOOP_JS });

        if is_open {
            let _ = eval.send(trigger_id.cloned());
            let _ = eval.send(content_id.cloned());

            spawn(async move {
                while let Ok(measurements) = eval.recv::<Vec<f64>>().await {
                    if measurements.len() < 8 {
                        continue;
                    }

                    let ref_x = measurements[0];
                    let ref_y = measurements[1];
                    let ref_w = measurements[2];
                    let ref_h = measurements[3];
                    let float_w = measurements[4];
                    let float_h = measurements[5];
                    let vp_w = measurements[6];
                    let vp_h = measurements[7];

                    let reference_rect = LayoutRect::new(
                        euclid::Point2D::new(ref_x, ref_y),
                        euclid::Size2D::new(ref_w, ref_h),
                    );
                    let floating_size = euclid::Size2D::new(float_w, float_h);
                    let clipping_rect = euclid::Box2D::new(
                        euclid::Point2D::new(0.0, 0.0),
                        euclid::Point2D::new(vp_w, vp_h),
                    );

                    let platform = PreMeasuredPlatform::new(
                        reference_rect,
                        floating_size,
                        clipping_rect,
                        false,
                    );

                    let middleware: Vec<Box<dyn Middleware<PreMeasured>>> = vec![
                        Box::new(Offset::new(offset_value)),
                        Box::new(Flip::<PreMeasured>::new()),
                        Box::new(Shift::<PreMeasured>::new()),
                    ];

                    let result = compute_position(
                        &PreMeasured,
                        &PreMeasured,
                        ComputePositionConfig::new(&platform)
                            .placement(placement)
                            .strategy(Strategy::Fixed)
                            .middleware(middleware),
                    );

                    position.set(Some(FloatingPosition {
                        x: result.x(),
                        y: result.y(),
                        placement: result.placement(),
                    }));
                }
            });
        }

        move || {
            let _ = eval.send(true);
        }
    });

    position
}
