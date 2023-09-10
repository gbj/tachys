#![feature(adt_const_params)]

pub mod dom;
pub mod html;
pub mod hydration;
pub mod view;

pub fn log(text: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(text));
}
