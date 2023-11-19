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
    const MIN_LENGTH: usize = 0;
    type State = P::State;

    fn to_html(
        self,
        _buf: &mut String,
        _class: &mut String,
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
        _buf: &mut String,
        _class: &mut String,
        _style: &mut String,
        _position: &mut Position,
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

macro_rules! prop_type {
    ($prop_type:ty) => {
        impl<R> IntoProperty<R> for $prop_type
        where
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
    };
}

prop_type!(JsValue);
prop_type!(String);
prop_type!(&String);
prop_type!(&str);
prop_type!(usize);
prop_type!(u8);
prop_type!(u16);
prop_type!(u32);
prop_type!(u64);
prop_type!(u128);
prop_type!(isize);
prop_type!(i8);
prop_type!(i16);
prop_type!(i32);
prop_type!(i64);
prop_type!(i128);
prop_type!(f32);
prop_type!(f64);
prop_type!(bool);
