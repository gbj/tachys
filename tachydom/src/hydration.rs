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
        }
    }

    pub fn sibling(&self) {
        let mut inner = self.0.borrow_mut();
        if let Some(node) = inner.next_sibling() {
            *inner = node;
        }
    }

    pub fn parent(&self) {
        let mut inner = self.0.borrow_mut();
        if let Some(node) = inner.parent_node() {
            *inner = node;
        }
    }

    pub fn set(&self, node: Node) {
        *self.0.borrow_mut() = node;
    }
}
