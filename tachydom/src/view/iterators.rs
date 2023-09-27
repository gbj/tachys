use web_sys::Node;

use super::{Mountable, Position, PositionState, Render};
use crate::{dom::comment, hydration::Cursor};
use std::fmt::Debug;
use wasm_bindgen::JsCast;
use web_sys::Element;

impl<T> Render for Option<T>
where
    T: Render,
{
    type State = OptionState<T>;

    fn to_html(&mut self, buf: &mut String, position: &PositionState) {
        match self {
            // pass Some(_) through directly
            Some(value) => value.to_html(buf, position),
            // otherwise render a marker that can be picked up during hydration
            None => buf.push_str("<!--Option-->"),
        }
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        match self {
            // if None, pull the text node and store it
            None => {
                if position.get() == Position::FirstChild {
                    cursor.child();
                } else {
                    cursor.sibling();
                }
                let node = cursor.current().to_owned();
                position.set(Position::NextChild);
                OptionState::None(node)
            }
            // if Some(_), just hydrate the child
            Some(value) => {
                let state = value.hydrate::<FROM_SERVER>(cursor, position);
                position.set(Position::NextChild);
                OptionState::Some(state)
            }
        }
    }

    fn build(self) -> Self::State {
        match self {
            // if None, pull the text node and store it
            None => OptionState::None(comment()),
            // if Some(_), just hydrate the child
            Some(value) => {
                let state = value.build();
                OptionState::Some(state)
            }
        }
    }

    fn rebuild(self, state: &mut Self::State) {
        match (&mut *state, self) {
            // both None: no need to do anything
            (OptionState::None(_), None) => {}
            // both Some: need to rebuild child
            (OptionState::Some(old), Some(new)) => {
                T::rebuild(new, old);
            }
            // Some => None: unmount replace with marker
            (OptionState::Some(old), None) => {
                let new_marker = comment();
                if let Some(marker) = old.as_mountable() {
                    marker
                        .unchecked_ref::<Element>()
                        .replace_with_with_node_1(&new_marker);
                    old.unmount();
                }
                *state = OptionState::None(new_marker);
            } // None => Some: build
            (OptionState::None(marker), Some(new)) => {
                let new_state = new.build();
                let mountable = new_state.as_mountable();
                if let Some(mountable) = mountable {
                    marker
                        .unchecked_ref::<Element>()
                        .replace_with_with_node_1(&mountable);
                }
                *state = OptionState::Some(new_state);
            }
        }
    }
}

/// View state for an optional view.
pub enum OptionState<T>
where
    T: Render,
{
    /// Contains a marker node that will be replaced when the
    /// state switches to `Some(T)`.
    None(Node),
    /// The view state.
    Some(T::State),
}

impl<T> Mountable for OptionState<T>
where
    T: Render,
{
    fn unmount(&mut self) {
        match self {
            OptionState::None(node) => {
                node.parent_node().unwrap().remove_child(node).unwrap();
            }
            OptionState::Some(state) => state.unmount(),
        }
    }

    fn as_mountable(&self) -> Option<Node> {
        match self {
            OptionState::None(node) => Some(node.clone()),
            OptionState::Some(state) => state.as_mountable(),
        }
    }
}

impl<T: Render> Debug for OptionState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None(arg0) => f.debug_tuple("None").field(arg0).finish(),
            Self::Some(_) => f.debug_tuple("Some").finish(),
        }
    }
}

impl<T> Render for Vec<T>
where
    T: Render,
{
    type State = Vec<T::State>;

    fn to_html(&mut self, buf: &mut String, position: &PositionState) {
        for item in self {
            item.to_html(buf, position);
        }
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        todo!()
    }

    fn build(self) -> Self::State {
        todo!()
    }

    fn rebuild(self, state: &mut Self::State) {
        todo!()
    }
}

impl<T> Mountable for Vec<T>
where
    T: Mountable,
{
    fn unmount(&mut self) {
        todo!()
    }

    fn as_mountable(&self) -> Option<Node> {
        todo!()
    }
}

pub trait IterView {
    type Iterator: Iterator<Item = Self::View>;
    type View: Render;

    fn iter_view(self) -> RenderIter<Self::Iterator, Self::View>;
}

impl<I, V> IterView for I
where
    I: Iterator<Item = V>,
    V: Render,
{
    type Iterator = I;
    type View = V;

    fn iter_view(self) -> RenderIter<Self::Iterator, Self::View> {
        RenderIter(self)
    }
}

pub struct RenderIter<I, V>(I)
where
    I: Iterator<Item = V>,
    V: Render;

impl<I, V> Render for RenderIter<I, V>
where
    I: Iterator<Item = V>,
    V: Render,
{
    type State = ();

    fn to_html(&mut self, buf: &mut String, position: &PositionState) {
        for mut next in self.0.by_ref() {
            next.to_html(buf, position);
        }
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        todo!()
    }

    fn build(self) -> Self::State {
        todo!()
    }

    fn rebuild(self, state: &mut Self::State) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::IterView;
    use crate::view::Render;

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
