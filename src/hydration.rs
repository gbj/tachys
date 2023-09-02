use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, Node};

#[derive(Debug)]
pub struct Cursor(Node);

impl Cursor {
    pub fn new(root: HtmlElement) -> Self {
        Self(root.unchecked_into())
    }

    pub fn current(&self) -> &Node {
        &self.0
    }

    pub fn child(&mut self) {
        if let Some(node) = self.0.first_child() {
            self.0 = node;
            web_sys::console::log_2(
                &wasm_bindgen::JsValue::from_str("Advanced to child"),
                &self.0,
            );
        } else {
            web_sys::console::error_2(
                &wasm_bindgen::JsValue::from_str("Could not get the first child of"),
                &self.0,
            );
        }
    }

    pub fn sibling(&mut self) {
        if let Some(node) = self.0.next_sibling() {
            self.0 = node;
            web_sys::console::log_2(
                &wasm_bindgen::JsValue::from_str("Advanced to sibling"),
                &self.0,
            );
        } else {
            web_sys::console::error_2(
                &wasm_bindgen::JsValue::from_str("Could not get the next sibling of"),
                &self.0,
            );
        }
    }

    pub fn parent(&mut self) {
        if let Some(node) = self.0.parent_node() {
            self.0 = node;
            web_sys::console::log_2(
                &wasm_bindgen::JsValue::from_str("Advanced to parent"),
                &self.0,
            );
        } else {
            web_sys::console::error_2(
                &wasm_bindgen::JsValue::from_str("Could not get the parent node of"),
                &self.0,
            );
        }
    }

    pub fn set(&mut self, node: Node) {
        self.0 = node;
        web_sys::console::log_2(
            &wasm_bindgen::JsValue::from_str("Set cursor back to "),
            &self.0,
        );
    }
}
