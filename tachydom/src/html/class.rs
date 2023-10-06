use super::attribute::Attribute;
use crate::{renderer::DomRenderer, view::ToTemplate};
use leptos_reactive::{create_render_effect, Effect};
use std::marker::PhantomData;

#[inline(always)]
pub fn class<R>(class: impl IntoClass<R>) -> impl Attribute<R>
where
    R: DomRenderer,
{
    Class {
        class,
        rndr: PhantomData,
    }
}

struct Class<C, R>
where
    C: IntoClass<R>,
    R: DomRenderer,
{
    class: C,
    rndr: PhantomData<R>,
}

impl<C, R> Attribute<R> for Class<C, R>
where
    C: IntoClass<R>,
    R: DomRenderer,
{
    type State = C::State;

    fn to_html(
        &self,
        _buf: &mut String,
        class: &mut String,
        _style: &mut String,
    ) {
        class.push(' ');
        self.class.to_html(class);
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &R::Element) -> Self::State {
        self.class.hydrate::<FROM_SERVER>(el)
    }

    fn build(self, el: &R::Element) -> Self::State {
        self.class.build(el)
    }

    fn rebuild(self, state: &mut Self::State) {
        self.class.rebuild(state)
    }
}

impl<C, R> ToTemplate for Class<C, R>
where
    C: IntoClass<R>,
    R: DomRenderer,
{
    fn to_template(buf: &mut String, position: &mut crate::view::Position) {
        todo!()
    }
}

pub trait IntoClass<R: DomRenderer> {
    type State;

    fn to_html(&self, class: &mut String);

    fn hydrate<const FROM_SERVER: bool>(self, el: &R::Element) -> Self::State;

    fn build(self, el: &R::Element) -> Self::State;

    fn rebuild(self, state: &mut Self::State);
}

impl<'a, R> IntoClass<R> for &'a str
where
    R: DomRenderer,
    R::Element: Clone,
{
    type State = (R::Element, &'a str);

    fn to_html(&self, class: &mut String) {
        class.push_str(self);
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &R::Element) -> Self::State {
        (el.clone(), self)
    }

    fn build(self, el: &R::Element) -> Self::State {
        R::set_attribute(el, "class", self);
        (el.clone(), self)
    }

    fn rebuild(self, state: &mut Self::State) {
        let (el, prev) = state;
        if self != *prev {
            R::set_attribute(el, "class", self);
        }
        *prev = self;
    }
}

impl<R> IntoClass<R> for String
where
    R: DomRenderer,
    R::Element: Clone,
{
    type State = (R::Element, String);

    fn to_html(&self, class: &mut String) {
        IntoClass::<R>::to_html(&self.as_str(), class);
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &R::Element) -> Self::State {
        (el.clone(), self)
    }

    fn build(self, el: &R::Element) -> Self::State {
        R::set_attribute(el, "class", &self);
        (el.clone(), self)
    }

    fn rebuild(self, state: &mut Self::State) {
        let (el, prev) = state;
        if self != *prev {
            R::set_attribute(el, "class", &self);
        }
        *prev = self;
    }
}

impl<R> IntoClass<R> for (&'static str, bool)
where
    R: DomRenderer,
{
    type State = (R::ClassList, bool);

    fn to_html(&self, class: &mut String) {
        let (name, include) = self;
        if *include {
            class.push_str(name);
        }
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &R::Element) -> Self::State {
        let class_list = R::class_list(el);
        (class_list, self.1)
    }

    fn build(self, el: &R::Element) -> Self::State {
        let (name, include) = self;
        let class_list = R::class_list(el);
        if include {
            R::add_class(&class_list, name);
        }
        (class_list, self.1)
    }

    fn rebuild(self, state: &mut Self::State) {
        let (name, include) = self;
        let (class_list, prev_include) = state;
        if include != *prev_include {
            if include {
                R::add_class(class_list, name);
            } else {
                R::remove_class(class_list, name);
            }
        }
        *prev_include = include;
    }
}

impl<F, C, R> IntoClass<R> for F
where
    F: Fn() -> C + 'static,
    C: IntoClass<R> + 'static,
    C::State: 'static,
    R: DomRenderer,
    R::ClassList: 'static,
    R::Element: Clone + 'static,
{
    type State = Effect<C::State>;

    fn to_html(&self, class: &mut String) {
        let value = self();
        value.to_html(class);
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &R::Element) -> Self::State {
        // TODO FROM_SERVER vs template
        let el = el.clone();
        create_render_effect(move |prev| {
            let value = self();
            if let Some(mut state) = prev {
                value.rebuild(&mut state);
                state
            } else {
                value.hydrate::<FROM_SERVER>(&el)
            }
        })
    }

    fn build(self, el: &R::Element) -> Self::State {
        let el = el.to_owned();
        create_render_effect(move |prev| {
            let value = self();
            if let Some(mut state) = prev {
                value.rebuild(&mut state);
                state
            } else {
                value.build(&el)
            }
        })
    }

    fn rebuild(self, state: &mut Self::State) {}
}

impl<F, R> IntoClass<R> for (&'static str, F)
where
    F: Fn() -> bool + 'static,
    R: DomRenderer,
    R::ClassList: 'static,
    R::Element: Clone,
{
    type State = Effect<bool>;

    fn to_html(&self, class: &mut String) {
        let (name, f) = self;
        let include = f();
        if include {
            <&str as IntoClass<R>>::to_html(name, class);
        }
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &R::Element) -> Self::State {
        // TODO FROM_SERVER vs template
        let (name, f) = self;
        let class_list = R::class_list(el);
        create_render_effect(move |prev| {
            let include = f();
            if Some(include) != prev {
                if include {
                    R::add_class(&class_list, name);
                } else {
                    R::remove_class(&class_list, name);
                }
            }
            include
        })
    }

    fn build(self, el: &R::Element) -> Self::State {
        let (name, f) = self;
        let class_list = R::class_list(el);
        create_render_effect(move |prev| {
            let include = f();
            if Some(include) != prev {
                if include {
                    R::add_class(&class_list, name);
                } else {
                    R::remove_class(&class_list, name);
                }
            }
            include
        })
    }

    fn rebuild(self, state: &mut Self::State) {}
}

#[cfg(test)]
mod tests {
    use crate::{
        html::{
            class::class,
            element::{p, HtmlElement},
        },
        renderer::dom::Dom,
        view::{Position, PositionState, RenderHtml},
    };

    #[test]
    fn adds_simple_class() {
        let mut html = String::new();
        let el: HtmlElement<_, _, _, Dom> = p(class("foo bar"), ());
        el.to_html(&mut html, &PositionState::new(Position::FirstChild));

        assert_eq!(html, r#"<p class="foo bar"></p>"#);
    }

    #[test]
    fn adds_class_with_dynamic() {
        let mut html = String::new();
        let el: HtmlElement<_, _, _, Dom> =
            p((class("foo bar"), class(("baz", true))), ());
        el.to_html(&mut html, &PositionState::new(Position::FirstChild));

        assert_eq!(html, r#"<p class="foo bar baz"></p>"#);
    }

    #[test]
    fn adds_class_with_dynamic_and_function() {
        let mut html = String::new();
        let el: HtmlElement<_, _, _, Dom> = p(
            (
                class("foo bar"),
                class(("baz", || true)),
                class(("boo", false)),
            ),
            (),
        );
        el.to_html(&mut html, &PositionState::new(Position::FirstChild));

        assert_eq!(html, r#"<p class="foo bar baz"></p>"#);
    }
}
