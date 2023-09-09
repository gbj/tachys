use crate::html::attribute::Attribute;
use crate::hydration::Cursor;
use crate::view::Position;
use crate::view::{ToTemplate, View};
use std::fmt::Debug;
use std::marker::PhantomData;
use wasm_bindgen::JsCast;
use web_sys::Element;

pub struct HtmlElement<E, At, Ch>
where
    E: ElementType,
    At: Attribute,
    Ch: View,
{
    ty: PhantomData<E>,
    attributes: At,
    children: Ch,
}

pub trait ElementType {
    const TAG: &'static str;
}

impl<E, At, Ch> View for HtmlElement<E, At, Ch>
where
    E: ElementType,
    At: Attribute,
    Ch: View,
{
    type State = (Element, At::State, Ch::State);

    fn to_html(&self, buf: &mut String, position: &mut Position) {
        // opening tag and attributes
        buf.push('<');
        buf.push_str(E::TAG);
        self.attributes.to_html(buf);
        buf.push('>');

        // children
        *position = Position::FirstChild;
        self.children.to_html(buf, position);

        // closing tag
        buf.push_str("</");
        buf.push_str(E::TAG);
        buf.push('>');
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &mut Cursor,
        position: &mut Position,
    ) -> Self::State {
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
            "position is {position:?}"
        )));
        if *position == Position::FirstChild {
            cursor.child();
        } else {
            cursor.sibling();
        }
        let el = cursor.current().to_owned();

        let attrs = self.attributes.hydrate::<FROM_SERVER>(el.unchecked_ref());

        // hydrate children
        *position = Position::FirstChild;
        let children = self.children.hydrate::<FROM_SERVER>(cursor, position);
        cursor.set(el.clone());

        // go to next sibling
        *position = Position::NextChild;

        (el.unchecked_into(), attrs, children)
    }

    fn rebuild(self, state: &mut Self::State) {
        let (el, attributes, children) = state;
        self.attributes.rebuild(attributes);
        self.children.rebuild(children);
    }
}

impl<E, At, Ch> ToTemplate for HtmlElement<E, At, Ch>
where
    E: ElementType,
    At: Attribute + ToTemplate,
    Ch: View + ToTemplate,
{
    fn to_template(buf: &mut String, position: &mut Position) {
        // opening tag and attributes
        buf.push('<');
        buf.push_str(E::TAG);
        <At as ToTemplate>::to_template(buf, position);
        buf.push('>');

        // children
        *position = Position::FirstChild;
        Ch::to_template(buf, position);

        // closing tag
        buf.push_str("</");
        buf.push_str(E::TAG);
        buf.push('>');
        *position = Position::NextChild;
    }
}

macro_rules! html_elements {
	($($tag:ident),* $(,)?) => {
        paste::paste! {
            $(
                pub fn $tag<At, Ch>(attributes: At, children: Ch) -> HtmlElement<[<$tag:camel>], At, Ch>
                where
                    At: Attribute ,
                    Ch: View
                {
                    HtmlElement {
                        ty: PhantomData,
                        attributes,
                        children
                    }
                }

                #[derive(Debug, Copy, Clone, PartialEq, Eq)]
                pub struct [<$tag:camel>];

                impl ElementType for [<$tag:camel>] {
                    const TAG: &'static str = stringify!($tag);
                }
            )*
		}
    }
}

macro_rules! html_self_closing_elements {
	($($tag:ident),* $(,)?) => {
        paste::paste! {
            $(
                pub fn $tag<At>(attributes: At) -> HtmlElement<[<$tag:camel>], At, ()>
                where
                    At: Attribute ,
                {
                    HtmlElement {
                        ty: PhantomData,
                        attributes,
                        children: ()
                    }
                }

                #[derive(Debug, Copy, Clone, PartialEq, Eq)]
                pub struct [<$tag:camel>];

                impl ElementType for [<$tag:camel>] {
                    const TAG: &'static str = stringify!($tag);
                }
            )*
		}
    }
}

html_self_closing_elements! {
    area,
    base,
    br,
    col,
    embed,
    hr,
    img,
    input,
    link,
    meta,
    param,
    source,
    track,
    wbr
}

html_elements! {
    a,
    abbr,
    acronym,
    address,
    applet,
    article,
    aside,
    audio,
    b,
    bdi,
    bdo,
    bgsound,
    big,
    blink,
    blockquote,
    body,
    button,
    canvas,
    caption,
    center,
    cite,
    code,
    colgroup,
    content,
    data,
    datalist,
    dd,
    del,
    details,
    dfn,
    dialog,
    dir,
    div,
    dl,
    dt,
    em,
    fieldset,
    figcaption,
    figure,
    font,
    footer,
    form,
    frame,
    frameset,
    h1,
    head,
    header,
    hgroup,
    html,
    i,
    iframe,
    image,
    ins,
    kbd,
    keygen,
    label,
    legend,
    li,
    main,
    map,
    mark,
    marquee,
    menu,
    menuitem,
    meter,
    nav,
    nobr,
    noembed,
    noframes,
    noscript,
    object,
    ol,
    optgroup,
    option,
    output,
    p,
    picture,
    plaintext,
    portal,
    pre,
    progress,
    q,
    rb,
    rp,
    rt,
    rtc,
    ruby,
    s,
    samp,
    script,
    section,
    select,
    shadow,
    slot,
    small,
    spacer,
    span,
    strike,
    strong,
    style,
    sub,
    summary,
    sup,
    table,
    tbody,
    td,
    template,
    textarea,
    tfoot,
    th,
    thead,
    time,
    title,
    tr,
    tt,
    u,
    ul,
    var,
    video,
    xmp
}
