use crate::html::element::ElementType;

pub mod dom;
#[cfg(feature = "testing")]
pub mod mock_dom;

/// Implements the instructions necessary to render an interface on some platform.
/// By default, this is implemented for the Document Object Model (DOM) in a Web
/// browser, but implementing this trait for some other platform allows you to use
/// the library to render any tree-based UI.
pub trait Renderer {
    /// The basic type of node in the view tree.
    type Node;
    /// A visible element in the view tree.
    type Element;
    /// A text node in the view tree.
    type Text;

    /// Creates a new element node.
    fn create_element<E: ElementType>() -> Self::Element;

    /// Creates a new text node.
    fn create_text_node(text: &str) -> Self::Text;

    /// Sets the text content of the node. If it's not a text node, this does nothing.
    fn set_text(node: &Self::Text, text: &str);

    /// Sets the given attribute on the given node by key and value.
    fn set_attribute(node: &Self::Element, name: &str, value: &str);

    /// Removes the given attribute on the given node.
    fn remove_attribute(node: &Self::Element, name: &str);

    /// Appends the new child to the parent, before the anchor node. If `anchor` is `None`,
    /// append to the end of the parent's children.
    fn insert_node(
        parent: &Self::Element,
        new_child: &Self::Node,
        anchor: Option<&Self::Node>,
    );

    /// Removes the child node from the parents, and returns the removed node.
    fn remove_node(
        parent: &Self::Element,
        child: &Self::Node,
    ) -> Option<Self::Node>;

    /// Gets the parent of the given node, if any.
    fn get_parent(node: &Self::Node) -> Option<Self::Node>;

    /// Returns the first child node of the given node, if any.
    fn first_child(node: &Self::Node) -> Option<Self::Node>;

    /// Returns the next sibling of the given node, if any.
    fn next_sibling(node: &Self::Node) -> Option<Self::Node>;
}
