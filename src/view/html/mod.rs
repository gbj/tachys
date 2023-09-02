use crate::dom::{Dom, El, Node};

use self::attribute::Attribute;
use super::{Mount, View};

pub mod attribute;

pub struct Html<A, C>
where
    A: Attribute,
    C: View,
{
    pub tag: El,
    pub attributes: A,
    pub children: C,
}

impl<A, C> View for Html<A, C>
where
    A: Attribute,
    C: View,
{
    type State = (Node, A::State, C::State);

    fn build(self) -> Self::State {
        let el = Dom::create_element(self.tag);
        let attrs = self.attributes.build(el);
        let mut children = self.children.build();
        C::mount(&mut children, Mount::Append { parent: el });
        (el, attrs, children)
    }

    fn rebuild(self, state: &mut Self::State) {
        let (el, attrs, children) = state;
        A::rebuild(self.attributes, attrs);
        C::rebuild(self.children, children);
    }

    fn mount(state: &mut Self::State, parent: Mount) {
        let (el, _, _) = state;
        parent.mount_node(*el);
    }

    fn unmount(state: &mut Self::State) {
        let (el, _, _) = state;
        el.remove();
    }
}
