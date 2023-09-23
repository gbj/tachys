use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, Node};

#[derive(Debug, Clone)]
pub struct Cursor(Rc<RefCell<Node>>);

impl Cursor {
    pub fn new(root: HtmlElement) -> Self {
        Self(Rc::new(RefCell::new(root.unchecked_into())))
    }

    pub fn current(&self) -> Node {
        self.0.borrow().clone()
    }

    pub fn child(&self) {
        let mut inner = self.0.borrow_mut();
        if let Some(node) = inner.first_child() {
            *inner = node;
            web_sys::console::log_2(
                &wasm_bindgen::JsValue::from_str("Advanced to child"),
                &inner,
            );
        } else {
            web_sys::console::error_2(
                &wasm_bindgen::JsValue::from_str("Could not get the first child of"),
                &inner,
            );
        }
    }

    pub fn sibling(&self) {
        let mut inner = self.0.borrow_mut();
        web_sys::console::log_2(
            &wasm_bindgen::JsValue::from_str("Going to sibling of"),
            &inner,
        );
        if let Some(node) = inner.next_sibling() {
            *inner = node;
            web_sys::console::log_2(
                &wasm_bindgen::JsValue::from_str("  Advanced to sibling"),
                &inner,
            );
        } else {
            web_sys::console::error_2(
                &wasm_bindgen::JsValue::from_str("  Could not get the next sibling of"),
                &inner,
            );
        }
    }

    pub fn parent(&self) {
        let mut inner = self.0.borrow_mut();
        if let Some(node) = inner.parent_node() {
            *inner = node;
            web_sys::console::log_2(
                &wasm_bindgen::JsValue::from_str("Advanced to parent"),
                &inner,
            );
        } else {
            web_sys::console::error_2(
                &wasm_bindgen::JsValue::from_str("Could not get the parent node of"),
                &inner,
            );
        }
    }

    pub fn set(&self, node: Node) {
        web_sys::console::log_2(
            &wasm_bindgen::JsValue::from_str("Set cursor back to "),
            &node,
        );
        *self.0.borrow_mut() = node;
    }
}
