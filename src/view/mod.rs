use std::{cell::RefCell, rc::Rc};

use itertools::Itertools;
use leptos_reactive::{create_render_effect, Effect};

use crate::dom::{Dom, Node};

pub mod html;
pub mod keyed;
mod strings;
mod tuples;

pub trait View {
    type State;

    fn build(self) -> Self::State;

    fn rebuild(self, state: &mut Self::State);

    fn mount(state: &mut Self::State, kind: Mount);

    fn unmount(state: &mut Self::State);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Mount {
    Append { parent: Node },
    Before { parent: Node, before: Node },
    OnlyChild { parent: Node },
}

impl Mount {
    pub fn mount_node(&self, node: Node) {
        match self {
            Mount::Append { parent } | Mount::OnlyChild { parent } => parent.append_child(node),
            Mount::Before { parent, before } => Dom::insert_before(*parent, node, *before),
        }
    }
}

impl<F: Fn() -> V + 'static, V: View + 'static> View for F {
    type State = Effect<V::State>;

    ////#[inline(always)]
    fn build(self) -> Self::State {
        create_render_effect({
            move |prev| {
                let value = self();
                if let Some(mut state) = prev {
                    value.rebuild(&mut state);
                    state
                } else {
                    value.build()
                }
            }
        })
    }

    ////#[inline(always)]
    fn rebuild(self, state: &mut Self::State) {
        todo!()
    }

    ////#[inline(always)]
    fn mount(state: &mut Self::State, kind: Mount) {
        state.with_value_mut(|prev| {
            // this should always be Some because effects run immediately
            if let Some(prev) = prev {
                V::mount(prev, kind);
            }
        });
    }

    ////#[inline(always)]
    fn unmount(state: &mut Self::State) {
        state.with_value_mut(|prev| {
            // this should always be Some because effects run immediately
            if let Some(prev) = prev {
                V::unmount(prev);
            }
        });
    }
}

/*
impl<const N: usize, T> View for [T; N]
where
    T: View,
{
    type State = [<T as View>::State; N];

    ////#[inline(always)]
    fn build(self) -> Self::State {
        self.map(T::build)
    }

    ////#[inline(always)]
    fn rebuild(self, state: &mut Self::State) {
        for (item, state) in self.into_iter().zip(state.iter_mut()) {
            T::rebuild(item, state);
        }
    }

    ////#[inline(always)]
    fn mount(state: &mut Self::State, parent: Node) {
        for item in state.iter_mut() {
            T::mount(item, parent)
        }
    }

    ////#[inline(always)]
    fn unmount(state: &mut Self::State) {
        for item in state.iter_mut() {
            T::unmount(item)
        }
    }
}
 */
