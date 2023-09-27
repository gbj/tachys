use super::{Mountable, Position, PositionState, Render, ToTemplate};
use crate::hydration::Cursor;
use crate::renderer::Renderer;
use crate::{dom::document, renderer::dom::Dom};
use wasm_bindgen::JsCast;
use web_sys::{Comment, Node, Text};

impl<'a> Render for &'a str {
    type State = (Text, &'a str);

    fn to_html(&mut self, buf: &mut String, position: &PositionState) {
        // add a comment node to separate from previous sibling, if any
        if matches!(position.get(), Position::NextChild | Position::LastChild) {
            buf.push_str("<!>")
        }
        buf.push_str(self);
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        if position.get() == Position::FirstChild {
            cursor.child();
        } else {
            cursor.sibling();
        }
        let mut node = cursor.current().to_owned().unchecked_into::<Text>();

        if FROM_SERVER && matches!(position.get(), Position::NextChild | Position::LastChild) {
            cursor.sibling();
        }
        if !FROM_SERVER {
            let new = document().create_text_node(self);
            node.unchecked_ref::<Comment>()
                .replace_with_with_node_1(&new);
            node = new;
        }
        position.set(Position::NextChild);

        (node, self)
    }

    fn build(self) -> Self::State {
        let node = document().create_text_node(self);
        (node, self)
    }

    fn rebuild(self, state: &mut Self::State) {
        let (node, prev) = state;
        if &self != prev {
            Dom::set_text(node, self);
            *prev = self;
        }
    }
}

impl<'a> ToTemplate for &'a str {
    fn to_template(buf: &mut String, position: &mut Position) {
        buf.push_str("<!>");
        *position = Position::NextChild;
    }
}

impl Render for String {
    type State = (Text, String);

    fn to_html(&mut self, buf: &mut String, position: &PositionState) {
        self.as_str().to_html(buf, position)
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        let (node, _) = self.as_str().hydrate::<FROM_SERVER>(cursor, position);
        (node, self)
    }

    fn build(self) -> Self::State {
        let node = document().create_text_node(&self);
        (node, self)
    }

    fn rebuild(self, state: &mut Self::State) {
        let (node, prev) = state;
        if &self != prev {
            Dom::set_text(node, &self);
            *prev = self;
        }
    }
}

impl ToTemplate for String {
    fn to_template(buf: &mut String, position: &mut Position) {
        <&str as ToTemplate>::to_template(buf, position)
    }
}

impl Mountable for Text {
    fn unmount(&mut self) {
        self.remove()
    }

    fn as_mountable(&self) -> Option<Node> {
        Some(self.clone().unchecked_into())
    }
}

impl Mountable for (Text, String) {
    fn unmount(&mut self) {
        self.0.unmount()
    }

    fn as_mountable(&self) -> Option<Node> {
        self.0.as_mountable()
    }
}

impl<'a> Mountable for (Text, &'a str) {
    fn unmount(&mut self) {
        self.0.unmount()
    }

    fn as_mountable(&self) -> Option<Node> {
        self.0.as_mountable()
    }
}
