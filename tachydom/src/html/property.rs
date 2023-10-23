use super::attribute::Attribute;
use crate::{
    renderer::DomRenderer,
    view::{Position, ToTemplate},
};
use std::{borrow::Cow, marker::PhantomData};
use wasm_bindgen::JsValue;

#[inline(always)]
pub fn property<P, R>(
    key: impl Into<Cow<'static, str>>,
    value: P,
) -> Property<P, R>
where
    P: IntoProperty<R>,
    R: DomRenderer,
{
    Property {
        key: key.into(),
        value,
        rndr: PhantomData,
    }
}

pub struct Property<P, R>
where
    P: IntoProperty<R>,
    R: DomRenderer,
{
    key: Cow<'static, str>,
    value: P,
    rndr: PhantomData<R>,
}

impl<P, R> Attribute<R> for Property<P, R>
where
    P: IntoProperty<R>,
    R: DomRenderer,
{
    type State = P::State;

    fn to_html(
        self,
        _buf: &mut String,
        class: &mut String,
        _style: &mut String,
    ) {
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &R::Element) -> Self::State {
        self.value.hydrate::<FROM_SERVER>(el, &self.key)
    }

    fn build(self, el: &R::Element) -> Self::State {
        self.value.build(el, &self.key)
    }

    fn rebuild(self, state: &mut Self::State) {
        self.value.rebuild(state, &self.key)
    }
}

impl<P, R> ToTemplate for Property<P, R>
where
    P: IntoProperty<R>,
    R: DomRenderer,
{
    fn to_template(
        buf: &mut String,
        class: &mut String,
        style: &mut String,
        position: &mut Position,
    ) {
    }
}

pub trait IntoProperty<R: DomRenderer> {
    type State;

    fn hydrate<const FROM_SERVER: bool>(
        self,
        el: &R::Element,
        key: &str,
    ) -> Self::State;

    fn build(self, el: &R::Element, key: &str) -> Self::State;

    fn rebuild(self, state: &mut Self::State, key: &str);
}

impl<T, R> IntoProperty<R> for T
where
    T: Into<JsValue>,
    R: DomRenderer,
    R::Element: Clone,
{
    type State = (R::Element, JsValue);

    fn hydrate<const FROM_SERVER: bool>(
        self,
        el: &R::Element,
        key: &str,
    ) -> Self::State {
        let value = self.into();
        R::set_property(el, key, &value);
        (el.clone(), value)
    }

    fn build(self, el: &R::Element, key: &str) -> Self::State {
        let value = self.into();
        R::set_property(el, key, &value);
        (el.clone(), value)
    }

    fn rebuild(self, state: &mut Self::State, key: &str) {
        let (el, prev) = state;
        let value = self.into();
        if value != *prev {
            R::set_property(el, key, &value);
        }
        *prev = value;
    }
}
