use super::Renderer;
use crate::{
    dom::document,
    html::element::{CreateElement, ElementType},
    ok_or_debug, or_debug,
    view::Mountable,
};
use once_cell::unsync::Lazy;
use wasm_bindgen::{intern, JsCast};
use web_sys::Element;

pub struct Dom;

impl Renderer for Dom {
    type Node = web_sys::Node;
    type Text = web_sys::Text;
    type Element = web_sys::Element;
    type Fragment = web_sys::DocumentFragment;

    fn create_element<E: ElementType + CreateElement<Dom>>() -> Self::Element {
        E::create_element()
    }

    fn create_fragment() -> Self::Fragment {
        document().create_document_fragment()
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

    fn replace_node(old: &Self::Node, new: &Self::Node) {
        or_debug!(
            old.unchecked_ref::<Element>().replace_with_with_node_1(new),
            old,
            "replaceWith"
        );
    }
}

impl<E: ElementType> CreateElement<Dom> for E {
    fn create_element() -> <Dom as Renderer>::Element {
        thread_local! {
            static ELEMENT: Lazy<<Dom as Renderer>::Element> = Lazy::new(|| {
                document().create_element(stringify!($tag)).unwrap()
            });
        }
        ELEMENT.with(|e| e.clone_node()).unwrap().unchecked_into()
    }
}

impl Mountable<Dom> for web_sys::Node {
    fn unmount(&mut self) {
        todo!()
    }

    fn as_mountable(&self) -> Option<<Dom as Renderer>::Node> {
        Some(self.clone())
    }
}

impl Mountable<Dom> for web_sys::Text {
    fn unmount(&mut self) {
        todo!()
    }

    fn as_mountable(&self) -> Option<<Dom as Renderer>::Node> {
        Some(self.unchecked_ref::<web_sys::Node>().clone())
    }
}

impl Mountable<Dom> for web_sys::Element {
    fn unmount(&mut self) {
        todo!()
    }

    fn as_mountable(&self) -> Option<<Dom as Renderer>::Node> {
        Some(self.unchecked_ref::<web_sys::Node>().clone())
    }
}

impl Mountable<Dom> for web_sys::DocumentFragment {
    fn unmount(&mut self) {
        todo!()
    }

    fn as_mountable(&self) -> Option<<Dom as Renderer>::Node> {
        Some(self.unchecked_ref::<web_sys::Node>().clone())
    }
}
