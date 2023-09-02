#![allow(unused)]
use std::cell::{Cell, RefCell};

use sledgehammer_bindgen::bindgen;

#[bindgen]
mod js {
    struct Channel;

    const JS: &str = r#"const nodes = [document.body];
export function get_node(id){
    return nodes[id];
}
const els = [
    "a",
    "abbr",
    "acronym",
    "address",
    "applet",
    "area",
    "article",
    "aside",
    "audio",
    "b",
    "base",
    "bdi",
    "bdo",
    "bgsound",
    "big",
    "blink",
    "blockquote",
    "body",
    "br",
    "button",
    "canvas",
    "caption",
    "center",
    "cite",
    "code",
    "col",
    "colgroup",
    "content",
    "data",
    "datalist",
    "dd",
    "del",
    "details",
    "dfn",
    "dialog",
    "dir",
    "div",
    "dl",
    "dt",
    "em",
    "embed",
    "fieldset",
    "figcaption",
    "figure",
    "font",
    "footer",
    "form",
    "frame",
    "frameset",
    "h1",
    "head",
    "header",
    "hgroup",
    "hr",
    "html",
    "i",
    "iframe",
    "image",
    "img",
    "input",
    "ins",
    "kbd",
    "keygen",
    "label",
    "legend",
    "li",
    "link",
    "main",
    "map",
    "mark",
    "marquee",
    "menu",
    "menuitem",
    "meta",
    "meter",
    "nav",
    "nobr",
    "noembed",
    "noframes",
    "noscript",
    "object",
    "ol",
    "optgroup",
    "option",
    "output",
    "p",
    "param",
    "picture",
    "plaintext",
    "portal",
    "pre",
    "progress",
    "q",
    "rb",
    "rp",
    "rt",
    "rtc",
    "ruby",
    "s",
    "samp",
    "script",
    "section",
    "select",
    "shadow",
    "slot",
    "small",
    "source",
    "spacer",
    "span",
    "strike",
    "strong",
    "style",
    "sub",
    "summary",
    "sup",
    "table",
    "tbody",
    "td",
    "template",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "title",
    "tr",
    "track",
    "tt",
    "u",
    "ul",
    "var",
    "video",
    "wbr",
    "xmp",
];

const attrs = [
    "accept-charset",
    "accept",
    "accesskey",
    "action",
    "align",
    "allow",
    "alt",
    "aria-atomic",
    "aria-busy",
    "aria-controls",
    "aria-current",
    "aria-describedby",
    "aria-description",
    "aria-details",
    "aria-disabled",
    "aria-dropeffect",
    "aria-errormessage",
    "aria-flowto",
    "aria-grabbed",
    "aria-haspopup",
    "aria-hidden",
    "aria-invalid",
    "aria-keyshortcuts",
    "aria-label",
    "aria-labelledby",
    "aria-live",
    "aria-owns",
    "aria-relevant",
    "aria-roledescription",
    "async",
    "autocapitalize",
    "autocomplete",
    "autofocus",
    "autoplay",
    "background",
    "bgcolor",
    "border",
    "buffered",
    "capture",
    "challenge",
    "charset",
    "checked",
    "cite",
    "class",
    "code",
    "codebase",
    "color",
    "cols",
    "colspan",
    "content",
    "contenteditable",
    "contextmenu",
    "controls",
    "coords",
    "crossorigin",
    "csp",
    "data",
    "datetime",
    "decoding",
    "default",
    "defer",
    "dir",
    "dirname",
    "disabled",
    "download",
    "draggable",
    "enctype",
    "enterkeyhint",
    "for",
    "form",
    "formaction",
    "formenctype",
    "formmethod",
    "formnovalidate",
    "formtarget",
    "headers",
    "height",
    "hidden",
    "high",
    "href",
    "hreflang",
    "http-equiv",
    "icon",
    "id",
    "importance",
    "inputmode",
    "integrity",
    "intrinsicsize",
    "ismap",
    "itemprop",
    "keytype",
    "kind",
    "label",
    "lang",
    "language",
    "list",
    "loading",
    "loop",
    "low",
    "manifest",
    "max",
    "maxlength",
    "media",
    "method",
    "min",
    "minlength",
    "multiple",
    "muted",
    "name",
    "novalidate",
    "open",
    "optimum",
    "pattern",
    "ping",
    "placeholder",
    "poster",
    "preload",
    "radiogroup",
    "readonly",
    "referrerpolicy",
    "rel",
    "required",
    "reversed",
    "role",
    "rows",
    "rowspan",
    "sandbox",
    "scope",
    "scoped",
    "selected",
    "shape",
    "size",
    "sizes",
    "slot",
    "span",
    "spellcheck",
    "src",
    "srcdoc",
    "srclang",
    "srcset",
    "start",
    "step",
    "style",
    "summary",
    "tabindex",
    "target",
    "title",
    "translate",
    "type",
    "usemap",
    "value",
    "width",
    "wrap",
];"#;

    extern "C" {
        #[wasm_bindgen]
        fn get_node(id: u32) -> web_sys::Node;
    }

    fn create_element(id: u32, element_id: u8) {
        "nodes[$id$]=document.createElement(els[$element_id$]);"
    }

    fn create_text_node(id: u32, val: &str) {
        "nodes[$id$]=document.createTextNode($val$);"
    }

    fn set_attribute(id: u32, attribute_id: u8, val: &str<u8, attr_cache>) {
        "nodes[$id$].setAttribute(attrs[$attribute_id$],$val$);"
    }

    fn remove_attribute(id: u32, attribute_id: u8) {
        "nodes[$id$].removeAttribute(attrs[$attribute_id$]);"
    }

    fn append_child(id: u32, id2: u32) {
        "nodes[$id$].appendChild(nodes[$id2$]);"
    }

    fn insert_before(parent: u32, id: u32, id2: u32) {
        "nodes[$parent$].insertBefore(nodes[$id$],nodes[$id2$]);"
    }

    fn set_text(id: u32, text: &str) {
        "nodes[$id$].textContent=$text$;"
    }

    fn set_data(id: u32, text: &str) {
        "nodes[$id$].data=$text$;"
    }

    fn remove(id: u32) {
        "nodes[$id$].remove();"
    }

    fn replace(id: u32, id2: u32) {
        "nodes[$id$].replaceWith(nodes[$id2$]);"
    }

    fn clone(id: u32, id2: u32) {
        "nodes[$id2$]=nodes[$id$].cloneNode(true);"
    }

    fn first_child(id: u32) {
        "node[id]=node[id].firstChild;"
    }

    fn next_sibling(id: u32) {
        "node[id]=node[id].nextSibling;"
    }
}

pub struct Dom;

thread_local! {
    static NODE_ID: Cell<u32> = Cell::new(0);
    static CHANNEL: RefCell<Channel> = Default::default();
    static LISTENERS: RefCell<Vec<(Node, &'static str, wasm_bindgen::JsValue)>> = Default::default();
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Node(u32);

impl Dom {
    fn next_id() -> Node {
        Node(NODE_ID.with(|n| {
            let curr = n.get();
            n.set(curr + 1);
            curr + 1
        }))
    }

    fn add_listener(node: &Node, event: &'static str, cb: Box<dyn FnMut(web_sys::Event)>) {
        LISTENERS.with(|l| {
            l.borrow_mut().push((
                *node,
                event,
                wasm_bindgen::closure::Closure::wrap(cb).into_js_value(),
            ))
        });
    }

    pub fn body() -> Node {
        Node(0)
    }

    pub fn create_element(ty: El) -> Node {
        let id = Dom::next_id();
        CHANNEL.with(|c| c.borrow_mut().create_element(id.0, ty as u8));
        id
    }

    pub fn create_text_node(data: &str) -> Node {
        let id = Dom::next_id();
        CHANNEL.with(|c| c.borrow_mut().create_text_node(id.0, data));
        id
    }

    pub fn insert_before(parent: Node, a: Node, b: Node) {
        CHANNEL.with(|c| c.borrow_mut().insert_before(parent.0, a.0, b.0));
    }

    pub fn flush() {
        CHANNEL.with(|c| c.borrow_mut().flush());
        // TODO event delegation instead
        LISTENERS.with(|listeners| {
            for (node, event, cb) in listeners.take() {
                use wasm_bindgen::JsCast;
                let el = node.as_web_sys().unchecked_into::<web_sys::Element>();
                el.add_event_listener_with_callback(event, cb.unchecked_ref());
            }
        })
    }
}

impl Node {
    pub fn as_web_sys(&self) -> web_sys::Node {
        get_node(self.0)
    }

    pub fn append_child(&self, child: Node) {
        CHANNEL.with(|c| c.borrow_mut().append_child(self.0, child.0))
    }

    pub fn set_data(&self, text: &str) {
        CHANNEL.with(|c| c.borrow_mut().set_data(self.0, text))
    }

    pub fn set_text(&self, text: &str) {
        CHANNEL.with(|c| c.borrow_mut().set_text(self.0, text))
    }

    pub fn set_attribute(&self, key: Attr, value: &str) {
        CHANNEL.with(|c| c.borrow_mut().set_attribute(self.0, key as u8, value))
    }

    pub fn remove(&self) {
        CHANNEL.with(|c| c.borrow_mut().remove(self.0))
    }

    pub fn add_event_listener(&self, event: &'static str, cb: Box<dyn FnMut(web_sys::Event)>) {
        Dom::add_listener(self, event, cb);
    }
}

macro_rules! html_elements {
	($($tag:ident),* $(,)?) => {
        $(pub fn $tag<A: $crate::view::html::attribute::Attribute, C: $crate::View>(attributes: A, children: C) -> $crate::view::html::Html<A, C> {
            $crate::view::html::Html {
                tag: $crate::dom::El::$tag,
                attributes,
                children
            }
        })*

        #[allow(non_camel_case_types)]
        #[repr(u8)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum El {
            $($tag,)*
        }
    }
}

html_elements! {
    a,
    abbr,
    acronym,
    address,
    applet,
    area,
    article,
    aside,
    audio,
    b,
    base,
    bdi,
    bdo,
    bgsound,
    big,
    blink,
    blockquote,
    body,
    br,
    button,
    canvas,
    caption,
    center,
    cite,
    code,
    col,
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
    embed,
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
    hr,
    html,
    i,
    iframe,
    image,
    img,
    input,
    ins,
    kbd,
    keygen,
    label,
    legend,
    li,
    link,
    main,
    map,
    mark,
    marquee,
    menu,
    menuitem,
    meta,
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
    param,
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
    source,
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
    track,
    tt,
    u,
    ul,
    var,
    video,
    wbr,
    xmp
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Attr {
    accept_charset,
    accept,
    accesskey,
    action,
    align,
    allow,
    alt,
    aria_atomic,
    aria_busy,
    aria_controls,
    aria_current,
    aria_describedby,
    aria_description,
    aria_details,
    aria_disabled,
    aria_dropeffect,
    aria_errormessage,
    aria_flowto,
    aria_grabbed,
    aria_haspopup,
    aria_hidden,
    aria_invalid,
    aria_keyshortcuts,
    aria_label,
    aria_labelledby,
    aria_live,
    aria_owns,
    aria_relevant,
    aria_roledescription,
    r#async,
    autocapitalize,
    autocomplete,
    autofocus,
    autoplay,
    background,
    bgcolor,
    border,
    buffered,
    capture,
    challenge,
    charset,
    checked,
    cite,
    class,
    code,
    codebase,
    color,
    cols,
    colspan,
    content,
    contenteditable,
    contextmenu,
    controls,
    coords,
    crossorigin,
    csp,
    data,
    datetime,
    decoding,
    default,
    defer,
    dir,
    dirname,
    disabled,
    download,
    draggable,
    enctype,
    enterkeyhint,
    r#for,
    form,
    formaction,
    formenctype,
    formmethod,
    formnovalidate,
    formtarget,
    headers,
    height,
    hidden,
    high,
    href,
    hreflang,
    http_equiv,
    icon,
    id,
    importance,
    inputmode,
    integrity,
    intrinsicsize,
    ismap,
    itemprop,
    keytype,
    kind,
    label,
    lang,
    language,
    list,
    loading,
    r#loop,
    low,
    manifest,
    max,
    maxlength,
    media,
    method,
    min,
    minlength,
    multiple,
    muted,
    name,
    novalidate,
    open,
    optimum,
    pattern,
    ping,
    placeholder,
    poster,
    preload,
    radiogroup,
    readonly,
    referrerpolicy,
    rel,
    required,
    reversed,
    role,
    rows,
    rowspan,
    sandbox,
    scope,
    scoped,
    selected,
    shape,
    size,
    sizes,
    slot,
    span,
    spellcheck,
    src,
    srcdoc,
    srclang,
    srcset,
    start,
    step,
    style,
    summary,
    tabindex,
    target,
    title,
    translate,
    r#type,
    usemap,
    value,
    width,
    wrap,
}
