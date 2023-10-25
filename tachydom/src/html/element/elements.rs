use crate::{
    html::{
        attribute::{Attr, Attribute, AttributeValue},
        element::{
            CreateElement, ElementType, ElementWithChildren, HtmlElement,
        },
    },
    renderer::{dom::Dom, Renderer},
    view::Render,
};
use next_tuple::TupleBuilder;
use once_cell::unsync::Lazy;
use std::{fmt::Debug, marker::PhantomData};

macro_rules! html_elements {
	($($tag:ident  [$($attr:ty),*]),* $(,)?) => {
        paste::paste! {
            $(
                // `tag()` function
                pub fn $tag<Rndr>() -> HtmlElement<[<$tag:camel>], (), (), Rndr>
                where
                    Rndr: Renderer
                {
                    HtmlElement {
                        ty: PhantomData,
                        attributes: (),
                        children: (),
                        rndr: PhantomData,
                    }
                }

                #[derive(Debug, Copy, Clone, PartialEq, Eq)]
                pub struct [<$tag:camel>];

                // Element creation
                //#[derive(Debug, Copy, Clone, PartialEq, Eq)]
                //pub struct [<$tag:camel>];

                                // Typed attribute methods
                $(
                    impl<At, Ch, Rndr> HtmlElement<[<$tag:camel>], At, Ch, Rndr>
                    where
                        At: Attribute<Rndr>,
                        Ch: Render<Rndr>,
                        Rndr: Renderer,
                    {
                        pub fn $attr<V>(self, value: V) -> HtmlElement <
                            [<$tag:camel>],
                            <At as TupleBuilder<Attr<$crate::html::attribute::[<$attr:camel>], V, Rndr>>>::Output,
                            Ch, Rndr
                        >
                        where
                            V: AttributeValue<Rndr>,
                            At: TupleBuilder<Attr<$crate::html::attribute::[<$attr:camel>], V, Rndr>>,
                            <At as TupleBuilder<Attr<$crate::html::attribute::[<$attr:camel>], V, Rndr>>>::Output: Attribute<Rndr>,
                        {
                            let HtmlElement { ty, rndr, children, attributes } = self;
                            HtmlElement {
                                ty,
                                rndr,
                                children,
                                attributes: attributes.next_tuple($crate::html::attribute::$attr(value))
                            }
                        }
                    }
                )*

                impl ElementType for [<$tag:camel>] {
                    const TAG: &'static str = stringify!($tag);
                    const SELF_CLOSING: bool = false;
                }

                impl ElementWithChildren for [<$tag:camel>] {}

                impl CreateElement<Dom> for [<$tag:camel>] {
                    fn create_element() -> <Dom as Renderer>::Element {
                        use wasm_bindgen::JsCast;

                        thread_local! {
                            static ELEMENT: Lazy<<Dom as Renderer>::Element> = Lazy::new(|| {
                                crate::dom::document().create_element(stringify!($tag)).unwrap()
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
	($($tag:ident  [$($attr:ty),*]),* $(,)?) => {
        paste::paste! {
            $(
                // `tag()` function
                pub fn $tag<Rndr>() -> HtmlElement<[<$tag:camel>], (), (), Rndr>
                where
                    Rndr: Renderer
                {
                    HtmlElement {
                        attributes: (),
                        children: (),
                        rndr: PhantomData,
                        ty: PhantomData
                    }
                }

                #[derive(Debug, Copy, Clone, PartialEq, Eq)]
                pub struct [<$tag:camel>];

                // Typed attribute methods
                $(
                    impl<At, Rndr> HtmlElement<[<$tag:camel>], At, (), Rndr>
                    where
                        At: Attribute<Rndr>,
                        Rndr: Renderer,
                    {
                        pub fn $attr<V>(self, value: V) -> HtmlElement<
                            [<$tag:camel>],
                            <At as TupleBuilder<Attr<$crate::html::attribute::[<$attr:camel>], V, Rndr>>>::Output,
                            (),
                            Rndr
                        >
                        where
                            V: AttributeValue<Rndr>,
                            At: TupleBuilder<Attr<$crate::html::attribute::[<$attr:camel>], V, Rndr>>,
                            <At as TupleBuilder<Attr<$crate::html::attribute::[<$attr:camel>], V, Rndr>>>::Output: Attribute<Rndr>,
                        {
                            let HtmlElement { ty, rndr, children, attributes } = self;
                            HtmlElement {
                                ty,
                                rndr,
                                children,
                                attributes: attributes.next_tuple($crate::html::attribute::$attr(value))
                            }
                        }
                    }
                )*

                // Element creation
                //#[derive(Debug, Copy, Clone, PartialEq, Eq)]
                //pub struct [<$tag:camel>];

                impl ElementType for [<$tag:camel>] {
                    const TAG: &'static str = stringify!($tag);
                    const SELF_CLOSING: bool = false;
                }

                impl CreateElement<Dom> for [<$tag:camel>] {
                    fn create_element() -> <Dom as Renderer>::Element {
                        use wasm_bindgen::JsCast;

                        thread_local! {
                            static ELEMENT: Lazy<<Dom as Renderer>::Element> = Lazy::new(|| {
                                crate::dom::document().create_element(stringify!($tag)).unwrap()
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
    area [alt, coords, download, href, hreflang, ping, rel, shape, target],
    base [href, target],
    br [],
    col [span],
    embed [height, src, r#type, width],
    hr [],
    img [alt, crossorigin, decoding, height, ismap, sizes, src, srcset, usemap, width],
    input [accept, alt, autocomplete, capture, checked, disabled, form, formaction, formenctype, formmethod, formnovalidate, formtarget, height, list, max, maxlength, min, minlength, multiple, name, pattern, placeholder, readonly, required, size, src, step, r#type, value, width],
    link [r#as, crossorigin, href, hreflang, media, rel, sizes, r#type],
    meta [charset, content, http_equiv, name],
    source [src, r#type],
    track [default, kind, label, src, srclang],
    wbr []
}

html_elements! {
  a [download, href, hreflang, ping, rel, target, r#type ],
  abbr [],
  address [],
  article [],
  aside [],
  audio [autoplay, controls, crossorigin, r#loop, muted, preload, src],
  b [],
  bdi [],
  bdo [],
  blink [],
  blockquote [cite],
  body [],
  button [disabled, form, formaction, formenctype, formmethod, formnovalidate, formtarget, name, r#type, value],
  canvas [height, width],
  caption [],
  cite [],
  code [],
  colgroup [span],
  data [value],
  datalist [],
  dd [],
  del [cite, datetime],
  details [open],
  dfn [],
  dialog [open],
  div [],
  dl [],
  dt [],
  em [],
  fieldset [],
  figcaption [],
  figure [],
  footer [],
  form [accept_charset, action, autocomplete, enctype, method, name, novalidate, target],
  h1 [],
  h2 [],
  h3 [],
  h4 [],
  h5 [],
  h6 [],
  head [],
  header [],
  hgroup [],
  html [],
  i [],
  iframe [allow, allowfullscreen, allowpaymentrequest, height, name, referrerpolicy, sandbox, src, srcdoc, width],
  ins [cite, datetime],
  kbd [],
  label [r#for, form],
  legend [],
  li [value],
   main [],
   map [name],
  mark [],
  menu [],
  meter [value, min, max, low, high, optimum, form],
  nav [],
  noscript [],
  object [data, form, height, name, r#type, usemap, width],
  ol [reversed, start, r#type],
  optgroup [disabled, label],
  // option, // creates conflict with core Option
  output [r#for, form, name],
  p [],
  picture [],
  portal [referrerpolicy, src],
  pre [],
  progress [max, value],
  q [cite],
  rp [],
  rt [],
  ruby [],
  s [],
  samp [],
  script [r#async, crossorigin, defer, fetchpriority, integrity, nomodule, referrerpolicy, src, r#type, blocking],
  search [],
  section [],
  select [autocomplete, disabled, form, multiple, name, required, size],
  slot [name],
  small [],
  span [],
  strong [],
  style [media, blocking],
  sub [],
  summary [],
  sup [],
  table [],
  tbody [],
  td [colspan, headers, rowspan],
  template [],
  textarea [autocomplete, cols, dirname, disabled, form, maxlength, minlength, name, placeholder, readonly, required, rows, wrap],
  tfoot [],
  th [abbr, colspan, headers, rowspan, scope],
  thead [],
  time [datetime],
  title [],
  tr [],
  u [],
  ul [],
  var [],
  video [controls, controlslist, crossorigin, disablepictureinpicture, disableremoteplayback, height, r#loop, muted, playsinline, poster, preload, src, width],
}

pub fn option<At, Ch, Rndr>(
    attributes: At,
    children: Ch,
) -> HtmlElement<Option_, At, Ch, Rndr>
where
    At: Attribute<Rndr>,
    Ch: Render<Rndr>,
    Rndr: Renderer,
{
    HtmlElement {
        ty: PhantomData,
        rndr: PhantomData,
        attributes,
        children,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Option_;

impl ElementType for Option_ {
    const TAG: &'static str = "option";
    const SELF_CLOSING: bool = false;
}

impl CreateElement<Dom> for Option_ {
    fn create_element() -> <Dom as Renderer>::Element {
        use wasm_bindgen::JsCast;

        thread_local! {
            static ELEMENT: Lazy<<Dom as Renderer>::Element> = Lazy::new(|| {
                crate::dom::document().create_element("option").unwrap()
            });
        }
        ELEMENT.with(|e| e.clone_node()).unwrap().unchecked_into()
    }
}
