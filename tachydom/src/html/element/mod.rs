use crate::{
    html::attribute::Attribute,
    hydration::Cursor,
    renderer::{CastFrom, Renderer},
    view::{
        template::{const_concat, const_concat_with_prefix, str_from_buffer},
        Mountable, Position, PositionState, Render, RenderHtml, ToTemplate,
    },
};
use next_tuple::TupleBuilder;
use std::marker::PhantomData;

mod elements;
use super::attribute::{global::AddAttribute, id, Attr, AttributeValue, Id};
pub use elements::*;

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

impl<E, At, Ch, NewChild, Rndr> ElementChild<NewChild>
    for HtmlElement<E, At, Ch, Rndr>
where
    E: ElementType + ElementWithChildren,
    At: Attribute<Rndr>,
    Ch: Render<Rndr> + TupleBuilder<NewChild>,
    <Ch as TupleBuilder<NewChild>>::Output: Render<Rndr>,
    Rndr: Renderer,
{
    type Output =
        HtmlElement<E, At, <Ch as TupleBuilder<NewChild>>::Output, Rndr>;

    fn child(self, child: NewChild) -> Self::Output {
        let HtmlElement {
            ty,
            rndr,
            attributes,
            children,
        } = self;
        HtmlElement {
            ty,
            rndr,
            attributes,
            children: children.next_tuple(child),
        }
    }
}

impl<E, At, Ch, Rndr, NewAttr> AddAttribute<NewAttr, Rndr>
    for HtmlElement<E, At, Ch, Rndr>
where
    E: ElementType,
    At: Attribute<Rndr> + TupleBuilder<NewAttr>,
    <At as TupleBuilder<NewAttr>>::Output: Attribute<Rndr>,
    Ch: Render<Rndr>,
    Rndr: Renderer,
{
    type Output =
        HtmlElement<E, <At as TupleBuilder<NewAttr>>::Output, Ch, Rndr>;

    fn add_attr(self, attr: NewAttr) -> Self::Output {
        let HtmlElement {
            ty,
            attributes,
            children,
            rndr,
        } = self;
        HtmlElement {
            ty,
            attributes: attributes.next_tuple(attr),
            children,
            rndr,
        }
    }
}

impl<E, At, Ch, Rndr> HtmlElement<E, At, Ch, Rndr>
where
    At: Attribute<Rndr>,
    Ch: Render<Rndr>,
    Rndr: Renderer,
{
    /// Adds any attribute to the element. This is a compile-time operation, and modifies
    /// the type of the view tree.
    /// ```rust
    /// use tachydom::{
    ///     html::{
    ///         attribute::id,
    ///         class::class,
    ///         element::{p, HtmlElement},
    ///     },
    ///     renderer::mock_dom::MockDom,
    ///     view::Render,
    /// };
    /// let el: HtmlElement<_, _, _, MockDom> =
    ///     p().attr(id("foo")).attr(class("bar"));
    /// let el = el.build();
    /// assert_eq!(el.el.to_debug_html(), "<p id=\"foo\" class=\"bar\"></p>");
    /// ```
    #[inline(always)]
    pub fn attr<NewAttr>(
        self,
        attr: NewAttr,
    ) -> HtmlElement<E, <At as TupleBuilder<NewAttr>>::Output, Ch, Rndr>
    where
        E: ElementType,
        At: TupleBuilder<NewAttr>,
        At: Attribute<Rndr> + TupleBuilder<NewAttr>,
        <At as TupleBuilder<NewAttr>>::Output: Attribute<Rndr>,
    {
        let HtmlElement {
            ty,
            rndr,
            attributes,
            children,
        } = self;
        HtmlElement {
            ty,
            rndr,
            children,
            attributes: attributes.next_tuple(attr),
        }
    }
}

pub trait ElementChild<NewChild> {
    type Output;

    fn child(self, child: NewChild) -> Self::Output;
}

pub trait ElementType {
    const TAG: &'static str;
    const SELF_CLOSING: bool;
}

pub trait ElementWithChildren {}

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
    const TEMPLATE: &'static str = str_from_buffer(&const_concat(&[
        "<",
        E::TAG,
        At::TEMPLATE,
        str_from_buffer(&const_concat_with_prefix(
            &[At::CLASS],
            " class=\"",
            "\"",
        )),
        str_from_buffer(&const_concat_with_prefix(
            &[At::STYLE],
            " style=\"",
            "\"",
        )),
        ">",
        Ch::TEMPLATE,
        "</",
        E::TAG,
        ">",
    ]));

    fn to_template(
        buf: &mut String,
        class: &mut String,
        style: &mut String,
        position: &mut Position,
    ) {
        // opening tag and attributes
        let mut class = String::new();
        let mut style = String::new();

        buf.push('<');
        buf.push_str(E::TAG);
        <At as ToTemplate>::to_template(buf, &mut class, &mut style, position);

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

        // children
        *position = Position::FirstChild;
        class.clear();
        style.clear();
        Ch::to_template(buf, &mut class, &mut style, position);

        // closing tag
        buf.push_str("</");
        buf.push_str(E::TAG);
        buf.push('>');
        *position = Position::NextChild;
    }
}

#[cfg(test)]
mod tests {
    use super::{main, p, HtmlElement};
    use crate::{
        html::{
            attribute::{global::GlobalAttributes, id, src},
            class::class,
            element::{em, ElementChild, Main},
        },
        renderer::mock_dom::MockDom,
        view::Render,
    };

    #[test]
    fn mock_dom_creates_element() {
        let el: HtmlElement<Main, _, _, MockDom> =
            main().child(p().id("test").lang("en").child("Hello, world!"));
        let el = el.build();
        assert_eq!(
            el.el.to_debug_html(),
            "<main><p id=\"test\" lang=\"en\">Hello, world!</p></main>"
        );
    }

    #[test]
    fn mock_dom_creates_element_with_several_children() {
        let el: HtmlElement<Main, _, _, MockDom> = main().child(p().child((
            "Hello, ",
            em().child("beautiful"),
            " world!",
        )));
        let el = el.build();
        assert_eq!(
            el.el.to_debug_html(),
            "<main><p>Hello, <em>beautiful</em> world!</p></main>"
        );
    }
}
