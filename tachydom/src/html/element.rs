use crate::{
    html::{
        attribute::{Attribute, GlobalAttribute},
        class::{Class, IntoClass},
    },
    hydration::Cursor,
    renderer::{dom::Dom, CastFrom, DomRenderer, Renderer},
    view::{
        Mountable, Position, PositionState, Render, RenderHtml, ToTemplate,
    },
};
use once_cell::unsync::Lazy;
use std::{fmt::Debug, marker::PhantomData};

pub struct HtmlElement<E, At, Ch, Rndr>
where
    At: Attribute<Rndr>,
    Ch: Render<Rndr>,
    Rndr: Renderer,
{
    ty: PhantomData<E>,
    rndr: PhantomData<Rndr>,
    attributes: At,
    children: Ch,
}

pub trait ElementType {
    const TAG: &'static str;
    const SELF_CLOSING: bool;
}

pub trait CreateElement<R: Renderer> {
    fn create_element() -> R::Element;
}

impl<E, At, Ch, Rndr> Render<Rndr> for HtmlElement<E, At, Ch, Rndr>
where
    E: CreateElement<Rndr>,
    At: Attribute<Rndr>,
    Ch: Render<Rndr>,
    Rndr: Renderer,
    Rndr::Node: Clone,
{
    type State = ElementState<At::State, Ch::State, Rndr>;

    fn rebuild(self, state: &mut Self::State) {
        let ElementState {
            attrs, children, ..
        } = state;
        self.attributes.rebuild(attrs);
        self.children.rebuild(children);
    }

    fn build(self) -> Self::State {
        let el = Rndr::create_element::<E>();
        let attrs = self.attributes.build(&el);
        let mut children = self.children.build();
        children.mount(&el, None);
        ElementState {
            el,
            attrs,
            children,
            rndr: PhantomData,
        }
    }
}

impl<E, At, Ch, Rndr> RenderHtml<Rndr> for HtmlElement<E, At, Ch, Rndr>
where
    E: ElementType + CreateElement<Rndr>,
    At: Attribute<Rndr>,
    Ch: RenderHtml<Rndr>,
    Rndr: Renderer,
    Rndr::Node: Clone,
    Rndr::Element: Clone,
{
    fn to_html(self, buf: &mut String, position: &PositionState) {
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
        cursor: &Cursor<Rndr>,
        position: &PositionState,
    ) -> Self::State {
        if position.get() == Position::FirstChild {
            cursor.child();
        } else {
            cursor.sibling();
        }
        let el = Rndr::Element::cast_from(cursor.current()).unwrap();

        let attrs = self.attributes.hydrate::<FROM_SERVER>(&el);

        // hydrate children
        position.set(Position::FirstChild);
        let children = self.children.hydrate::<FROM_SERVER>(cursor, position);
        cursor.set(el.as_ref().clone());

        // go to next sibling
        position.set(Position::NextChild);

        ElementState {
            el,
            attrs,
            children,
            rndr: PhantomData,
        }
    }
}

pub struct ElementState<At, Ch, R: Renderer> {
    pub el: R::Element,
    pub attrs: At,
    pub children: Ch,
    rndr: PhantomData<R>,
}

impl<At, Ch, R> Mountable<R> for ElementState<At, Ch, R>
where
    R: Renderer,
{
    fn unmount(&mut self) {
        R::remove(self.el.as_ref());
    }

    fn mount(&mut self, parent: &R::Element, marker: Option<&R::Node>) {
        R::insert_node(parent, self.el.as_ref(), marker);
    }

    fn insert_before_this(
        &self,
        parent: &<R as Renderer>::Element,
        child: &mut dyn Mountable<R>,
    ) -> bool {
        child.mount(parent, Some(self.el.as_ref()));
        true
    }
}

impl<E, At, Ch, Rndr> ToTemplate for HtmlElement<E, At, Ch, Rndr>
where
    E: ElementType,
    At: Attribute<Rndr> + ToTemplate,
    Ch: Render<Rndr> + ToTemplate,
    Rndr: Renderer,
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
	($($tag:ident  [$($attr:ty),*]),* $(,)?) => {
        paste::paste! {
            $(
                pub fn $tag<At, Ch, Rndr>(attributes: At, children: Ch) -> HtmlElement<[<$tag:camel>], At, Ch, Rndr>
                where
                    At: Attribute<Rndr> + [<$tag:camel Attribute>],
                    Ch: Render<Rndr>,
                    Rndr: Renderer
                {
                    HtmlElement {
                        ty: PhantomData,
                        rndr: PhantomData,
                        attributes,
                        children
                    }
                }

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

                build_attributes! { [<$tag:camel Attribute>]}
                $(impl [<$tag:camel Attribute>] for $crate::html::attribute::$attr {})*
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
                    At: Attribute<Rndr> + [<$tag:camel Attribute>],
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

                build_attributes! { [<$tag:camel Attribute>]}
                $(impl [<$tag:camel Attribute>] for $crate::html::attribute::$attr {})*
            )*
		}
    }
}

macro_rules! build_attributes {
    ($attr_trait:ident) => {
        /// Attributes for the `<stringify!($tag)>` element.
        pub trait $attr_trait {}

        // Support all global attributes
        impl<T: GlobalAttribute> $attr_trait for T {}

        // Support all specified attributes.
        impl<K, V, R> $attr_trait for $crate::html::attribute::Attr<K, V, R>
        where
            K: $crate::html::attribute::AttributeKey + $attr_trait,
            V: $crate::html::attribute::AttributeValue<R>,
            R: Renderer,
        {
        }

        // Support no attributes.
        impl $attr_trait for () {}

        // Support tuples of attributes.
        impl<A: $attr_trait> $attr_trait for (A,) {}
        impl_attr_trait_for_tuple!($attr_trait, A, B);
        impl_attr_trait_for_tuple!($attr_trait, A, B, C);
        impl_attr_trait_for_tuple!($attr_trait, A, B, C, D);
        impl_attr_trait_for_tuple!($attr_trait, A, B, C, D, E);
        impl_attr_trait_for_tuple!($attr_trait, A, B, C, D, E, F);
        impl_attr_trait_for_tuple!($attr_trait, A, B, C, D, E, F, G);
        impl_attr_trait_for_tuple!($attr_trait, A, B, C, D, E, F, G, H);
        impl_attr_trait_for_tuple!($attr_trait, A, B, C, D, E, F, G, H, I);
        impl_attr_trait_for_tuple!($attr_trait, A, B, C, D, E, F, G, H, I, J);
        impl_attr_trait_for_tuple!(
            $attr_trait,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K
        );
        impl_attr_trait_for_tuple!(
            $attr_trait,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L
        );
        impl_attr_trait_for_tuple!(
            $attr_trait,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M
        );
        impl_attr_trait_for_tuple!(
            $attr_trait,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N
        );
        impl_attr_trait_for_tuple!(
            $attr_trait,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O
        );
        impl_attr_trait_for_tuple!(
            $attr_trait,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P
        );
        impl_attr_trait_for_tuple!(
            $attr_trait,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q
        );
        impl_attr_trait_for_tuple!(
            $attr_trait,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R
        );
        impl_attr_trait_for_tuple!(
            $attr_trait,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
            S
        );
        impl_attr_trait_for_tuple!(
            $attr_trait,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
            S,
            T
        );
        impl_attr_trait_for_tuple!(
            $attr_trait,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
            S,
            T,
            U
        );
        impl_attr_trait_for_tuple!(
            $attr_trait,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
            S,
            T,
            U,
            V
        );
        impl_attr_trait_for_tuple!(
            $attr_trait,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
            S,
            T,
            U,
            V,
            W
        );
        impl_attr_trait_for_tuple!(
            $attr_trait,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
            S,
            T,
            U,
            V,
            W,
            X
        );
        impl_attr_trait_for_tuple!(
            $attr_trait,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
            S,
            T,
            U,
            V,
            W,
            X,
            Y
        );
        impl_attr_trait_for_tuple!(
            $attr_trait,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
            S,
            T,
            U,
            V,
            W,
            X,
            Y,
            Z
        );

        // Support classes and styles.
        impl<C, R> $attr_trait for Class<C, R>
        where
            C: IntoClass<R>,
            R: DomRenderer,
        {
        }

        impl<S, R> $attr_trait for $crate::html::style::Style<S, R>
        where
            S: $crate::html::style::IntoStyle<R>,
            R: DomRenderer,
        {
        }
    };
}

macro_rules! impl_attr_trait_for_tuple {
    ($attr_trait:ident, $first:ident, $($ty:ident),* $(,)?) => {
        impl<$first, $($ty),*> $attr_trait for ($first, $($ty,)*)
            where $first: $attr_trait, $($ty: $attr_trait,)*
            {}
    };
}

html_self_closing_elements! {
    area [Alt, Coords, Download, Href, Hreflang, Ping, Rel, Shape, Target],
    base [Href, Target],
    br [],
    col [Span],
    embed [Height, Src, Type, Width],
    hr [],
    img [Alt, Crossorigin, Decoding, Height, Ismap, Sizes, Src, Srcset, Usemap, Width],
    input [Accept, Alt, Autocomplete, Capture, Checked, Disabled, Form, Formaction, Formenctype, Formmethod, Formnovalidate, Formtarget, Height, List, Max, Maxlength, Min, Minlength, Multiple, Name, Pattern, Placeholder, Readonly, Required, Size, Src, Step, Type, Value, Width],
    link [As, Crossorigin, Href, Hreflang, Media, Rel, Sizes, Type],
    meta [Charset, Content, HttpEquiv, Name],
    source [Src, Type],
    track [Default, Kind, Label, Src, Srclang],
    wbr []
}

html_elements! {
    a [Download, Href, Hreflang, Ping, Rel, Target, Type],
    abbr [],
    address [],
    article [],
    aside [],
    audio [Autoplay, Controls, Crossorigin, Loop, Muted, Preload, Src],
    b [],
    bdi [],
    bdo [],
    blink [],
    blockquote [Cite],
    body [],
    button [Disabled, Form, Formaction, Formenctype, Formmethod, Formnovalidate, Formtarget, Name, Type, Value],
    canvas [Height, Width],
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
    dt [],
    em [],
    fieldset [],
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
    label [For, Form],
    legend [],
    li [Value],
    main [],
    map [Name],
    mark [],
    menu [],
    meter [Value, Min, Max, Low, High, Optimum, Form],
    nav [],
    noscript [],
    object [Data, Form, Height, Name, Type, Usemap, Width],
    ol [Reversed, Start, Type],
    optgroup [Disabled, Label],
    // option, // creates conflict with core Option
    output [For, Form, Name],
    p [],
    picture [],
    portal [Referrerpolicy, Src],
    pre [],
    progress [Max, Value],
    q [Cite],
    rp [],
    rt [],
    ruby [],
    s [],
    samp [],
    script [Async, Crossorigin, Defer, Fetchpriority, Integrity, Nomodule, Referrerpolicy, Src, Type, Blocking],
    search [],
    section [],
    select [Autocomplete, Disabled, Form, Multiple, Name, Required, Size],
    slot [Name],
    small [],
    span [],
    strong [],
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
    video [Controls, Controlslist, Crossorigin, Disablepictureinpicture, Disableremoteplayback, Height, Loop, Muted, Playsinline, Poster, Preload, Src, Width],
}

pub fn option<At, Ch, Rndr>(
    attributes: At,
    children: Ch,
) -> HtmlElement<Option_, At, Ch, Rndr>
where
    At: Attribute<Rndr> + OptionAttribute,
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

build_attributes! { OptionAttribute }
impl OptionAttribute for crate::html::attribute::Disabled {}
impl OptionAttribute for crate::html::attribute::Label {}
impl OptionAttribute for crate::html::attribute::Selected {}
impl OptionAttribute for crate::html::attribute::Value {}

#[cfg(test)]
mod tests {
    use super::{main, p, HtmlElement};
    use crate::{
        html::{
            attribute::{id, src},
            element::em,
        },
        renderer::mock_dom::MockDom,
        view::Render,
    };

    #[test]
    fn mock_dom_creates_element() {
        let el: HtmlElement<_, _, _, MockDom> =
            main((), p(id("test"), "Hello, world!"));
        let el = el.build();
        assert_eq!(
            el.el.to_debug_html(),
            "<main><p id=\"test\">Hello, world!</p></main>"
        );
    }

    #[test]
    fn mock_dom_creates_element_with_several_children() {
        let el: HtmlElement<_, _, _, MockDom> =
            main((), p((), ("Hello, ", em((), "beautiful"), " world!")));
        let el = el.build();
        assert_eq!(
            el.el.to_debug_html(),
            "<main><p>Hello, <em>beautiful</em> world!</p></main>"
        );
    }
}
