#![feature(type_name_of_val)]

pub mod app;
use tachy_reaccy::Root;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(target_arch = "wasm32")]
extern crate wee_alloc;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn hydrate() {
    use tachydom::{dom::body, view::RenderHtml};
    console_error_panic_hook::set_once();
    Root::global_hydrate(|| {
        let root = crate::app::my_app();
        let state = root.hydrate_from::<true>(&body());
        std::mem::forget(state);
    });
}
