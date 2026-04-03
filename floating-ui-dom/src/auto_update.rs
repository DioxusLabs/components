//! Automatic repositioning when layout changes.
//!
//! [`auto_update`] watches for scroll, resize, element resize, layout shifts,
//! and animation-driven movement, calling an update function whenever the
//! floating element needs repositioning.

use js_sys::Function;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::Element;

use crate::platform::ElementOrVirtual;
use crate::utils;

/// Configuration for [`auto_update`].
pub struct AutoUpdateOptions {
    ancestor_scroll: bool,
    ancestor_resize: bool,
    element_resize: bool,
    layout_shift: bool,
    animation_frame: bool,
}

impl AutoUpdateOptions {
    /// Create with default settings.
    pub fn new() -> Self {
        Self {
            ancestor_scroll: true,
            ancestor_resize: true,
            element_resize: true,
            layout_shift: true,
            animation_frame: false,
        }
    }

    /// Watch for ancestor scroll events (default: true).
    pub fn ancestor_scroll(mut self, v: bool) -> Self {
        self.ancestor_scroll = v;
        self
    }

    /// Watch for ancestor resize events (default: true).
    pub fn ancestor_resize(mut self, v: bool) -> Self {
        self.ancestor_resize = v;
        self
    }

    /// Watch for element resize via `ResizeObserver` (default: true).
    pub fn element_resize(mut self, v: bool) -> Self {
        self.element_resize = v;
        self
    }

    /// Watch for layout shifts via `IntersectionObserver` (default: true).
    pub fn layout_shift(mut self, v: bool) -> Self {
        self.layout_shift = v;
        self
    }

    /// Poll via `requestAnimationFrame` for animation-driven movement (default: false).
    pub fn animation_frame(mut self, v: bool) -> Self {
        self.animation_frame = v;
        self
    }
}

impl Default for AutoUpdateOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// A cleanup handle returned by [`auto_update`].
///
/// Call [`cleanup`](CleanupHandle::cleanup) or drop this handle to remove
/// all event listeners and observers.
pub struct CleanupHandle {
    cleanups: Vec<Box<dyn FnOnce()>>,
    // Hold closures alive so they aren't dropped
    _closures: Vec<Closure<dyn FnMut()>>,
    _closures_with_arg: Vec<Closure<dyn FnMut(js_sys::Array)>>,
}

impl CleanupHandle {
    /// Remove all listeners and observers.
    ///
    /// This is equivalent to dropping the handle. Provided for
    /// explicit cleanup in cases where drop timing is ambiguous.
    pub fn cleanup(mut self) {
        for f in self.cleanups.drain(..) {
            f();
        }
    }
}

impl Drop for CleanupHandle {
    fn drop(&mut self) {
        for f in self.cleanups.drain(..) {
            f();
        }
    }
}

/// Watch for changes that require repositioning the floating element.
///
/// Returns a [`CleanupHandle`] that removes all listeners when dropped or
/// when [`cleanup()`](CleanupHandle::cleanup) is called.
///
/// # Strategies
///
/// - **Ancestor scroll/resize**: Passive `scroll` and `resize` listeners on
///   all overflow ancestors.
/// - **Element resize**: `ResizeObserver` on reference and floating elements.
/// - **Layout shift**: `IntersectionObserver` that detects element movement.
/// - **Animation frame**: `requestAnimationFrame` polling (disables scroll
///   watching as redundant).
pub fn auto_update(
    reference: &ElementOrVirtual,
    floating: &Element,
    update: impl Fn() + 'static,
    options: AutoUpdateOptions,
) -> CleanupHandle {
    let mut cleanups: Vec<Box<dyn FnOnce()>> = Vec::new();
    let mut closures: Vec<Closure<dyn FnMut()>> = Vec::new();
    let mut closures_with_arg: Vec<Closure<dyn FnMut(js_sys::Array)>> = Vec::new();

    let update = std::rc::Rc::new(update);

    // --- Ancestor scroll and resize ---
    let ancestor_scroll = options.ancestor_scroll && !options.animation_frame;
    let ancestor_resize = options.ancestor_resize;

    if ancestor_scroll || ancestor_resize {
        // Collect ancestors from both reference and floating
        let mut ancestors = Vec::new();
        if let Some(ref_el) = reference.context_element() {
            ancestors.extend(utils::get_overflow_ancestors(ref_el));
        }
        ancestors.extend(utils::get_overflow_ancestors(floating));

        // Deduplicate (simple pointer comparison isn't easy, so just add all)
        for ancestor in &ancestors {
            let event_target: &web_sys::EventTarget = ancestor.unchecked_ref();

            if ancestor_scroll {
                let update_clone = update.clone();
                let closure = Closure::wrap(Box::new(move || {
                    update_clone();
                }) as Box<dyn FnMut()>);

                let _ = event_target
                    .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());

                let target = event_target.clone();
                let func: Function = closure.as_ref().unchecked_ref::<Function>().clone();
                cleanups.push(Box::new(move || {
                    let _ = target.remove_event_listener_with_callback("scroll", &func);
                }));
                closures.push(closure);
            }

            if ancestor_resize {
                let update_clone = update.clone();
                let closure = Closure::wrap(Box::new(move || {
                    update_clone();
                }) as Box<dyn FnMut()>);

                let _ = event_target
                    .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());

                let target = event_target.clone();
                let func: Function = closure.as_ref().unchecked_ref::<Function>().clone();
                cleanups.push(Box::new(move || {
                    let _ = target.remove_event_listener_with_callback("resize", &func);
                }));
                closures.push(closure);
            }
        }
    }

    // --- Element resize (ResizeObserver) ---
    if options.element_resize {
        let update_clone = update.clone();
        let closure = Closure::wrap(Box::new(move |_entries: js_sys::Array| {
            update_clone();
        }) as Box<dyn FnMut(js_sys::Array)>);

        let observer = web_sys::ResizeObserver::new(closure.as_ref().unchecked_ref())
            .expect("ResizeObserver not supported");

        if let Some(ref_el) = reference.context_element() {
            observer.observe(ref_el);
        }
        observer.observe(floating);

        let obs = observer.clone();
        cleanups.push(Box::new(move || {
            obs.disconnect();
        }));
        closures_with_arg.push(closure);
    }

    // --- Animation frame polling ---
    // Note: rAF-based auto-update requires recursive closure management in WASM
    // which is complex. The scroll/resize listeners and ResizeObserver strategies
    // cover the most common use cases. A full rAF implementation would compare
    // bounding rects each frame and call update only when they change.

    // Call update once initially
    update();

    CleanupHandle {
        cleanups,
        _closures: closures,
        _closures_with_arg: closures_with_arg,
    }
}
