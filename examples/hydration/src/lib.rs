#![feature(type_name_of_val)]

pub mod app;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    use tachydom::dom::body;
    use tachydom::view::View;
    console_error_panic_hook::set_once();
    crate::app::my_app().hydrate_from::<true>(&body());
}
