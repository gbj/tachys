use super::{
    Mountable, Position, PositionState, Render, RenderHtml, ToTemplate,
};
use crate::{
    hydration::Cursor,
    renderer::{CastFrom, Renderer},
};

impl<'a, R: Renderer> Render<R> for &'a str {
    type State = StrState<'a, R>;

    fn build(self) -> Self::State {
        let node = R::create_text_node(self);
        StrState { node, str: self }
    }

    fn rebuild(self, state: &mut Self::State) {
        let StrState { node, str } = state;
        if &self != str {
            R::set_text(node, self);
            *str = self;
        }
    }
}

impl<'a, R> RenderHtml<R> for &'a str
where
    R: Renderer,
    R::Node: Clone,
    R::Element: Clone,
{
    const MIN_LENGTH: usize = 0;

    fn to_html_with_buf(self, buf: &mut String, position: &PositionState) {
        // add a comment node to separate from previous sibling, if any
        if matches!(position.get(), Position::NextChild | Position::LastChild) {
            buf.push_str("<!>")
        }
        buf.push_str(self);
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor<R>,
        position: &PositionState,
    ) -> Self::State {
        if position.get() == Position::FirstChild {
            cursor.child();
        } else {
            cursor.sibling();
        }

        let node = cursor.current();
        let node = R::Text::cast_from(node)
            .expect("couldn't cast text node from node");

        if matches!(position.get(), Position::NextChild | Position::LastChild) {
            cursor.sibling();
        }
        if !FROM_SERVER {
            R::set_text(&node, self);
        }
        position.set(Position::NextChild);

        StrState { node, str: self }
    }
}

impl<'a> ToTemplate for &'a str {
    const TEMPLATE: &'static str = " <!>";

    fn to_template(
        buf: &mut String,
        class: &mut String,
        style: &mut String,
        position: &mut Position,
    ) {
        buf.push(' ');
        if matches!(*position, Position::NextChild | Position::LastChild) {
            buf.push_str("<!>")
        }
        *position = Position::NextChild;
    }
}

impl<R: Renderer> Render<R> for String {
    type State = StringState<R>;

    fn build(self) -> Self::State {
        let node = R::create_text_node(&self);
        StringState { node, str: self }
    }

    fn rebuild(self, state: &mut Self::State) {
        let StringState { node, str } = state;
        if &self != str {
            R::set_text(node, &self);
            *str = self;
        }
    }
}

impl<R> RenderHtml<R> for String
where
    R: Renderer,
    R::Node: Clone,
    R::Element: Clone,
{
    const MIN_LENGTH: usize = 0;

    fn to_html_with_buf(self, buf: &mut String, position: &PositionState) {
        <&str as RenderHtml<R>>::to_html_with_buf(&self.as_str(), buf, position)
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor<R>,
        position: &PositionState,
    ) -> Self::State {
        let StrState { node, .. } =
            self.as_str().hydrate::<FROM_SERVER>(cursor, position);
        StringState { node, str: self }
    }
}

impl ToTemplate for String {
    const TEMPLATE: &'static str = <&str as ToTemplate>::TEMPLATE;

    fn to_template(
        buf: &mut String,
        class: &mut String,
        style: &mut String,
        position: &mut Position,
    ) {
        <&str as ToTemplate>::to_template(buf, class, style, position)
    }
}

pub struct StringState<R: Renderer> {
    node: R::Text,
    str: String,
}

pub struct StrState<'a, R: Renderer> {
    pub node: R::Text,
    str: &'a str,
}

impl<R: Renderer> Mountable<R> for StringState<R> {
    fn unmount(&mut self) {
        self.node.unmount()
    }

    fn mount(
        &mut self,
        parent: &<R as Renderer>::Element,
        marker: Option<&<R as Renderer>::Node>,
    ) {
        R::insert_node(parent, self.node.as_ref(), marker);
    }

    fn insert_before_this(
        &self,
        parent: &<R as Renderer>::Element,
        child: &mut dyn Mountable<R>,
    ) -> bool {
        child.mount(parent, Some(self.node.as_ref()));
        true
    }
}

impl<'a, R> Mountable<R> for StrState<'a, R>
where
    R: Renderer,
{
    fn unmount(&mut self) {
        self.node.unmount()
    }

    fn mount(
        &mut self,
        parent: &<R as Renderer>::Element,
        marker: Option<&<R as Renderer>::Node>,
    ) {
        R::insert_node(parent, self.node.as_ref(), marker);
    }

    fn insert_before_this(
        &self,
        parent: &<R as Renderer>::Element,
        child: &mut dyn Mountable<R>,
    ) -> bool {
        child.mount(parent, Some(self.node.as_ref()));
        true
    }
}
