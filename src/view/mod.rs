use itertools::Itertools;

use crate::dom::Node;

pub mod html;
pub mod keyed;
mod strings;
mod tuples;

pub trait View {
    type State;

    fn build(self) -> Self::State;

    fn rebuild(self, state: &mut Self::State);

    fn mount(state: &mut Self::State, parent: Node);

    fn unmount(state: &mut Self::State);
}

pub struct Static<T>(pub T);

impl<T> View for Static<T>
where
    T: View,
{
    type State = <T as View>::State;

    #[inline(always)]
    fn build(self) -> Self::State {
        T::build(self.0)
    }

    #[inline(always)]
    fn rebuild(self, state: &mut Self::State) {}

    #[inline(always)]
    fn mount(state: &mut Self::State, parent: Node) {
        T::mount(state, parent)
    }

    #[inline(always)]
    fn unmount(state: &mut Self::State) {
        T::unmount(state)
    }
}

impl<const N: usize, T> View for [T; N]
where
    T: View,
{
    type State = [<T as View>::State; N];

    #[inline(always)]
    fn build(self) -> Self::State {
        self.map(T::build)
    }

    #[inline(always)]
    fn rebuild(self, state: &mut Self::State) {
        for (item, state) in self.into_iter().zip(state.iter_mut()) {
            T::rebuild(item, state);
        }
    }

    #[inline(always)]
    fn mount(state: &mut Self::State, parent: Node) {
        for item in state.iter_mut() {
            T::mount(item, parent)
        }
    }

    #[inline(always)]
    fn unmount(state: &mut Self::State) {
        for item in state.iter_mut() {
            T::unmount(item)
        }
    }
}

impl<T> View for Vec<T>
where
    T: View,
{
    type State = (Vec<<T as View>::State>, Option<Node>);

    #[inline(always)]
    fn build(self) -> Self::State {
        (self.into_iter().map(T::build).collect(), None)
    }

    #[inline(always)]
    fn rebuild(self, state: &mut Self::State) {
        let (old, parent) = state;
        let parent = parent.unwrap();
        // this is an unkeyed diff
        if old.is_empty() {
            let mut new = self.build().0;
            for item in new.iter_mut() {
                T::mount(item, parent);
            }
            *old = new;
        } else if self.is_empty() {
            parent.set_text("");
            old.clear();
        } else {
            for item in self.into_iter().zip_longest(old.iter_mut()) {
                match item {
                    itertools::EitherOrBoth::Both(new, old) => T::rebuild(new, old),
                    itertools::EitherOrBoth::Left(new) => T::mount(&mut new.build(), parent),
                    itertools::EitherOrBoth::Right(old) => T::unmount(old),
                }
            }
        }
    }

    #[inline(always)]
    fn mount(state: &mut Self::State, parent: Node) {
        state.1 = Some(parent);
        for item in state.0.iter_mut() {
            T::mount(item, parent)
        }
    }

    #[inline(always)]
    fn unmount(state: &mut Self::State) {
        for item in state.0.iter_mut() {
            T::unmount(item)
        }
    }
}
