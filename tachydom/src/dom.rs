use web_sys::{Document, HtmlElement, Window};

pub fn window() -> Window {
    web_sys::window().unwrap()
}

pub fn document() -> Document {
    window().document().unwrap()
}

pub fn body() -> HtmlElement {
    document().body().unwrap()
}

pub fn log(s: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(s));
}
