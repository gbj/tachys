use gtk::{
    prelude::{Cast, WidgetExt},
    Label, Widget,
};
use tachydom::{
    renderer::{CastFrom, Renderer},
    view::{Mountable, Render},
};

pub struct TachyGtk;

#[derive(Clone)]
pub struct Element(pub Widget);
pub struct Text(pub Element);

impl<T> From<T> for Element
where
    T: Into<Widget>,
{
    fn from(value: T) -> Self {
        Element(value.into())
    }
}

impl Mountable<TachyGtk> for Element {
    fn unmount(&mut self) {
        todo!()
    }

    fn mount(
        &mut self,
        parent: &<TachyGtk as Renderer>::Element,
        marker: Option<&<TachyGtk as Renderer>::Node>,
    ) {
        self.0
            .insert_before(&parent.0, marker.as_ref().map(|m| &m.0));
    }
}

impl Mountable<TachyGtk> for Text {
    fn unmount(&mut self) {
        todo!()
    }

    fn mount(
        &mut self,
        parent: &<TachyGtk as Renderer>::Element,
        marker: Option<&<TachyGtk as Renderer>::Node>,
    ) {
        self.0
             .0
            .insert_before(&parent.0, marker.as_ref().map(|m| &m.0));
    }
}

impl CastFrom<Element> for Element {
    fn cast_from(source: Element) -> Option<Self> {
        todo!()
    }
}

impl CastFrom<Element> for Text {
    fn cast_from(source: Element) -> Option<Self> {
        todo!()
    }
}

impl AsRef<Element> for Element {
    fn as_ref(&self) -> &Element {
        self
    }
}

impl AsRef<Element> for Text {
    fn as_ref(&self) -> &Element {
        &self.0
    }
}

impl Renderer for TachyGtk {
    type Node = Element;
    type Element = Element;
    type Text = Text;
    type Placeholder = Element;

    fn create_text_node(text: &str) -> Self::Text {
        Text(Element::from(Label::new(Some(text))))
    }

    fn create_placeholder() -> Self::Placeholder {
        todo!()
    }

    fn set_text(node: &Self::Text, text: &str) {
        let node_as_text = node.0 .0.downcast_ref::<Label>().unwrap();
        node_as_text.set_label(text);
    }

    fn set_attribute(node: &Self::Element, name: &str, value: &str) {
        todo!()
    }

    fn remove_attribute(node: &Self::Element, name: &str) {
        todo!()
    }

    fn insert_node(
        parent: &Self::Element,
        new_child: &Self::Node,
        marker: Option<&Self::Node>,
    ) {
        new_child
            .0
            .insert_before(&parent.0, marker.as_ref().map(|n| &n.0));
    }

    fn replace_node(old: &Self::Node, new: &Self::Node) {
        todo!()
    }

    fn remove_node(
        parent: &Self::Element,
        child: &Self::Node,
    ) -> Option<Self::Node> {
        todo!()
    }

    fn remove(node: &Self::Node) {
        todo!()
    }

    fn get_parent(node: &Self::Node) -> Option<Self::Node> {
        todo!()
    }

    fn first_child(node: &Self::Node) -> Option<Self::Node> {
        todo!()
    }

    fn next_sibling(node: &Self::Node) -> Option<Self::Node> {
        todo!()
    }

    fn log_node(node: &Self::Node) {
        todo!()
    }
}

#[derive(Clone)]
pub struct Button<C, F>(C, F)
where
    C: Render<TachyGtk>,
    F: Fn(&gtk::Button) + 'static;

pub fn button<C, F>(children: C, on_click: F) -> Button<C, F>
where
    C: Render<TachyGtk>,
    F: Fn(&gtk::Button) + 'static,
{
    Button(children, on_click)
}

impl<C, F> Render<TachyGtk> for Button<C, F>
where
    C: Render<TachyGtk>,
    F: Fn(&gtk::Button) + 'static,
{
    type State = ElementState<C::State>;

    fn build(self) -> Self::State {
        use gtk::prelude::ButtonExt;

        let Button(children, on_click) = self;
        let button = gtk::Button::new();
        let handler = button.connect_clicked(on_click);
        let button = Element::from(button);
        let mut children = children.build();
        children.mount(&button, None);
        ElementState(button, children)
    }

    fn rebuild(self, state: &mut Self::State) {
        todo!()
    }
}

pub struct ElementState<C>(pub Element, C)
where
    C: Mountable<TachyGtk>;

impl<C> Mountable<TachyGtk> for ElementState<C>
where
    C: Mountable<TachyGtk>,
{
    fn unmount(&mut self) {
        todo!()
    }

    fn mount(
        &mut self,
        parent: &<TachyGtk as Renderer>::Element,
        marker: Option<&<TachyGtk as Renderer>::Node>,
    ) {
        TachyGtk::insert_node(parent, &self.0, marker);
    }
}
