use wasm_bindgen::{intern, JsCast};
use web_sys::{Element, Text};

use super::Renderer;
use crate::{dom::document, html::element::ElementType, ok_or_debug, or_debug};

pub struct Dom;

impl Renderer for Dom {
    type Node = web_sys::Node;

    fn create_element<E: ElementType>() -> Self::Node {
        E::create_element().into()
    }

    fn create_text_node(text: &str) -> Self::Node {
        document().create_text_node(text).into()
    }

    fn set_text(node: &Self::Node, text: &str) {
        node.unchecked_ref::<Text>().set_data(text);
    }

    fn set_attribute(node: &Self::Node, name: &str, value: &str) {
        or_debug!(
            node.unchecked_ref::<Element>()
                .set_attribute(intern(name), value),
            node,
            "setAttribute"
        );
    }

    fn remove_attribute(node: &Self::Node, name: &str) {
        or_debug!(
            node.unchecked_ref::<Element>()
                .remove_attribute(intern(name)),
            node,
            "removeAttribute"
        );
    }

    fn insert_node(
        parent: &Self::Node,
        new_child: &Self::Node,
        anchor: Option<&Self::Node>,
    ) -> Option<Self::Node> {
        ok_or_debug!(
            parent.insert_before(new_child, anchor),
            parent,
            "insertNode"
        )
    }

    fn remove_node(parent: &Self::Node, child: &Self::Node) -> Option<Self::Node> {
        ok_or_debug!(parent.remove_child(child), parent, "removeNode")
    }

    fn get_parent(node: &Self::Node) -> Option<Self::Node> {
        node.parent_node()
    }

    fn first_child(node: &Self::Node) -> Option<Self::Node> {
        node.first_child()
    }

    fn next_sibling(node: &Self::Node) -> Option<Self::Node> {
        node.next_sibling()
    }
}
