pub trait AttributeValue {
    fn to_html(&self, key: &str, buf: &mut String);

    fn to_template(key: &str, buf: &mut String);

    fn hydrate<const FROM_SERVER: bool>(self, key: &str, el: &Element);
}

impl AttributeValue for () {
    fn to_html(&self, _key: &str, _buf: &mut String) {}

    fn to_template(_key: &str, _buf: &mut String) {}

    fn hydrate<const FROM_SERVER: bool>(self, _key: &str, _el: &Element) {}
}

impl<'a> AttributeValue for &'a str {
    fn to_html(&self, key: &str, buf: &mut String) {
        buf.push(' ');
        buf.push_str(key);
        buf.push_str("=\"");
        buf.push_str(&escape_attr(self));
        buf.push('"');
    }

    fn to_template(key: &str, buf: &mut String) {}

    fn hydrate<const FROM_SERVER: bool>(self, key: &str, el: &Element) {
        // if we're actually hydrating from SSRed HTML, we don't need to set the attribute
        // if we're hydrating from a CSR-cloned <template>, we do need to set non-StaticAttr attributes
        if !FROM_SERVER {
            el.set_attribute(key, self);
        }
    }
}

pub trait AttributeValue {
    fn to_html(&self, key: &str, buf: &mut String);

    fn to_template(key: &str, buf: &mut String);

    fn hydrate<const FROM_SERVER: bool>(self, key: &str, el: &Element);
}

impl AttributeValue for () {
    fn to_html(&self, _key: &str, _buf: &mut String) {}

    fn to_template(_key: &str, _buf: &mut String) {}

    fn hydrate<const FROM_SERVER: bool>(self, _key: &str, _el: &Element) {}
}

impl<'a> AttributeValue for &'a str {
    fn to_html(&self, key: &str, buf: &mut String) {
        buf.push(' ');
        buf.push_str(key);
        buf.push_str("=\"");
        buf.push_str(&escape_attr(self));
        buf.push('"');
    }

    fn to_template(key: &str, buf: &mut String) {}

    fn hydrate<const FROM_SERVER: bool>(self, key: &str, el: &Element) {
        // if we're actually hydrating from SSRed HTML, we don't need to set the attribute
        // if we're hydrating from a CSR-cloned <template>, we do need to set non-StaticAttr attributes
        if !FROM_SERVER {
            el.set_attribute(key, self);
        }
    }
}
