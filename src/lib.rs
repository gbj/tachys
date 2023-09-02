#![feature(adt_const_params)]

pub mod app;
use tracing_subscriber::fmt;
use tracing_subscriber_wasm::MakeConsoleWriter;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod dom;
pub mod html;
pub mod hydration;
pub mod view;

#[wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();

    /*     fmt()
    .with_writer(
        // To avoide trace events in the browser from showing their
        // JS backtrace, which is very annoying, in my opinion
        MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG),
    )
    // For some reason, if we don't do this in the browser, we get
    // a runtime error.
    .without_time()
    .init(); */

    //app::my_app().hydrate();
    app::my_app().client_render();
}
