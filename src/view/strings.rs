use super::{Position, View};
use crate::dom::document;
use crate::hydration::Cursor;
use wasm_bindgen::JsCast;
use web_sys::{Comment, Text};

impl<'a> View for &'a str {
    type State = ();

    fn to_html(&self, buf: &mut String, position: Position) {
        // add a comment node to separate from previous sibling, if any
        if matches!(position, Position::NextChild | Position::LastChild) {
            buf.push_str("<!>")
        }
        buf.push_str(self);
    }

    fn to_template(buf: &mut String, position: Position) -> Position {
        buf.push_str("<!>");
        Position::NextChild
    }

    fn hydrate<const IS_HYDRATING: bool>(
        self,
        cursor: &mut Cursor,
        position: Position,
    ) -> Position {
        crate::dom::log(&format!("hydrating {self}"));
        if position == Position::FirstChild {
            cursor.child();
        } else {
            cursor.sibling();
        }
        if IS_HYDRATING && matches!(position, Position::NextChild | Position::LastChild) {
            crate::dom::log("skipping <!>");
            cursor.sibling();
        }
        if !IS_HYDRATING {
            cursor
                .current()
                .unchecked_ref::<Comment>()
                .replace_with_with_node_1(&document().create_text_node(&self));
        }
        Position::NextChild
    }
}

impl View for String {
    type State = ();

    fn to_html(&self, buf: &mut String, position: Position) {
        self.as_str().to_html(buf, position)
    }

    fn to_template(buf: &mut String, position: Position) -> Position {
        <&str as View>::to_template(buf, position)
    }

    fn hydrate<const IS_HYDRATING: bool>(
        self,
        cursor: &mut Cursor,
        position: Position,
    ) -> Position {
        self.as_str().hydrate::<IS_HYDRATING>(cursor, position)
    }
}

#[derive(Debug)]
pub struct Static<const V: &'static str>;

impl<const V: &'static str> View for Static<V> {
    type State = ();

    fn to_html(&self, buf: &mut String, position: Position) {
        // add a comment node to separate from previous sibling, if any
        if matches!(position, Position::NextChild | Position::LastChild) {
            buf.push_str("<!>")
        }
        buf.push_str(V)
    }

    fn to_template(buf: &mut String, position: Position) -> Position {
        if matches!(position, Position::NextChild | Position::LastChild) {
            buf.push_str("<!>")
        }
        buf.push_str(V);
        Position::NextChild
    }

    fn hydrate<const IS_HYDRATING: bool>(
        self,
        cursor: &mut Cursor,
        position: Position,
    ) -> Position {
        if position == Position::FirstChild {
            cursor.child();
        } else {
            cursor.sibling();
        }
        Position::NextChild
    }
}
