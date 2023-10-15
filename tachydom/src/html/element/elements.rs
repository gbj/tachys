use crate::{
    html::{
        attribute::{global::AddAttribute, *},
        class::{Class, IntoClass},
        element::{CreateElement, ElementChild, ElementType, HtmlElement},
    },
    hydration::Cursor,
    renderer::{dom::Dom, DomRenderer, Renderer},
    view::{PositionState, Render, RenderHtml},
};
use next_tuple::TupleBuilder;
use once_cell::unsync::Lazy;
use std::{fmt::Debug, marker::PhantomData};

macro_rules! html_elements {
	($($tag:ident  [$($attr:ty),*]),* $(,)?) => {
        paste::paste! {
            $(
                // `tag()` function
                pub fn $tag<Rndr>() -> [<Html $tag:camel>]<(), (), Rndr>
                where
                    Rndr: Renderer
                {
                     [<Html $tag:camel>] {
                        attributes: (),
                        children: (),
                        rndr: PhantomData,
                    }
                }

                #[derive(Debug, Copy, Clone, PartialEq, Eq)]
                pub struct [<Html $tag:camel>]<At, Ch, Rndr>
                where
                    At: Attribute<Rndr>,
                    Ch: Render<Rndr>,
                    Rndr: Renderer {
                    attributes: At,
                    children: Ch,
                    rndr: PhantomData<Rndr>
                }

                // .child()
                impl<At, Ch, NewChild, Rndr> ElementChild<NewChild> for [<Html $tag:camel>]<At, Ch, Rndr>
                where
                    At: Attribute<Rndr>,
                    Ch: Render<Rndr> + TupleBuilder<NewChild>,
                    <Ch as TupleBuilder<NewChild>>::Output: Render<Rndr>,
                    Rndr: Renderer
                {
                    type Output = [<Html $tag:camel>]<At, <Ch as TupleBuilder<NewChild>>::Output, Rndr>;

                    fn child(
                        self,
                        child: NewChild,
                    ) -> Self::Output
                    {
                        let [<Html $tag:camel>] {
                            attributes,
                            children,
                            rndr
                        } = self;
                        [<Html $tag:camel>] {
                            attributes,
                            children: children.next_tuple(child),
                            rndr
                        }
                    }
                }

                // Typed attribute methods
                $(
                    impl<At, Ch, Rndr> [<Html $tag:camel>]<At, Ch, Rndr>
                    where
                        At: Attribute<Rndr>,
                        Ch: Render<Rndr>,
                        Rndr: Renderer,
                    {
                        pub fn $attr<V>(self, value: V) -> [<Html $tag:camel>] <
                            <At as TupleBuilder<Attr<$crate::html::attribute::[<$attr:camel>], V, Rndr>>>::Output,
                            Ch, Rndr
                        >
                        where
                            V: AttributeValue<Rndr>,
                            At: TupleBuilder<Attr<$crate::html::attribute::[<$attr:camel>], V, Rndr>>,
                            <At as TupleBuilder<Attr<$crate::html::attribute::[<$attr:camel>], V, Rndr>>>::Output: Attribute<Rndr>,
                        {
                            let [<Html $tag:camel>] {
                                attributes,
                                children,
                                rndr
                            } = self;
                            [<Html $tag:camel>] {
                                attributes: attributes.next_tuple($crate::html::attribute::$attr(value)),
                                children,
                                rndr
                            }
                        }
                    }
                )*

                // Global Attributes
                impl<At, Ch, Rndr, NewAttr> AddAttribute<NewAttr, Rndr> for [<Html $tag:camel>]<At, Ch, Rndr>
                where
                    At: Attribute<Rndr> + TupleBuilder<NewAttr>,
                    <At as TupleBuilder<NewAttr>>::Output: Attribute<Rndr>,
                    Ch: Render<Rndr>,
                    Rndr: Renderer
                {
                    type Output = [<Html $tag:camel>]<<At as TupleBuilder<NewAttr>>::Output, Ch, Rndr>;

                    fn add_attr(self, attr: NewAttr) -> Self::Output {
                        let [<Html $tag:camel>] {
                            attributes,
                            children,
                            rndr
                        } = self;
                        [<Html $tag:camel>] {
                            attributes: attributes.next_tuple(attr),
                            children,
                            rndr
                        }
                    }
                }

                // Element creation
                #[derive(Debug, Copy, Clone, PartialEq, Eq)]
                pub struct [<$tag:camel>];

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

                // Render and RenderHtml implementations simply delegate to HtmlElement
                impl<At, Ch, Rndr> Render<Rndr> for [<Html $tag:camel>]<At, Ch, Rndr>
                where
                    At: Attribute<Rndr>,
                    Ch: Render<Rndr>,
                    Rndr: Renderer,
                    Rndr::Node: Clone,
                    HtmlElement<[<$tag:camel>], At, Ch, Rndr>: Render<Rndr>
                {
                    type State = <HtmlElement<[<$tag:camel>], At, Ch, Rndr> as Render<Rndr>>::State;

                    fn build(self) -> Self::State {
                        let [<Html $tag:camel>] {
                            attributes,
                            children,
                            rndr
                        } = self;
                        HtmlElement {
                            attributes,
                            children,
                            ty: PhantomData,
                            rndr,
                        }.build()
                    }

                    fn rebuild(self, state: &mut Self::State) {
                        let [<Html $tag:camel>] {
                            attributes,
                            children,
                            rndr
                        } = self;
                        HtmlElement {
                            attributes,
                            children,
                            ty: PhantomData,
                            rndr,
                        }.rebuild(state);
                    }
                }

                impl<At, Ch, Rndr> RenderHtml<Rndr> for [<Html $tag:camel>]<At, Ch, Rndr>
                where
                    At: Attribute<Rndr>,
                    Ch: Render<Rndr>,
                    Rndr: Renderer,
                    Rndr::Node: Clone,
                    Rndr::Element: Clone,
                    HtmlElement<[<$tag:camel>], At, Ch, Rndr>: RenderHtml<Rndr>
                {
                    fn to_html(self, buf: &mut String, position: &PositionState) {
                        let [<Html $tag:camel>] {
                            attributes,
                            children,
                            rndr
                        } = self;
                        HtmlElement {
                            attributes,
                            children,
                            ty: PhantomData,
                            rndr,
                        }.to_html(buf, position)
                    }

                    fn hydrate<const FROM_SERVER: bool>(
                        self,
                        cursor: &Cursor<Rndr>,
                        position: &PositionState,
                    ) -> Self::State {
                        let [<Html $tag:camel>] {
                            attributes,
                            children,
                            rndr
                        } = self;
                        HtmlElement {
                            attributes,
                            children,
                            ty: PhantomData,
                            rndr,
                        }.hydrate::<FROM_SERVER>(cursor, position)
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
                pub fn $tag<Rndr>() -> [<Html $tag:camel>]<(), Rndr>
                where
                    Rndr: Renderer
                {
                     [<Html $tag:camel>] {
                        attributes: (),
                        rndr: PhantomData,
                    }
                }

                #[derive(Debug, Copy, Clone, PartialEq, Eq)]
                pub struct [<Html $tag:camel>]<At, Rndr>
                where
                    At: Attribute<Rndr>,
                    Rndr: Renderer {
                    attributes: At,
                    rndr: PhantomData<Rndr>
                }

                // Typed attribute methods
                $(
                    impl<At, Rndr> [<Html $tag:camel>]<At, Rndr>
                    where
                        At: Attribute<Rndr>,
                        Rndr: Renderer,
                    {
                        pub fn $attr<V>(self, value: V) -> [<Html $tag:camel>] <
                            <At as TupleBuilder<Attr<$crate::html::attribute::[<$attr:camel>], V, Rndr>>>::Output,
                            Rndr
                        >
                        where
                            V: AttributeValue<Rndr>,
                            At: TupleBuilder<Attr<$crate::html::attribute::[<$attr:camel>], V, Rndr>>,
                            <At as TupleBuilder<Attr<$crate::html::attribute::[<$attr:camel>], V, Rndr>>>::Output: Attribute<Rndr>,
                        {
                            let [<Html $tag:camel>] {
                                attributes,
                                rndr
                            } = self;
                            [<Html $tag:camel>] {
                                attributes: attributes.next_tuple($crate::html::attribute::$attr(value)),
                                rndr
                            }
                        }
                    }
                )*

                // Global Attributes
                impl<At, Rndr, NewAttr> AddAttribute<NewAttr, Rndr> for [<Html $tag:camel>]<At, Rndr>
                where
                    At: Attribute<Rndr> + TupleBuilder<NewAttr>,
                    <At as TupleBuilder<NewAttr>>::Output: Attribute<Rndr>,
                    Rndr: Renderer
                {
                    type Output = [<Html $tag:camel>]<<At as TupleBuilder<NewAttr>>::Output, Rndr>;

                    fn add_attr(self, attr: NewAttr) -> Self::Output {
                        let [<Html $tag:camel>] {
                            attributes,
                            rndr
                        } = self;
                        [<Html $tag:camel>] {
                            attributes: attributes.next_tuple(attr),
                            rndr
                        }
                    }
                }

                // Element creation
                #[derive(Debug, Copy, Clone, PartialEq, Eq)]
                pub struct [<$tag:camel>];

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

                // Render and RenderHtml implementations simply delegate to HtmlElement
                impl<At, Rndr> Render<Rndr> for [<Html $tag:camel>]<At, Rndr>
                where
                    At: Attribute<Rndr>,
                    Rndr: Renderer,
                    Rndr::Node: Clone,
                    HtmlElement<[<$tag:camel>], At, (), Rndr>: Render<Rndr>
                {
                    type State = <HtmlElement<[<$tag:camel>], At, (), Rndr> as Render<Rndr>>::State;

                    fn build(self) -> Self::State {
                        let [<Html $tag:camel>] {
                            attributes,
                            rndr
                        } = self;
                        HtmlElement {
                            attributes,
                            children: (),
                            ty: PhantomData,
                            rndr,
                        }.build()
                    }

                    fn rebuild(self, state: &mut Self::State) {
                        let [<Html $tag:camel>] {
                            attributes,
                            rndr
                        } = self;
                        HtmlElement {
                            attributes,
                            children: (),
                            ty: PhantomData,
                            rndr,
                        }.rebuild(state);
                    }
                }

                impl<At, Rndr> RenderHtml<Rndr> for [<Html $tag:camel>]<At, Rndr>
                where
                    At: Attribute<Rndr>,
                    Rndr: Renderer,
                    Rndr::Node: Clone,
                    Rndr::Element: Clone,
                    HtmlElement<[<$tag:camel>], At, (), Rndr>: RenderHtml<Rndr>
                {
                    fn to_html(self, buf: &mut String, position: &PositionState) {
                        let [<Html $tag:camel>] {
                            attributes,
                            rndr
                        } = self;
                        HtmlElement {
                            attributes,
                            children: (),
                            ty: PhantomData,
                            rndr,
                        }.to_html(buf, position)
                    }

                    fn hydrate<const FROM_SERVER: bool>(
                        self,
                        cursor: &Cursor<Rndr>,
                        position: &PositionState,
                    ) -> Self::State {
                        let [<Html $tag:camel>] {
                            attributes,
                            rndr
                        } = self;
                        HtmlElement {
                            attributes,
                            children: (),
                            ty: PhantomData,
                            rndr,
                        }.hydrate::<FROM_SERVER>(cursor, position)
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
