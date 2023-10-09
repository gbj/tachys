use crate::{
    html::{
        attribute::{global::AddAttribute, *},
        class::{Class, IntoClass},
        element::{CreateElement, ElementChild, ElementType, HtmlElement},
    },
    renderer::{dom::Dom, DomRenderer, Renderer},
    tuple_builder::TupleBuilder,
    view::Render,
};
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
                        pub fn [<$attr:lower>]<V>(self, value: V) -> [<Html $tag:camel>] <
                            <At as TupleBuilder<Attr<$crate::html::attribute::$attr, V, Rndr>>>::Output,
                            Ch, Rndr
                        >
                        where
                            V: AttributeValue<Rndr>,
                            At: TupleBuilder<Attr<$crate::html::attribute::$attr, V, Rndr>>,
                            <At as TupleBuilder<Attr<$crate::html::attribute::$attr, V, Rndr>>>::Output: Attribute<Rndr>,
                        {
                            let [<Html $tag:camel>] {
                                attributes,
                                children,
                                rndr
                            } = self;
                            [<Html $tag:camel>] {
                                attributes: attributes.next_tuple($crate::html::attribute::[<$attr:lower>](value)),
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
            )*
		}
    }
}

macro_rules! html_self_closing_elements {
	($($tag:ident [$($attr:ty),*]),* $(,)?) => {
        paste::paste! {
            $(
                pub fn $tag<At, Rndr>(attributes: At) -> HtmlElement<[<$tag:camel>], At, (), Rndr>
                where
                    At: Attribute<Rndr>,
                    Rndr: Renderer
                {
                    HtmlElement {
                        ty: PhantomData,
                        rndr: PhantomData,
                        attributes,
                        children: ()
                    }
                }

                #[derive(Debug, Copy, Clone, PartialEq, Eq)]
                pub struct [<$tag:camel>];

                impl ElementType for [<$tag:camel>] {
                    const TAG: &'static str = stringify!($tag);
                    const SELF_CLOSING: bool = true;
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

                $(
                    impl<At, Ch, Rndr> HtmlElement<[<$tag:camel>], At, Ch, Rndr>
                    where
                        At: Attribute<Rndr>,
                        Ch: Render<Rndr>,
                        Rndr: Renderer,
                    {
                        pub fn [<$attr:lower>]<V>(self, value: V) -> HtmlElement<[<$tag:camel>], <At as TupleBuilder<Attr<$crate::html::attribute::$attr, V, Rndr>>>::Output, Ch, Rndr>
                        where
                            V: AttributeValue<Rndr>,
                            At: TupleBuilder<Attr<$crate::html::attribute::$attr, V, Rndr>>,
                            <At as TupleBuilder<Attr<$crate::html::attribute::$attr, V, Rndr>>>::Output: Attribute<Rndr>,
                        {
                            self.attr($crate::html::attribute::[<$attr:lower>](value))
                        }
                    }
                )*
            )*
		}
    }
}

/* html_self_closing_elements! {
    area [Alt, Coords, Download, Href, Hreflang, Ping, Rel, Shape, Target],
    base [Href, Target],
    br [],
    col [Span],
    embed [Height, Src, /* Type, */Width],
    hr [],
    img [Alt, Crossorigin, Decoding, Height, Ismap, Sizes, Src, Srcset, Usemap, Width],
    input [Accept, Alt, Autocomplete, Capture, Checked, Disabled, Form, Formaction, Formenctype, Formmethod, Formnovalidate, Formtarget, Height, List, Max, Maxlength, Min, Minlength, Multiple, Name, Pattern, Placeholder, Readonly, Required, Size, Src, Step, /* Type, */Value, Width],
    link [/* As, */ Crossorigin, Href, Hreflang, Media, Rel, Sizes/* , Type */],
    meta [Charset, Content, /* HttpEquiv, */ Name],
    source [Src/* , Type */],
    track [Default, Kind, Label, Src, Srclang],
    wbr []
} */

html_elements! {
/*    a [Download, Href, Hreflang, Ping, Rel, Target/* , Type */],
   abbr [],
   address [],
   article [],
   aside [],
   audio [Autoplay, Controls, Crossorigin, /* Loop, */ Muted, Preload, Src],
   b [],
   bdi [],
   bdo [],
   blink [],
   blockquote [Cite],
   body [], */
   button [Disabled, Form, Formaction, Formenctype, Formmethod, Formnovalidate, Formtarget, Name, /* Type, */Value],
   /* canvas [Height, Width],
   caption [],
   cite [],
   code [],
   colgroup [Span],
   data [Value],
   datalist [],
   dd [],
   del [Cite, Datetime],
   details [Open],
   dfn [],
   dialog [Open],
   div [],
   dl [],
   dt [],*/
   em [],
   /*fieldset [],
   figcaption [],
   figure [],
   footer [],
   form [AcceptCharset, Action, Autocomplete, Enctype, Method, Name, Novalidate, Target],
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
   iframe [Allow, Allowfullscreen, Allowpaymentrequest, Height, Name, Referrerpolicy, Sandbox, Src, Srcdoc, Width],
   ins [Cite, Datetime],
   kbd [],
   label [/* For, */ Form],
   legend [],
   li [Value],
 */   main [],
/*    map [Name],
   mark [],
   menu [],
   meter [Value, Min, Max, Low, High, Optimum, Form],
   nav [],
   noscript [],
   object [Data, Form, Height, Name, /* Type, */Usemap, Width],
   ol [Reversed, Start/* , Type */],
   optgroup [Disabled, Label],
   // option, // creates conflict with core Option
   output [/* For, */ Form, Name], */
   p [],
   /* picture [],
   portal [Referrerpolicy, Src],
   pre [],
   progress [Max, Value],
   q [Cite],
   rp [],
   rt [],
   ruby [],
   s [],
   samp [],
   script [/* Async, */ Crossorigin, Defer, Fetchpriority, Integrity, Nomodule, Referrerpolicy, Src, /* Type, */Blocking],
   search [],
   section [],
   select [Autocomplete, Disabled, Form, Multiple, Name, Required, Size],
   slot [Name],
   small [],*/
   span [],
   /*strong [],
   style [Media, Blocking],
   sub [],
   summary [],
   sup [],
   table [],
   tbody [],
   td [Colspan, Headers, Rowspan],
   template [],
   textarea [Autocomplete, Cols, Dirname, Disabled, Form, Maxlength, Minlength, Name, Placeholder, Readonly, Required, Rows, Wrap],
   tfoot [],
   th [Abbr, Colspan, Headers, Rowspan, Scope],
   thead [],
   time [Datetime],
   title [],
   tr [],
   u [],
   ul [],
   var [],
   video [Controls, Controlslist, Crossorigin, Disablepictureinpicture, Disableremoteplayback, Height, /* Loop, */ Muted, Playsinline, Poster, Preload, Src, Width],
 */}

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
