use crate::dom::Node;

pub mod html;
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
