use crate::{
    html::{
        attribute::*,
        class::{Class, IntoClass},
        element::{
            CreateElement, Element, ElementChild, ElementType,
            GlobalAttributes, HtmlAttribute, HtmlElement,
        },
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
                // `tag()`

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
                    At: Attribute<Rndr> + HtmlAttribute<[<$tag:camel>]>,
                    Ch: Render<Rndr>,
                    Rndr: Renderer {
                    attributes: At,
                    children: Ch,
                    rndr: PhantomData<Rndr>
                }

                // .child()

                impl<At, Ch, NewChild, Rndr> ElementChild<NewChild> for [<Html $tag:camel>]<At, Ch, Rndr>
                where
                    At: Attribute<Rndr> + HtmlAttribute<[<$tag:camel>]>,
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
                        At: Attribute<Rndr> + HtmlAttribute<[<$tag:camel>]>,
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
                            <At as TupleBuilder<Attr<$crate::html::attribute::$attr, V, Rndr>>>::Output: Attribute<Rndr> + HtmlAttribute<[<$tag:camel>]>,
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
                impl<At, Ch, Rndr> Element<Rndr> for [<Html $tag:camel>]<At, Ch, Rndr>
                where
                    At: Attribute<Rndr> + HtmlAttribute<[<$tag:camel>]>,
                    Ch: Render<Rndr>,
                    Rndr: Renderer,
                {
                    type Attributes = At;
                    type Children = Ch;
                }

                impl<At, Ch, Rndr> GlobalAttributes<Self, At, Ch, Rndr> for [<Html $tag:camel>]<At, Ch, Rndr>
                where
                    Self: Sized,
                    At: Attribute<Rndr> + HtmlAttribute<[<$tag:camel>]>,
                    Ch: Render<Rndr>,
                    Rndr: Renderer {
                    fn set_attr<NewAttr>(
                        self,
                        new_attr: NewAttr,
                    ) -> impl Element<
                        Rndr,
                        Attributes = <At as TupleBuilder<NewAttr>>::Output,
                        Children = Ch,
                    >
                    where
                        At: TupleBuilder<NewAttr> + HtmlAttribute<[<$tag:camel>]>,
                        <At as TupleBuilder<NewAttr>>::Output: Attribute<Rndr> + HtmlAttribute<[<$tag:camel>]> {
                        let [<Html $tag:camel>] {
                            attributes,
                            children,
                            rndr
                        } = self;
                        [<Html $tag:camel>] {
                            attributes: attributes.next_tuple(new_attr),
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

                // Typed attributes
                build_attributes! { [<$tag:camel>] }
                $(impl HtmlAttribute<[<$tag:camel>]> for $crate::html::attribute::$attr {})*

                // Render and RenderHtml implementations simply delegate to HtmlElement
                impl<At, Ch, Rndr> Render<Rndr> for [<Html $tag:camel>]<At, Ch, Rndr>
                where
                    At: Attribute<Rndr> + HtmlAttribute<[<$tag:camel>]>,
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
                    At: Attribute<Rndr> + HtmlAttribute<[<$tag:camel>]>,
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

                build_attributes! { [<$tag:camel>] }
                $(impl HtmlAttribute<[<$tag:camel>]> for $crate::html::attribute::$attr {})*

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

macro_rules! build_attributes {
    ($el:ident) => {
        // Support all global attributes
        impl<T: GlobalAttribute> HtmlAttribute<$el> for T {}

        // Support all specified attributes.
        impl<K, V, R> HtmlAttribute<$el>
            for $crate::html::attribute::Attr<K, V, R>
        where
            K: $crate::html::attribute::AttributeKey + HtmlAttribute<$el>,
            V: $crate::html::attribute::AttributeValue<R>,
            R: Renderer,
        {
        }

        // Support no attributes.
        impl HtmlAttribute<$el> for () {}
    };
}

macro_rules! impl_attr_trait_for_tuple {
    ($first:ident, $($ty:ident),* $(,)?) => {
        impl<El: ElementType, $first, $($ty),*> HtmlAttribute<El> for ($first, $($ty,)*)
            where $first: HtmlAttribute<El>, $($ty: HtmlAttribute<El>,)*
            {}
    };
}

// Support tuples of attributes.
impl<E: ElementType, A: HtmlAttribute<E>> HtmlAttribute<E> for (A,) {}
impl_attr_trait_for_tuple!(A, B);
impl_attr_trait_for_tuple!(A, B, C);
impl_attr_trait_for_tuple!(A, B, C, D);
impl_attr_trait_for_tuple!(A, B, C, D, E);
impl_attr_trait_for_tuple!(A, B, C, D, E, F);
impl_attr_trait_for_tuple!(A, B, C, D, E, F, G);
impl_attr_trait_for_tuple!(A, B, C, D, E, F, G, H);
impl_attr_trait_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_attr_trait_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_attr_trait_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_attr_trait_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_attr_trait_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_attr_trait_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_attr_trait_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_attr_trait_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
impl_attr_trait_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_attr_trait_for_tuple!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R
);
impl_attr_trait_for_tuple!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S
);
impl_attr_trait_for_tuple!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T
);
impl_attr_trait_for_tuple!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U
);
impl_attr_trait_for_tuple!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V
);
impl_attr_trait_for_tuple!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W
);
impl_attr_trait_for_tuple!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X
);
impl_attr_trait_for_tuple!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y
);
impl_attr_trait_for_tuple!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y,
    Z
);

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
    At: Attribute<Rndr> + HtmlAttribute<Option_>,
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

build_attributes! { Option_ }
impl HtmlAttribute<Option_> for crate::html::attribute::Disabled {}
impl HtmlAttribute<Option_> for crate::html::attribute::Label {}
impl HtmlAttribute<Option_> for crate::html::attribute::Selected {}
impl HtmlAttribute<Option_> for crate::html::attribute::Value {}
