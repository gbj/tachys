use std::borrow::Cow;
use std::marker::PhantomData;
use web_sys::Element;

pub trait Attribute {
    fn to_html(&self, buf: &mut String);

    fn to_template(buf: &mut String);

    fn hydrate<const IS_HYDRATING: bool>(self, el: &Element);
}

impl Attribute for () {
    fn to_html(&self, _buf: &mut String) {}

    fn to_template(_buf: &mut String) {}

    fn hydrate<const IS_HYDRATING: bool>(self, _el: &Element) {}
}

pub trait AttributeValue {
    fn to_html(&self, key: &str, buf: &mut String);

    fn to_template(key: &str, buf: &mut String);

    fn hydrate<const IS_HYDRATING: bool>(self, key: &str, el: &Element);
}

impl AttributeValue for () {
    fn to_html(&self, _key: &str, _buf: &mut String) {}

    fn to_template(_key: &str, _buf: &mut String) {}

    fn hydrate<const IS_HYDRATING: bool>(self, _key: &str, _el: &Element) {}
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

    fn hydrate<const IS_HYDRATING: bool>(self, key: &str, el: &Element) {
        // if we're actually hydrating from SSRed HTML, we don't need to set the attribute
        // if we're hydrating from a CSR-cloned <template>, we do need to set non-StaticAttr attributes
        if !IS_HYDRATING {
            el.set_attribute(key, self);
        }
    }
}

impl AttributeValue for String {
    fn to_html(&self, key: &str, buf: &mut String) {
        self.as_str().to_html(key, buf);
    }

    fn to_template(key: &str, buf: &mut String) {}

    fn hydrate<const IS_HYDRATING: bool>(self, key: &str, el: &Element) {
        self.as_str().hydrate::<IS_HYDRATING>(key, el)
    }
}

impl AttributeValue for bool {
    fn to_html(&self, key: &str, buf: &mut String) {
        if *self {
            buf.push(' ');
            buf.push_str(key);
        }
    }

    fn to_template(key: &str, buf: &mut String) {}

    fn hydrate<const IS_HYDRATING: bool>(self, key: &str, el: &Element) {
        // if we're actually hydrating from SSRed HTML, we don't need to set the attribute
        // if we're hydrating from a CSR-cloned <template>, we do need to set non-StaticAttr attributes
        if !IS_HYDRATING {
            el.set_attribute(key, "");
        }
    }
}

impl<V: AttributeValue> AttributeValue for Option<V> {
    fn to_html(&self, key: &str, buf: &mut String) {
        if let Some(v) = self {
            v.to_html(key, buf);
        }
    }

    fn to_template(key: &str, buf: &mut String) {}

    fn hydrate<const IS_HYDRATING: bool>(self, key: &str, el: &Element) {
        // if we're actually hydrating from SSRed HTML, we don't need to set the attribute
        // if we're hydrating from a CSR-cloned <template>, we do need to set non-StaticAttr attributes
        if !IS_HYDRATING {
            if let Some(v) = self {
                v.hydrate::<IS_HYDRATING>(key, el);
            }
        }
    }
}

#[derive(Debug)]
pub struct StaticAttr<K: Attribute, const V: &'static str> {
    key: PhantomData<K>,
}

impl<K: Attribute, const V: &'static str> StaticAttr<K, V> {
    pub fn new() -> Self {
        Self { key: PhantomData }
    }
}

// TODO
fn escape_attr(value: &str) -> Cow<'_, str> {
    value.into()
}

macro_rules! attributes {
	($($key:ident $html:literal),* $(,)?) => {
        paste::paste! {
            $(
                pub fn $key<V>(value: V) -> $crate::html::attribute::[<$key:camel>]<V>
				where V: $crate::html::attribute::AttributeValue + std::fmt::Debug
                {
                    $crate::html::attribute::[<$key:camel>] {
                        value
                    }
                }

                #[derive(Debug)]
                pub struct [<$key:camel>]<V>
                where
                    V: $crate::html::attribute::AttributeValue + std::fmt::Debug,
                {
                    value: V
                }

                impl<V> $crate::html::attribute::Attribute for [<$key:camel>]<V>
                where
                    V: $crate::html::attribute::AttributeValue + std::fmt::Debug
                {
                    fn to_html(&self, buf: &mut String) {
                        self.value.to_html($html, buf);
                    }

					fn to_template(buf: &mut String) {
						V::to_template($html, buf);
					}

                    fn hydrate<const IS_HYDRATING: bool>(self, el: &Element) {
                        self.value.hydrate::<IS_HYDRATING>($html, el);
                    }
                }

				impl<const V: &'static str> Attribute for StaticAttr<[<$key:camel>]<()>, V> {
					fn to_html(&self, buf: &mut String) {
						V.to_html($html, buf)
					}

					fn to_template(buf: &mut String) {
						buf.push_str(concat!(" ", $html, "=\""));
						buf.push_str(V);
						buf.push('"');
					}

                    fn hydrate<const IS_HYDRATING: bool>(self, _el: &Element) { }
				}
            )*
		}
    }
}

// TODO attribute names with underscores should be kebab-cased
attributes! {
    accept_charset "accept-charset",
    accept "accept",
    accesskey "accesskey",
    action "action",
    align "align",
    allow "allow",
    alt "alt",
    aria_atomic "aria-atomic",
    aria_busy "aria-busy",
    aria_controls "aria-controls",
    aria_current "aria-current",
    aria_describedby "aria-describedby",
    aria_description "aria-description",
    aria_details "aria-details",
    aria_disabled "aria-disabled",
    aria_dropeffect "aria-dropeffect",
    aria_errormessage "aria-errormessage",
    aria_flowto "aria-flowto",
    aria_grabbed "aria-grabbed",
    aria_haspopup "aria-haspopup",
    aria_hidden "aria-hidden",
    aria_invalid "aria-invalid",
    aria_keyshortcuts "aria-keyshortcuts",
    aria_label "aria-label",
    aria_labelledby "aria-labelledby",
    aria_live "aria-live",
    aria_owns "aria-owns",
    aria_relevant "aria-relevant",
    aria_roledescription "aria-roledescription",
    r#async "async",
    autocapitalize "autocapitalize",
    autocomplete "autocomplete",
    autofocus "autofocus",
    autoplay "autoplay",
    background "background",
    bgcolor "bgcolor",
    border "border",
    buffered "buffered",
    capture "capture",
    challenge "challenge",
    charset "charset",
    checked "checked",
    cite "cite",
    class "class",
    code "code",
    codebase "codebase",
    color "color",
    cols "cols",
    colspan "colspan",
    content "content",
    contenteditable "contenteditable",
    contextmenu "contextmenu",
    controls "controls",
    coords "coords",
    crossorigin "crossorigin",
    csp "csp",
    data "data",
    datetime "datetime",
    decoding "decoding",
    default "default",
    defer "defer",
    dir "dir",
    dirname "dirname",
    disabled "disabled",
    download "download",
    draggable "draggable",
    enctype "enctype",
    enterkeyhint "enterkeyhint",
    r#for "for",
    form "form",
    formaction "formaction",
    formenctype "formenctype",
    formmethod "formmethod",
    formnovalidate "formnovalidate",
    formtarget "formtarget",
    headers "headers",
    height "height",
    hidden "hidden",
    high "high",
    href "href",
    hreflang "hreflang",
    http_equiv "http-equiv",
    icon "icon",
    id "id",
    importance "importance",
    inputmode "inputmode",
    integrity "integrity",
    intrinsicsize "intrinsicsize",
    ismap "ismap",
    itemprop "itemprop",
    keytype "keytype",
    kind "kind",
    label "label",
    lang "lang",
    language "language",
    list "list",
    loading "loading",
    r#loop "loop",
    low "low",
    manifest "manifest",
    max "max",
    maxlength "maxlength",
    media "media",
    method "method",
    min "min",
    minlength "minlength",
    multiple "multiple",
    muted "muted",
    name "name",
    novalidate "novalidate",
    open "open",
    optimum "optimum",
    pattern "pattern",
    ping "ping",
    placeholder "placeholder",
    poster "poster",
    preload "preload",
    radiogroup "radiogroup",
    readonly "readonly",
    referrerpolicy "referrerpolicy",
    rel "rel",
    required "required",
    reversed "reversed",
    role "role",
    rows "rows",
    rowspan "rowspan",
    sandbox "sandbox",
    scope "scope",
    scoped "scoped",
    selected "selected",
    shape "shape",
    size "size",
    sizes "sizes",
    slot "slot",
    span "span",
    spellcheck "spellcheck",
    src "src",
    srcdoc "srcdoc",
    srclang "srclang",
    srcset "srcset",
    start "start",
    step "step",
    style "style",
    summary "summary",
    tabindex "tabindex",
    target "target",
    title "title",
    translate "translate",
    r#type "type",
    usemap "usemap",
    value "value",
    width "width",
    wrap "wrap",
}
