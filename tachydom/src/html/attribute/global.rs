use super::Lang;
use crate::{
    html::{
        attribute::*,
        class::{class, Class, IntoClass},
        event::{on, EventDescriptor, On},
        style::{style, IntoStyle, Style},
    },
    renderer::DomRenderer,
};

pub trait AddAttribute<NewAttr, Rndr>
where
    Rndr: Renderer,
{
    type Output;

    fn add_attr(self, attr: NewAttr) -> Self::Output;
}

pub trait GlobalAttributes<Rndr, V>
where
    Self: Sized
        + AddAttribute<Attr<Accesskey, V, Rndr>, Rndr>
        + AddAttribute<Attr<Id, V, Rndr>, Rndr>
        + AddAttribute<Attr<Lang, V, Rndr>, Rndr>,
    V: AttributeValue<Rndr>,
    Rndr: DomRenderer,
{
    fn accesskey(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Accesskey, V, Rndr>, Rndr>>::Output {
        self.add_attr(accesskey(value))
    }

    fn id(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Id, V, Rndr>, Rndr>>::Output {
        self.add_attr(id(value))
    }

    fn lang(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Lang, V, Rndr>, Rndr>>::Output {
        self.add_attr(lang(value))
    }
}

pub trait ClassAttribute<C, Rndr>
where
    C: IntoClass<Rndr>,
    Rndr: DomRenderer,
    Self: Sized + AddAttribute<Class<C, Rndr>, Rndr>,
{
    fn class(
        self,
        value: C,
    ) -> <Self as AddAttribute<Class<C, Rndr>, Rndr>>::Output {
        self.add_attr(class(value))
    }
}

pub trait StyleAttribute<S, Rndr>
where
    S: IntoStyle<Rndr>,
    Rndr: DomRenderer,
    Self: Sized + AddAttribute<Style<S, Rndr>, Rndr>,
{
    fn style(
        self,
        value: S,
    ) -> <Self as AddAttribute<Style<S, Rndr>, Rndr>>::Output {
        self.add_attr(style(value))
    }
}

pub trait OnAttribute<E, F, Rndr>
where
    E: EventDescriptor + 'static,
    E::EventType: 'static,
    E::EventType: From<Rndr::Event>,
    F: FnMut(E::EventType) + 'static,
    Rndr: DomRenderer,
    Self: Sized + AddAttribute<On<Rndr>, Rndr>,
{
    fn on(
        self,
        event: E,
        cb: F,
    ) -> <Self as AddAttribute<On<Rndr>, Rndr>>::Output
where {
        self.add_attr(on(event, cb))
    }
}

impl<T, Rndr, V> GlobalAttributes<Rndr, V> for T
where
    T: AddAttribute<Attr<Accesskey, V, Rndr>, Rndr>
        + AddAttribute<Attr<Id, V, Rndr>, Rndr>
        + AddAttribute<Attr<Lang, V, Rndr>, Rndr>
        + AddAttribute<On<Rndr>, Rndr>,
    V: AttributeValue<Rndr>,
    Rndr: DomRenderer,
{
}

impl<T, C, Rndr> ClassAttribute<C, Rndr> for T
where
    T: AddAttribute<Class<C, Rndr>, Rndr>,
    C: IntoClass<Rndr>,
    Rndr: DomRenderer,
{
}

impl<T, S, Rndr> StyleAttribute<S, Rndr> for T
where
    T: AddAttribute<Style<S, Rndr>, Rndr>,
    S: IntoStyle<Rndr>,
    Rndr: DomRenderer,
{
}

impl<T, E, F, Rndr> OnAttribute<E, F, Rndr> for T
where
    T: AddAttribute<On<Rndr>, Rndr>,
    E: EventDescriptor + 'static,
    E::EventType: 'static,
    E::EventType: From<Rndr::Event>,
    F: FnMut(E::EventType) + 'static,
    Rndr: DomRenderer,
{
}
