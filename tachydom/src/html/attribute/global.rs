use super::Lang;
use crate::{
    html::{
        attribute::*,
        class::{class, Class, IntoClass},
        event::{on, EventDescriptor, On},
        property::{property, IntoProperty, Property},
        style::{style, IntoStyle, Style},
    },
    renderer::DomRenderer,
};
use core::convert::From;

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
        + AddAttribute<Attr<Autocapitalize, V, Rndr>, Rndr>
        + AddAttribute<Attr<Autofocus, V, Rndr>, Rndr>
        + AddAttribute<Attr<Contenteditable, V, Rndr>, Rndr>
        + AddAttribute<Attr<Dir, V, Rndr>, Rndr>
        + AddAttribute<Attr<Draggable, V, Rndr>, Rndr>
        + AddAttribute<Attr<Enterkeyhint, V, Rndr>, Rndr>
        + AddAttribute<Attr<Hidden, V, Rndr>, Rndr>
        + AddAttribute<Attr<Id, V, Rndr>, Rndr>
        + AddAttribute<Attr<Inert, V, Rndr>, Rndr>
        + AddAttribute<Attr<Inputmode, V, Rndr>, Rndr>
        + AddAttribute<Attr<Is, V, Rndr>, Rndr>
        + AddAttribute<Attr<Itemid, V, Rndr>, Rndr>
        + AddAttribute<Attr<Itemprop, V, Rndr>, Rndr>
        + AddAttribute<Attr<Itemref, V, Rndr>, Rndr>
        + AddAttribute<Attr<Itemscope, V, Rndr>, Rndr>
        + AddAttribute<Attr<Itemtype, V, Rndr>, Rndr>
        + AddAttribute<Attr<Lang, V, Rndr>, Rndr>
        + AddAttribute<Attr<Nonce, V, Rndr>, Rndr>
        + AddAttribute<Attr<Part, V, Rndr>, Rndr>
        + AddAttribute<Attr<Popover, V, Rndr>, Rndr>
        + AddAttribute<Attr<Role, V, Rndr>, Rndr>
        + AddAttribute<Attr<Slot, V, Rndr>, Rndr>
        + AddAttribute<Attr<Spellcheck, V, Rndr>, Rndr>
        + AddAttribute<Attr<Tabindex, V, Rndr>, Rndr>
        + AddAttribute<Attr<Title, V, Rndr>, Rndr>
        + AddAttribute<Attr<Translate, V, Rndr>, Rndr>
        + AddAttribute<Attr<Virtualkeyboardpolicy, V, Rndr>, Rndr>,
    V: AttributeValue<Rndr>,
    Rndr: Renderer,
{
    fn accesskey(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Accesskey, V, Rndr>, Rndr>>::Output {
        self.add_attr(accesskey(value))
    }

    fn autocapitalize(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Autocapitalize, V, Rndr>, Rndr>>::Output
    {
        self.add_attr(autocapitalize(value))
    }

    fn autofocus(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Autofocus, V, Rndr>, Rndr>>::Output {
        self.add_attr(autofocus(value))
    }

    fn contenteditable(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Contenteditable, V, Rndr>, Rndr>>::Output
    {
        self.add_attr(contenteditable(value))
    }

    fn dir(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Dir, V, Rndr>, Rndr>>::Output {
        self.add_attr(dir(value))
    }

    fn draggable(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Draggable, V, Rndr>, Rndr>>::Output {
        self.add_attr(draggable(value))
    }

    fn enterkeyhint(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Enterkeyhint, V, Rndr>, Rndr>>::Output {
        self.add_attr(enterkeyhint(value))
    }

    fn hidden(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Hidden, V, Rndr>, Rndr>>::Output {
        self.add_attr(hidden(value))
    }

    fn id(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Id, V, Rndr>, Rndr>>::Output {
        self.add_attr(id(value))
    }

    fn inert(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Inert, V, Rndr>, Rndr>>::Output {
        self.add_attr(inert(value))
    }

    fn inputmode(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Inputmode, V, Rndr>, Rndr>>::Output {
        self.add_attr(inputmode(value))
    }

    fn is(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Is, V, Rndr>, Rndr>>::Output {
        self.add_attr(is(value))
    }

    fn itemid(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Itemid, V, Rndr>, Rndr>>::Output {
        self.add_attr(itemid(value))
    }

    fn itemprop(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Itemprop, V, Rndr>, Rndr>>::Output {
        self.add_attr(itemprop(value))
    }

    fn itemref(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Itemref, V, Rndr>, Rndr>>::Output {
        self.add_attr(itemref(value))
    }

    fn itemscope(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Itemscope, V, Rndr>, Rndr>>::Output {
        self.add_attr(itemscope(value))
    }

    fn itemtype(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Itemtype, V, Rndr>, Rndr>>::Output {
        self.add_attr(itemtype(value))
    }

    fn lang(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Lang, V, Rndr>, Rndr>>::Output {
        self.add_attr(lang(value))
    }

    fn nonce(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Nonce, V, Rndr>, Rndr>>::Output {
        self.add_attr(nonce(value))
    }

    fn part(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Part, V, Rndr>, Rndr>>::Output {
        self.add_attr(part(value))
    }

    fn popover(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Popover, V, Rndr>, Rndr>>::Output {
        self.add_attr(popover(value))
    }

    fn role(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Role, V, Rndr>, Rndr>>::Output {
        self.add_attr(role(value))
    }

    fn slot(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Slot, V, Rndr>, Rndr>>::Output {
        self.add_attr(slot(value))
    }

    fn spellcheck(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Spellcheck, V, Rndr>, Rndr>>::Output {
        self.add_attr(spellcheck(value))
    }

    fn tabindex(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Tabindex, V, Rndr>, Rndr>>::Output {
        self.add_attr(tabindex(value))
    }

    fn title(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Title, V, Rndr>, Rndr>>::Output {
        self.add_attr(title(value))
    }

    fn translate(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Translate, V, Rndr>, Rndr>>::Output {
        self.add_attr(translate(value))
    }

    fn virtualkeyboardpolicy(
        self,
        value: V,
    ) -> <Self as AddAttribute<Attr<Virtualkeyboardpolicy, V, Rndr>, Rndr>>::Output{
        self.add_attr(virtualkeyboardpolicy(value))
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

pub trait PropAttribute<K, P, Rndr>
where
    K: AsRef<str>,
    P: IntoProperty<Rndr>,
    Rndr: DomRenderer,
    Self: Sized + AddAttribute<Property<K, P, Rndr>, Rndr>,
{
    fn prop(
        self,
        key: K,
        value: P,
    ) -> <Self as AddAttribute<Property<K, P, Rndr>, Rndr>>::Output {
        self.add_attr(property(key, value))
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
        + AddAttribute<Attr<Autocapitalize, V, Rndr>, Rndr>
        + AddAttribute<Attr<Autofocus, V, Rndr>, Rndr>
        + AddAttribute<Attr<Contenteditable, V, Rndr>, Rndr>
        + AddAttribute<Attr<Dir, V, Rndr>, Rndr>
        + AddAttribute<Attr<Draggable, V, Rndr>, Rndr>
        + AddAttribute<Attr<Enterkeyhint, V, Rndr>, Rndr>
        + AddAttribute<Attr<Hidden, V, Rndr>, Rndr>
        + AddAttribute<Attr<Id, V, Rndr>, Rndr>
        + AddAttribute<Attr<Inert, V, Rndr>, Rndr>
        + AddAttribute<Attr<Inputmode, V, Rndr>, Rndr>
        + AddAttribute<Attr<Is, V, Rndr>, Rndr>
        + AddAttribute<Attr<Itemid, V, Rndr>, Rndr>
        + AddAttribute<Attr<Itemprop, V, Rndr>, Rndr>
        + AddAttribute<Attr<Itemref, V, Rndr>, Rndr>
        + AddAttribute<Attr<Itemscope, V, Rndr>, Rndr>
        + AddAttribute<Attr<Itemtype, V, Rndr>, Rndr>
        + AddAttribute<Attr<Lang, V, Rndr>, Rndr>
        + AddAttribute<Attr<Nonce, V, Rndr>, Rndr>
        + AddAttribute<Attr<Part, V, Rndr>, Rndr>
        + AddAttribute<Attr<Popover, V, Rndr>, Rndr>
        + AddAttribute<Attr<Role, V, Rndr>, Rndr>
        + AddAttribute<Attr<Slot, V, Rndr>, Rndr>
        + AddAttribute<Attr<Spellcheck, V, Rndr>, Rndr>
        + AddAttribute<Attr<Tabindex, V, Rndr>, Rndr>
        + AddAttribute<Attr<Title, V, Rndr>, Rndr>
        + AddAttribute<Attr<Translate, V, Rndr>, Rndr>
        + AddAttribute<Attr<Virtualkeyboardpolicy, V, Rndr>, Rndr>,
    V: AttributeValue<Rndr>,
    Rndr: Renderer,
{
}

impl<T, C, Rndr> ClassAttribute<C, Rndr> for T
where
    T: AddAttribute<Class<C, Rndr>, Rndr>,
    C: IntoClass<Rndr>,
    Rndr: DomRenderer,
{
}

impl<T, K, P, Rndr> PropAttribute<K, P, Rndr> for T
where
    T: AddAttribute<Property<K, P, Rndr>, Rndr>,
    K: AsRef<str>,
    P: IntoProperty<Rndr>,
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
