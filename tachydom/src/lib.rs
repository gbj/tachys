#![cfg_attr(feature = "nightly", feature(adt_const_params))]

use wasm_bindgen::JsValue;
use web_sys::Element;

pub mod dom;
pub mod html;
pub mod hydration;
pub mod view;

pub fn log(text: &str) {
    web_sys::console::log_1(&JsValue::from_str(text));
}

pub(crate) trait UnwrapOrDebug {
    type Output;

    fn or_debug(self, el: &Element, label: &'static str) -> Self::Output;
}

impl UnwrapOrDebug for Result<(), JsValue> {
    type Output = ();

    #[track_caller]
    fn or_debug(self, el: &Element, name: &'static str) -> Self::Output {
        #[cfg(debug_assertions)]
        {
            if let Err(err) = self {
                let location = std::panic::Location::caller();
                web_sys::console::warn_3(
                    &JsValue::from_str(&format!(
                        "[WARNING] Non-fatal error at {location}, while calling {name} on "
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
