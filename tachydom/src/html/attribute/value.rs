use crate::renderer::{dom::Dom, Renderer};
use std::borrow::Cow;
use web_sys::Element;

pub trait AttributeValue {
    type State;

    fn to_html(&self, key: &str, buf: &mut String);

    fn to_template(key: &str, buf: &mut String);

    fn hydrate<const FROM_SERVER: bool>(self, key: &str, el: &Element) -> Self::State;

    fn build(self, el: &Element, key: &str) -> Self::State;

    fn rebuild(self, key: &str, state: &mut Self::State);
}

impl AttributeValue for () {
    type State = ();
    fn to_html(&self, _key: &str, _buf: &mut String) {}

    fn to_template(_key: &str, _buf: &mut String) {}

    fn hydrate<const FROM_SERVER: bool>(self, _key: &str, _el: &Element) {}

    fn build(self, _el: &Element, _key: &str) -> Self::State {}

    fn rebuild(self, _key: &str, _state: &mut Self::State) {}
}

impl<'a> AttributeValue for &'a str {
    type State = (Element, &'a str);

    fn to_html(&self, key: &str, buf: &mut String) {
        buf.push(' ');
        buf.push_str(key);
        buf.push_str("=\"");
        buf.push_str(&escape_attr(self));
        buf.push('"');
    }

    fn to_template(key: &str, buf: &mut String) {
        // TODO
    }

    fn hydrate<const FROM_SERVER: bool>(self, key: &str, el: &Element) -> Self::State {
        // if we're actually hydrating from SSRed HTML, we don't need to set the attribute
        // if we're hydrating from a CSR-cloned <template>, we do need to set non-StaticAttr attributes
        if !FROM_SERVER {
            Dom::set_attribute(el, key, self);
        }
        (el.clone(), self)
    }

    fn build(self, el: &Element, key: &str) -> Self::State {
        Dom::set_attribute(el, key, self);
        (el.to_owned(), self)
    }

    fn rebuild(self, key: &str, state: &mut Self::State) {
        let (el, prev_value) = state;
        if self != *prev_value {
            Dom::set_attribute(el, key, self);
        }
        *prev_value = self;
    }
}

impl AttributeValue for String {
    type State = (Element, String);

    fn to_html(&self, key: &str, buf: &mut String) {
        self.as_str().to_html(key, buf);
    }

    fn to_template(key: &str, buf: &mut String) {
        // TODO
    }

    fn hydrate<const FROM_SERVER: bool>(self, key: &str, el: &Element) -> Self::State {
        let (el, _) = self.as_str().hydrate::<FROM_SERVER>(key, el);
        (el, self)
    }

    fn build(self, el: &Element, key: &str) -> Self::State {
        Dom::set_attribute(el, key, &self);
        (el.to_owned(), self)
    }

    fn rebuild(self, key: &str, state: &mut Self::State) {
        let (el, prev_value) = state;
        if self != *prev_value {
            Dom::set_attribute(el, key, &self);
        }
        *prev_value = self;
    }
}

impl AttributeValue for bool {
    type State = (Element, bool);

    fn to_html(&self, key: &str, buf: &mut String) {
        if *self {
            buf.push(' ');
            buf.push_str(key);
        }
    }

    fn to_template(key: &str, buf: &mut String) {
        // TODO
    }

    fn hydrate<const FROM_SERVER: bool>(self, key: &str, el: &Element) -> Self::State {
        // if we're actually hydrating from SSRed HTML, we don't need to set the attribute
        // if we're hydrating from a CSR-cloned <template>, we do need to set non-StaticAttr attributes
        if !FROM_SERVER {
            Dom::set_attribute(el, key, "");
        }
        (el.clone(), self)
    }

    fn build(self, el: &Element, key: &str) -> Self::State {
        if self {
            Dom::set_attribute(el, key, "");
        }
        (el.to_owned(), self)
    }

    fn rebuild(self, key: &str, state: &mut Self::State) {
        let (el, prev_value) = state;
        if self != *prev_value {
            if self {
                Dom::set_attribute(el, key, "");
            } else {
                Dom::remove_attribute(el, key);
            }
        }
        *prev_value = self;
    }
}

impl<V: AttributeValue> AttributeValue for Option<V> {
    type State = (Element, Option<V::State>);

    fn to_html(&self, key: &str, buf: &mut String) {
        if let Some(v) = self {
            v.to_html(key, buf);
        }
    }

    fn to_template(key: &str, buf: &mut String) {
        // TODO
    }

    fn hydrate<const FROM_SERVER: bool>(self, key: &str, el: &Element) -> Self::State {
        // if we're actually hydrating from SSRed HTML, we don't need to set the attribute
        // if we're hydrating from a CSR-cloned <template>, we do need to set non-StaticAttr attributes
        let state = if !FROM_SERVER {
            self.map(|v| v.hydrate::<FROM_SERVER>(key, el))
        } else {
            None
        };
        (el.clone(), state)
    }

    fn build(self, el: &Element, key: &str) -> Self::State {
        let el = el.clone();
        let v = self.map(|v| v.build(&el, key));
        (el, v)
    }

    fn rebuild(self, key: &str, state: &mut Self::State) {
        todo!()
    }
}

// TODO
fn escape_attr(value: &str) -> Cow<'_, str> {
    value.into()
}
