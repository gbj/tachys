use web_sys::Element;

use crate::{
    html::attribute::{Attribute, AttributeKey, AttributeValue},
    hydration::Cursor,
};
use std::marker::PhantomData;

use super::{Position, ToTemplate, View};

#[derive(Debug)]
pub struct StaticAttr<K: AttributeKey, const V: &'static str> {
    ty: PhantomData<K>,
}

pub fn static_attr<K: AttributeKey, const V: &'static str>() -> StaticAttr<K, V> {
    StaticAttr { ty: PhantomData }
}

impl<K, const V: &'static str> ToTemplate for StaticAttr<K, V>
where
    K: AttributeKey,
{
    fn to_template(buf: &mut String, _position: &mut Position) {
        buf.push(' ');
        buf.push_str(K::KEY);
        buf.push_str("=\"");
        buf.push_str(V);
        buf.push('"');
    }
}

impl<K, const V: &'static str> Attribute for StaticAttr<K, V>
where
    K: AttributeKey,
{
    type State = ();

    fn to_html(&self, buf: &mut String, _class: &mut String, _style: &mut String) {
        AttributeValue::to_html(&V, K::KEY, buf)
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {}

    fn rebuild(self, state: &mut Self::State) {}
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
