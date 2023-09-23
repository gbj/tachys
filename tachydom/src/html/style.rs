use std::borrow::Cow;

use leptos_reactive::{create_render_effect, Effect};
use wasm_bindgen::JsCast;
use web_sys::{CssStyleDeclaration, Element, HtmlElement};

use crate::or_debug;
use crate::view::ToTemplate;

use super::attribute::Attribute;

/// Adds to the style attribute of the parent element.
///
/// This can take a plain string value, which will be assigned to the `style`
///
#[inline(always)]
pub fn style<S>(s: S) -> Style<S>
where
    S: IntoStyle,
{
    Style(s)
}

pub struct Style<S>(S)
where
    S: IntoStyle;

impl<S> Attribute for Style<S>
where
    S: IntoStyle,
{
    type State = S::State;

    fn to_html(&self, _buf: &mut String, _class: &mut String, style: &mut String) {
        self.0.to_html(style);
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
        self.0.hydrate::<FROM_SERVER>(el)
    }

    fn build(self, el: &Element) -> Self::State {
        self.0.build(el)
    }

    fn rebuild(self, state: &mut Self::State) {
        self.0.rebuild(state)
    }
}

impl<S> ToTemplate for Style<S>
where
    S: IntoStyle,
{
    fn to_template(buf: &mut String, position: &mut crate::view::Position) {
        todo!()
    }
}

/// Any type that can be added to the `style` attribute or set as a style in
/// the [`CssStyleDeclaration`]. This could be a plain string, or a property name-value pair.
pub trait IntoStyle {
    type State;

    fn to_html(&self, class: &mut String);

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State;

    fn build(self, el: &Element) -> Self::State;

    fn rebuild(self, state: &mut Self::State);
}

pub trait StylePropertyValue {
    type State;

    fn to_html(&self, name: &str, class: &mut String);

    fn hydrate<const FROM_SERVER: bool>(self, name: Cow<'static, str>, el: &Element)
        -> Self::State;

    fn rebuild(self, name: Cow<'static, str>, state: &mut Self::State);
}

impl<'a> IntoStyle for &'a str {
    type State = (Element, &'a str);

    fn to_html(&self, style: &mut String) {
        style.push_str(self);
        style.push(';');
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
        (el.to_owned(), self)
    }

    fn build(self, el: &Element) -> Self::State {
        el.set_attribute("style", self);
        (el.to_owned(), self)
    }

    fn rebuild(self, state: &mut Self::State) {
        let (el, prev) = state;
        if self != *prev {
            or_debug!(el.set_attribute("style", self), el, "setAttribute");
        }
        *prev = self;
    }
}

impl IntoStyle for String {
    type State = (Element, String);

    fn to_html(&self, style: &mut String) {
        style.push_str(self);
        style.push(';');
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
        (el.to_owned(), self)
    }

    fn build(self, el: &Element) -> Self::State {
        el.set_attribute("style", &self);
        (el.to_owned(), self)
    }

    fn rebuild(self, state: &mut Self::State) {
        let (el, prev) = state;
        if self != *prev {
            el.set_attribute("style", &self);
        }
        *prev = self;
    }
}

impl<'a> IntoStyle for (&'a str, &'a str) {
    type State = (CssStyleDeclaration, &'a str);

    fn to_html(&self, style: &mut String) {
        let (name, value) = self;
        style.push_str(name);
        style.push(':');
        style.push_str(value);
        style.push(';');
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
        let style = el.unchecked_ref::<HtmlElement>().style();
        (style, self.1)
    }

    fn build(self, el: &Element) -> Self::State {
        let (name, value) = self;
        let style = el.unchecked_ref::<HtmlElement>().style();
        style.set_property(name, &value);
        (style, self.1)
    }

    fn rebuild(self, state: &mut Self::State) {
        let (name, value) = self;
        let (style, prev) = state;
        if value != *prev {
            style.set_property(name, value);
        }
        *prev = value;
    }
}

impl<'a> IntoStyle for (&'a str, String) {
    type State = (CssStyleDeclaration, String);

    fn to_html(&self, style: &mut String) {
        let (name, value) = self;
        style.push_str(name);
        style.push_str(":");
        style.push_str(value);
        style.push(';');
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
        let style = el.unchecked_ref::<HtmlElement>().style();
        (style, self.1)
    }

    fn build(self, el: &Element) -> Self::State {
        let (name, value) = &self;
        let style = el.unchecked_ref::<HtmlElement>().style();
        style.set_property(name, &value);
        (style, self.1)
    }

    fn rebuild(self, state: &mut Self::State) {
        let (name, value) = self;
        let (style, prev) = state;
        if value != *prev {
            style.set_property(name, &value);
        }
        *prev = value;
    }
}

impl<F, S> IntoStyle for (&'static str, F)
where
    F: Fn() -> S + 'static,
    S: Into<Cow<'static, str>>,
{
    type State = Effect<(CssStyleDeclaration, Cow<'static, str>)>;

    fn to_html(&self, style: &mut String) {
        let (name, f) = self;
        let value = f();
        style.push_str(name);
        style.push(':');
        style.push_str(&value.into());
        style.push(';');
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
        let (name, f) = self;
        // TODO FROM_SERVER vs template
        let style = el.unchecked_ref::<HtmlElement>().style();
        create_render_effect(move |prev| {
            let value = f().into();
            if let Some(mut state) = prev {
                let (style, prev): &mut (CssStyleDeclaration, Cow<'static, str>) = &mut state;
                if &value != prev {
                    style.set_property(name, &value);
                }
                *prev = value;
                state
            } else {
                (style.clone(), value)
            }
        })
    }

    fn build(self, el: &Element) -> Self::State {
        todo!()
    }

    fn rebuild(self, state: &mut Self::State) {}
}

impl<F, C> IntoStyle for F
where
    F: Fn() -> C + 'static,
    C: IntoStyle + 'static,
{
    type State = Effect<C::State>;

    fn to_html(&self, class: &mut String) {
        let value = self();
        value.to_html(class);
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
        // TODO FROM_SERVER vs template
        let el = el.to_owned();
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

    fn build(self, el: &Element) -> Self::State {
        todo!()
    }

    fn rebuild(self, state: &mut Self::State) {}
}

#[cfg(test)]
mod tests {
    use crate::{
        html::{element::p, style::style},
        view::{Position, PositionState, Render},
    };

    #[test]
    fn adds_simple_style() {
        let mut html = String::new();
        let el = p(style("display: block"), ());
        el.to_html(&mut html, &PositionState::new(Position::FirstChild));

        assert_eq!(html, r#"<p style="display: block;"></p>"#);
    }

    #[test]
    fn mixes_plain_and_specific_styles() {
        let mut html = String::new();
        let el = p((style("display: block"), style(("color", "blue"))), ());
        el.to_html(&mut html, &PositionState::new(Position::FirstChild));

        assert_eq!(html, r#"<p style="display: block;color:blue;"></p>"#);
    }

    #[test]
    fn handles_dynamic_styles() {
        let mut html = String::new();
        let el = p(
            (
                style("display: block"),
                style(("color", "blue")),
                style(("font-weight", || "bold".to_string())),
            ),
            (),
        );
        el.to_html(&mut html, &PositionState::new(Position::FirstChild));

        assert_eq!(
            html,
            r#"<p style="display: block;color:blue;font-weight:bold;"></p>"#
        );
    }

    /* #[test]
    fn adds_class_with_dynamic() {
        let mut html = String::new();
        let el = p((class("foo bar"), class(("baz", true))), ());
        el.to_html(&mut html, &mut Position::FirstChild);

        assert_eq!(html, r#"<p class="foo bar baz"></p>"#);
    }

    #[test]
    fn adds_class_with_dynamic_and_function() {
        let mut html = String::new();
        let el = p(
            (
                class("foo bar"),
                class(("baz", || true)),
                class(("boo", false)),
            ),
            (),
        );
        el.to_html(&mut html, &mut Position::FirstChild);

        assert_eq!(html, r#"<p class="foo bar baz"></p>"#);
    } */
}
