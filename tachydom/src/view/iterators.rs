use super::{Mountable, Position, PositionState, Render, RenderHtml};
use crate::{
    hydration::Cursor,
    renderer::{CastFrom, Renderer},
};

impl<T, R> Render<R> for Option<T>
where
    T: Render<R>,
    R: Renderer,
{
    type State = OptionState<T, R>;

    fn build(self) -> Self::State {
        let placeholder = R::create_placeholder();
        OptionState {
            placeholder,
            state: self.map(T::build),
        }
    }

    fn rebuild(self, state: &mut Self::State) {
        match (&mut state.state, self) {
            // both None: no need to do anything
            (None, None) => {}
            // both Some: need to rebuild child
            (Some(old), Some(new)) => {
                T::rebuild(new, old);
            }
            // Some => None: unmount replace with marker
            (Some(old), None) => {
                old.unmount();
                state.state = None;
            } // None => Some: build
            (None, Some(new)) => {
                let mut new_state = new.build();
                R::mount_before(&mut new_state, state.placeholder.as_ref());
                state.state = Some(new_state);
            }
        }
    }
}

impl<T, R> RenderHtml<R> for Option<T>
where
    T: RenderHtml<R>,
    R: Renderer,
    R::Node: Clone,
    R::Element: Clone,
{
    fn to_html(&self, buf: &mut String, position: &PositionState) {
        if let Some(value) = self {
            value.to_html(buf, position);
        }
        // placeholder
        buf.push_str("<!>");
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor<R>,
        position: &PositionState,
    ) -> Self::State {
        // hydrate the state, if it exists
        let state = self.map(|s| s.hydrate::<FROM_SERVER>(cursor, position));

        // pull the placeholder
        if position.get() == Position::FirstChild {
            cursor.child();
        } else {
            cursor.sibling();
        }
        let placeholder = cursor.current().to_owned();
        let placeholder = R::Placeholder::cast_from(placeholder).unwrap();
        position.set(Position::NextChild);

        OptionState { placeholder, state }
    }
}

/// View state for an optional view.
pub struct OptionState<T, R>
where
    T: Render<R>,
    R: Renderer,
{
    /// Marks the location of this view.
    placeholder: R::Placeholder,
    /// The view state.
    state: Option<T::State>,
}

impl<T, R> Mountable<R> for OptionState<T, R>
where
    T: Render<R>,
    R: Renderer,
{
    fn unmount(&mut self) {
        if let Some(ref mut state) = self.state {
            state.unmount();
        }
        R::remove(self.placeholder.as_ref());
    }

    fn mount(&mut self, parent: &R::Element, marker: Option<&R::Node>) {
        if let Some(ref mut state) = self.state {
            state.mount(parent, marker);
        }
        self.placeholder.mount(parent, marker);
    }
}

/*
impl<T, R> Render<R> for Vec<T>
where
    T: Render<R>,
    R: Renderer,
{
    type State = Vec<T::State>;

    fn build(self) -> Self::State {
        todo!()
    }

    fn rebuild(self, state: &mut Self::State) {
        todo!()
    }
}

impl<T, R> RenderHtml<R> for Vec<T>
where
    T: RenderHtml<R>,
    R: Renderer,
    R::Node: Clone,
{
    fn to_html(&self, buf: &mut String, position: &PositionState) {
        for item in self {
            item.to_html(buf, position);
        }
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor<R>,
        position: &PositionState,
    ) -> Self::State {
        todo!()
    }
}

impl<T, R> Mountable<R> for Vec<T>
where
    T: Mountable<R>,
    R: Renderer,
{
    fn unmount(&mut self) {
        todo!()
    }

    fn as_mountable(&self) -> Option<Node> {
        todo!()
    }
}

pub trait IterView<R: Renderer> {
    type Iterator: Iterator<Item = Self::View>;
    type View: Render<R>;

    fn iter_view(self) -> RenderIter<Self::Iterator, Self::View, R>;
}

impl<I, V, R> IterView<R> for I
where
    I: Iterator<Item = V>,
    V: Render<R>,
    R: Renderer,
{
    type Iterator = I;
    type View = V;

    fn iter_view(self) -> RenderIter<Self::Iterator, Self::View, R> {
        RenderIter {
            inner: self,
            rndr: PhantomData,
        }
    }
}

pub struct RenderIter<I, V, R>
where
    I: Iterator<Item = V>,
    V: Render<R>,
    R: Renderer,
{
    inner: I,
    rndr: PhantomData<R>,
}

impl<I, V, R> Render<R> for RenderIter<I, V, R>
where
    I: Iterator<Item = V>,
    V: Render<R>,
    R: Renderer,
{
    type State = ();

    fn build(self) -> Self::State {
        todo!()
    }

    fn rebuild(self, state: &mut Self::State) {
        todo!()
    }
}

impl<I, V, R> RenderHtml<R> for RenderIter<I, V, R>
where
    I: Iterator<Item = V>,
    V: RenderHtml<R>,
    R: Renderer,
    R::Node: Clone,
{
    fn to_html(&self, buf: &mut String, position: &PositionState) {
        for mut next in self.0.by_ref() {
            next.to_html(buf, position);
        }
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor<R>,
        position: &PositionState,
    ) -> Self::State {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::IterView;
    use crate::view::{Render, RenderHtml};

    #[test]
    fn iter_view_takes_iterator() {
        let strings = vec!["a", "b", "c"];
        let mut iter_view = strings
            .into_iter()
            .map(|n| n.to_ascii_uppercase())
            .iter_view();
        let mut buf = String::new();
        iter_view.to_html(&mut buf, &Default::default());
        assert_eq!(buf, "ABC");
    }
}
*/
