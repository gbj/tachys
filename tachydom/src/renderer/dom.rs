use super::Renderer;
use crate::{dom::document, html::element::ElementType, ok_or_debug, or_debug};
use wasm_bindgen::intern;

pub struct Dom;

impl Renderer for Dom {
    type Node = web_sys::Node;
    type Text = web_sys::Text;
    type Element = web_sys::Element;

    fn create_element<E: ElementType>() -> Self::Element {
        E::create_element()
    }

    fn create_text_node(text: &str) -> Self::Text {
        document().create_text_node(text)
    }

    fn set_text(node: &Self::Text, text: &str) {
        node.set_data(text);
    }

    fn set_attribute(node: &Self::Element, name: &str, value: &str) {
        or_debug!(
            node.set_attribute(intern(name), value),
            node,
            "setAttribute"
        );
    }

    fn remove_attribute(node: &Self::Element, name: &str) {
        or_debug!(node.remove_attribute(intern(name)), node, "removeAttribute");
    }

    fn insert_node(
        parent: &Self::Element,
        new_child: &Self::Node,
        anchor: Option<&Self::Node>,
    ) {
        ok_or_debug!(
            parent.insert_before(new_child, anchor),
            parent,
            "insertNode"
        );
    }

    fn remove_node(
        parent: &Self::Element,
        child: &Self::Node,
    ) -> Option<Self::Node> {
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
