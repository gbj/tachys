use once_cell::unsync::Lazy;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement, Node, Window};

pub fn window() -> Window {
    web_sys::window().unwrap()
}

pub fn document() -> Document {
    window().document().unwrap()
}

pub fn body() -> HtmlElement {
    document().body().unwrap()
}

pub fn comment() -> Node {
    thread_local! {
        static COMMENT: Lazy<Node> = Lazy::new(|| {
            document().create_comment("").unchecked_into()
        });
    }
    COMMENT.with(|n| n.clone_node().unwrap())
}

pub fn log(s: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(s));
}
