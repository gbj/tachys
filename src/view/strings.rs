use super::View;
use crate::dom::{Dom, Node};

impl<'a> View for &'a str {
    type State = (Self, Node);

    #[inline(always)]
    fn build(self) -> Self::State {
        let text = Dom::create_text_node(self);
        (self, text)
    }

    #[inline(always)]
    fn rebuild(self, state: &mut Self::State) {
        let (prev, node) = state;
        if &self != prev {
            node.set_data(self);
            state.0 = self;
        }
    }

    #[inline(always)]
    fn mount(state: &mut Self::State, parent: Node) {
        parent.append_child(&state.1);
    }

    #[inline(always)]
    fn unmount(state: &mut Self::State) {
        state.1.remove();
    }
}

impl View for String {
    type State = (Self, Node);

    #[inline(always)]
    fn build(self) -> Self::State {
        let text = Dom::create_text_node(&self);
        (self, text)
    }

    #[inline(always)]
    fn rebuild(self, state: &mut Self::State) {
        let (prev, node) = state;
        if &self != prev {
            node.set_data(&self);
            state.0 = self;
        }
    }

    #[inline(always)]
    fn mount(state: &mut Self::State, parent: Node) {
        parent.append_child(&state.1);
    }

    #[inline(always)]
    fn unmount(state: &mut Self::State) {
        state.1.remove();
    }
}
