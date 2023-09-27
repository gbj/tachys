use crate::{
    dom::document,
    html::attribute::Attribute,
    hydration::Cursor,
    renderer::{dom::Dom, Renderer},
    view::{
        Mountable, Position, PositionState, Render, RenderHtml, ToTemplate,
    },
};
use once_cell::unsync::Lazy;
use std::{fmt::Debug, marker::PhantomData};
use wasm_bindgen::JsCast;
use web_sys::{Element, Node};

pub struct HtmlElement<E, At, Ch>
where
    E: ElementType,
    At: Attribute,
    Ch: Render,
{
    ty: PhantomData<E>,
    attributes: At,
    children: Ch,
}

pub trait ElementType {
    const TAG: &'static str;
    const SELF_CLOSING: bool;

    fn create_element() -> Element;
}

impl<E, At, Ch> Render for HtmlElement<E, At, Ch>
where
    E: ElementType,
    At: Attribute,
    Ch: Render,
{
    type State = (Element, At::State, Ch::State);

    fn rebuild(self, state: &mut Self::State) {
        let (_, attributes, children) = state;
        self.attributes.rebuild(attributes);
        self.children.rebuild(children);
    }

    fn build(self) -> Self::State {
        let el = E::create_element();
        let at = self.attributes.build(&el);
        let children = self.children.build();
        if let Some(child) = children.as_mountable() {
            Dom::insert_node(&el, &child, None);
        }
        (el, at, children)
    }
}

impl<E, At, Ch> RenderHtml for HtmlElement<E, At, Ch>
where
    E: ElementType,
    At: Attribute,
    Ch: RenderHtml,
{
    fn to_html(&mut self, buf: &mut String, position: &PositionState) {
        // opening tag
        buf.push('<');
        buf.push_str(E::TAG);

        // attributes

        // `class` and `style` are created first, and pushed later
        // this is because they can be filled by a mixture of values that include
        // either the whole value (`class="..."` or `style="..."`) and individual
        // classes and styles (`class:foo=true` or `style:height="40px"`), so they
        // need to be filled during the whole attribute-creation process and then
        // added

        // String doesn't allocate until the first push, so this is cheap if there
        // is no class or style on an element
        let mut class = String::new();
        let mut style = String::new();

        // inject regular attributes, and fill class and style
        self.attributes.to_html(buf, &mut class, &mut style);

        if !class.is_empty() {
            buf.push(' ');
            buf.push_str("class=\"");
            buf.push_str(class.trim_start().trim_end());
            buf.push('"');
        }
        if !style.is_empty() {
            buf.push(' ');
            buf.push_str("style=\"");
            buf.push_str(style.trim_start().trim_end());
            buf.push('"');
        }

        buf.push('>');

        if !E::SELF_CLOSING {
            // children
            position.set(Position::FirstChild);
            self.children.to_html(buf, position);

            // closing tag
            buf.push_str("</");
            buf.push_str(E::TAG);
            buf.push('>');
        }
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        if position.get() == Position::FirstChild {
            cursor.child();
        } else {
            cursor.sibling();
        }
        let el = cursor.current().to_owned();

        let attrs = self.attributes.hydrate::<FROM_SERVER>(el.unchecked_ref());

        // hydrate children
        position.set(Position::FirstChild);
        let children = self.children.hydrate::<FROM_SERVER>(cursor, position);
        cursor.set(el.clone());

        // go to next sibling
        position.set(Position::NextChild);

        (el.unchecked_into(), attrs, children)
    }
}

impl<At, Ch> Mountable for (Element, At, Ch) {
    fn unmount(&mut self) {
        self.0.remove()
    }

    fn as_mountable(&self) -> Option<Node> {
        Some(self.0.clone().unchecked_into())
    }
}

impl<E, At, Ch> ToTemplate for HtmlElement<E, At, Ch>
where
    E: ElementType,
    At: Attribute + ToTemplate,
    Ch: Render + ToTemplate,
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
                    Ch: Render
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
                    const SELF_CLOSING: bool = false;

                    fn create_element() -> Element {
                        thread_local! {
                            static ELEMENT: Lazy<Element> = Lazy::new(|| {
                                document().create_element(stringify!($tag)).unwrap()
                            });
                        }
                        ELEMENT.with(|e| e.clone_node()).unwrap().unchecked_into()
                    }
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
                    const SELF_CLOSING: bool = true;

                    fn create_element() -> Element {
                        thread_local! {
                            static ELEMENT: Lazy<Element> = Lazy::new(|| {
                                document().create_element(stringify!($tag)).unwrap()
                            });
                        }
                        ELEMENT.with(|e| e.clone_node()).unwrap().unchecked_into()
                    }
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
    // option, // creates conflict with core Option
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

pub fn option<At, Ch>(
    attributes: At,
    children: Ch,
) -> HtmlElement<Option_, At, Ch>
where
    At: Attribute,
    Ch: Render,
{
    HtmlElement {
        ty: PhantomData,
        attributes,
        children,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Option_;

impl ElementType for Option_ {
    const TAG: &'static str = "option";
    const SELF_CLOSING: bool = false;

    fn create_element() -> Element {
        thread_local! {
            static ELEMENT: Lazy<Element> = Lazy::new(|| {
                document().create_element("option").unwrap()
            });
        }
        ELEMENT.with(|e| e.clone_node()).unwrap().unchecked_into()
    }
}
