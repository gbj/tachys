use super::Lang;
use crate::{html::attribute::*, renderer::Renderer};

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
    Rndr: Renderer,
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

impl<T, Rndr, V> GlobalAttributes<Rndr, V> for T
where
    T: AddAttribute<Attr<Accesskey, V, Rndr>, Rndr>
        + AddAttribute<Attr<Id, V, Rndr>, Rndr>
        + AddAttribute<Attr<Lang, V, Rndr>, Rndr>,
    V: AttributeValue<Rndr>,
    Rndr: Renderer,
{
}
