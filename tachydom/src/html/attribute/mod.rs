mod key;
mod value;
use crate::view::{Position, ToTemplate};
pub use key::*;
use std::{fmt::Debug, marker::PhantomData};
pub use value::*;
use web_sys::Element;

pub trait Attribute {
    type State;

    fn to_html(&self, buf: &mut String);

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State;

    fn rebuild(self, state: &mut Self::State);
}

impl Attribute for () {
    type State = ();

    fn to_html(&self, _buf: &mut String) {}

    fn hydrate<const FROM_SERVER: bool>(self, _el: &Element) -> Self::State {}

    fn rebuild(self, state: &mut Self::State) {}
}

#[derive(Debug)]
pub struct Attr<K, V>(pub K, pub V)
where
    K: AttributeKey,
    V: AttributeValue;

impl<K, V> ToTemplate for Attr<K, V>
where
    K: AttributeKey,
    V: AttributeValue,
{
    fn to_template(buf: &mut String, _position: &mut Position) {
        V::to_template(K::KEY, buf);
    }
}

impl<K, V> Attribute for Attr<K, V>
where
    K: AttributeKey,
    V: AttributeValue,
{
    type State = V::State;

    fn to_html(&self, buf: &mut String) {
        self.1.to_html(K::KEY, buf);
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
        self.1.hydrate::<FROM_SERVER>(K::KEY, el)
    }

    fn rebuild(self, state: &mut Self::State) {
        V::rebuild(self.1, K::KEY, state);
    }
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

    fn to_html(&self, buf: &mut String) {
        V.to_html(K::KEY, buf)
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {}

    fn rebuild(self, state: &mut Self::State) {}
}

#[derive(Debug)]
pub struct StaticAttr<K: AttributeKey, const V: &'static str> {
    ty: PhantomData<K>,
}

pub fn static_attr<K: AttributeKey, const V: &'static str>() -> StaticAttr<K, V> {
    StaticAttr { ty: PhantomData }
}
