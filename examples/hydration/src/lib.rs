#![feature(type_name_of_val)]

pub mod app;
use tachy_reaccy::global_root;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    use tachydom::{dom::body, view::RenderHtml};
    console_error_panic_hook::set_once();
    global_root(|| {
        let root = crate::app::my_app();
        let state = root.hydrate_from::<true>(&body());
        std::mem::forget(state);
    });
}
