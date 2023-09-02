use crate::html::attribute::Attribute;
use crate::hydration::Cursor;
use crate::view::Position;
use crate::view::View;
use std::fmt::Debug;
use wasm_bindgen::JsCast;

macro_rules! html_elements {
	($($tag:ident),* $(,)?) => {
        paste::paste! {
            $(
                pub fn $tag<At, Ch>(attributes: At, children: Ch) -> [<$tag:camel>]<At, Ch>
                where
                    At: Attribute + Debug,
                    Ch: View + Debug
                {
                    [<$tag:camel>] {
                        attributes,
                        children
                    }
                }

                #[derive(Debug)]
                pub struct [<$tag:camel>]<At, Ch>
                where
                    At: Attribute + Debug,
                    Ch: View + Debug
                {
                    attributes: At,
                    children: Ch
                }

                impl<At, Ch> View for [<$tag:camel>]<At, Ch>
                where
                    At: Attribute + Debug,
                    Ch: View + Debug + Debug
                {
                    type State = ();

                    fn to_html(&self, buf: &mut String, _position: Position) {
                        // opening tag and attributes
                        buf.push_str(concat!("<", stringify!($tag)));
                        self.attributes.to_html(buf);
                        buf.push('>');

                        // children
                        self.children.to_html(buf, Position::FirstChild);

                        // closing tag
                        buf.push_str(concat!("</", stringify!($tag), ">"));
                    }

                    fn to_template(buf: &mut String, _position: Position) -> Position {
                        // opening tag and attributes
                        buf.push_str(concat!("<", stringify!($tag)));
                        At::to_template(buf);
                        buf.push('>');

                        // children
                        Ch::to_template(buf, Position::FirstChild);

                        // closing tag
                        buf.push_str(concat!("</", stringify!($tag), ">"));
                        Position::NextChild
                    }

                    fn hydrate<const IS_HYDRATING: bool>(self, cursor: &mut Cursor, position: Position) -> Position {
                        if position == Position::FirstChild {
                            cursor.child();
                        } else {
                            cursor.sibling();
                        }
                        $crate::dom::log(concat!("hydrating <", stringify!($tag), ">"));
                        let curr = cursor.current().clone();
                        self.attributes.hydrate::<IS_HYDRATING>(curr.unchecked_ref());
                        self.children.hydrate::<IS_HYDRATING>(cursor, Position::FirstChild);
                        cursor.set(curr.clone());
                        Position::NextChild
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
                pub fn $tag<At>(attributes: At) -> [<$tag:camel>]<At>
                where
                    At: Attribute + Debug,
                {
                    [<$tag:camel>] {
                        attributes,
                    }
                }

                #[derive(Debug)]
                pub struct [<$tag:camel>]<At>
                where
                    At: Attribute + Debug,
                {
                    attributes: At,
                }

                impl<At> View for [<$tag:camel>]<At>
                where
                    At: Attribute + Debug,
                {
                    type State = ();

                    fn to_html(&self, buf: &mut String, _position: Position) {
                        // opening tag and attributes
                        buf.push_str(concat!("<", stringify!($tag)));
                        self.attributes.to_html(buf);
                        buf.push('>');
                    }

                    fn to_template(buf: &mut String, _position: Position) -> Position {
                        // opening tag and attributes
                        buf.push_str(concat!("<", stringify!($tag)));
                        At::to_template(buf);
                        buf.push('>');
                        Position::NextChild
                    }

                    fn hydrate<const IS_HYDRATING: bool>(self, cursor: &mut $crate::hydration::Cursor, position: Position) -> Position {
                        $crate::dom::log(concat!("hydrating <", stringify!($tag), ">"));
                        if position == Position::FirstChild {
                            cursor.child();
                        } else {
                            cursor.sibling();
                        }
                        Position::NextChild
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
