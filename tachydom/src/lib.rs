#![allow(incomplete_features)] // yolo
#![cfg_attr(feature = "nightly", feature(adt_const_params))]

use wasm_bindgen::JsValue;
use web_sys::Node;

pub mod async_views;
pub mod dom;
pub mod html;
pub mod hydration;
pub mod renderer;
pub mod spawner;
pub mod ssr;
pub mod view;

#[cfg(all(feature = "leptos", not(feature = "reaccy")))]
mod leptos;

#[cfg(feature = "reaccy")]
mod tachy_reaccy;

pub fn log(text: &str) {
    web_sys::console::log_1(&JsValue::from_str(text));
}

pub(crate) trait UnwrapOrDebug {
    type Output;

    fn or_debug(self, el: &Node, label: &'static str);

    fn ok_or_debug(
        self,
        el: &Node,
        label: &'static str,
    ) -> Option<Self::Output>;
}

impl<T> UnwrapOrDebug for Result<T, JsValue> {
    type Output = T;

    #[track_caller]
    fn or_debug(self, el: &Node, name: &'static str) {
        #[cfg(debug_assertions)]
        {
            if let Err(err) = self {
                let location = std::panic::Location::caller();
                web_sys::console::warn_3(
                    &JsValue::from_str(&format!(
                        "[WARNING] Non-fatal error at {location}, while \
                         calling {name} on "
                    )),
                    el,
                    &err,
                );
            }
        }
        #[cfg(not(debug_assertions))]
        {
            _ = self;
        }
    }

    #[track_caller]
    fn ok_or_debug(
        self,
        el: &Node,
        name: &'static str,
    ) -> Option<Self::Output> {
        #[cfg(debug_assertions)]
        {
            if let Err(err) = &self {
                let location = std::panic::Location::caller();
                web_sys::console::warn_3(
                    &JsValue::from_str(&format!(
                        "[WARNING] Non-fatal error at {location}, while \
                         calling {name} on "
                    )),
                    el,
                    err,
                );
            }
            self.ok()
        }
        #[cfg(not(debug_assertions))]
        {
            self.ok()
        }
    }
}

#[macro_export]
macro_rules! or_debug {
    ($action:expr, $el:expr, $label:literal) => {
        if cfg!(debug_assertions) {
            $crate::UnwrapOrDebug::or_debug($action, $el, $label);
        } else {
            _ = $action;
        }
    };
}

#[macro_export]
macro_rules! ok_or_debug {
    ($action:expr, $el:expr, $label:literal) => {
        if cfg!(debug_assertions) {
            $crate::UnwrapOrDebug::ok_or_debug($action, $el, $label)
        } else {
            $action.ok()
        }
    };
}
