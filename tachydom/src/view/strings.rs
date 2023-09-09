use super::{Position, ToTemplate, View};
use crate::dom::document;
use crate::hydration::Cursor;
use wasm_bindgen::JsCast;
use web_sys::{Comment, Text};

impl<'a> View for &'a str {
    type State = (Text, &'a str);

    fn to_html(&self, buf: &mut String, position: &mut Position) {
        // add a comment node to separate from previous sibling, if any
        if matches!(position, Position::NextChild | Position::LastChild) {
            buf.push_str("<!>")
        }
        buf.push_str(self);
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &mut Cursor,
        position: &mut Position,
    ) -> Self::State {
        if *position == Position::FirstChild {
            cursor.child();
        } else {
            cursor.sibling();
        }
        let mut node = cursor.current().to_owned().unchecked_into::<Text>();

        if FROM_SERVER && matches!(*position, Position::NextChild | Position::LastChild) {
            cursor.sibling();
        }
        if !FROM_SERVER {
            let new = document().create_text_node(self);
            node.unchecked_ref::<Comment>()
                .replace_with_with_node_1(&new);
            node = new;
        }
        *position = Position::NextChild;

        (node, self)
    }

    fn rebuild(self, state: &mut Self::State) {
        let (node, prev) = state;
        if &self != prev {
            node.set_data(self);
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

impl View for String {
    type State = (Text, String);

    fn to_html(&self, buf: &mut String, position: &mut Position) {
        self.as_str().to_html(buf, position)
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &mut Cursor,
        position: &mut Position,
    ) -> Self::State {
        let (node, _) = self.as_str().hydrate::<FROM_SERVER>(cursor, position);
        (node, self)
    }

    fn rebuild(self, state: &mut Self::State) {
        let (node, prev) = state;
        if &self != prev {
            node.set_data(&self);
            *prev = self;
        }
    }
}

impl ToTemplate for String {
    fn to_template(buf: &mut String, position: &mut Position) {
        <&str as ToTemplate>::to_template(buf, position)
    }
}

#[derive(Debug)]
pub struct Static<const V: &'static str>;

impl<const V: &'static str> View for Static<V> {
    type State = ();

    fn to_html(&self, buf: &mut String, position: &mut Position) {
        // add a comment node to separate from previous sibling, if any
        if matches!(position, Position::NextChild | Position::LastChild) {
            buf.push_str("<!>")
        }
        buf.push_str(V)
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &mut Cursor,
        position: &mut Position,
    ) -> Self::State {
        if *position == Position::FirstChild {
            cursor.child();
        } else {
            cursor.sibling();
        }
        *position = Position::NextChild;
    }

    // This type is specified as static, so no rebuilding is done.
    fn rebuild(self, _state: &mut Self::State) {}
}

impl<const V: &'static str> ToTemplate for Static<V> {
    fn to_template(buf: &mut String, position: &mut Position) {
        if matches!(*position, Position::NextChild | Position::LastChild) {
            buf.push_str("<!>")
        }
        buf.push_str(V);
        *position = Position::NextChild;
    }
}
