use std::{cell::RefCell, rc::Rc};

use crate::{
    dom::{Dom, Node},
    view::View,
};

pub struct Component<S, F, V>
where
    S: State,
    F: Fn(&S, ComponentLink<S>) -> V,
    V: View,
{
    model: S,
    view_fn: F,
}

impl<S, F, V> Component<S, F, V>
where
    S: State,
    F: Fn(&S, ComponentLink<S>) -> V,
    V: View,
{
    pub fn new(initial: S, view_fn: F) -> Self {
        Self {
            model: initial,
            view_fn,
        }
    }
}

pub struct ComponentLink<C: State> {
    updater: Rc<RefCell<Option<Box<dyn FnMut(C::Msg)>>>>,
}

impl<C: State> Clone for ComponentLink<C> {
    fn clone(&self) -> Self {
        Self {
            updater: Rc::clone(&self.updater),
        }
    }
}

impl<C: State> ComponentLink<C> {
    pub fn send(&self, msg: C::Msg) {
        let mut updater = self.updater.borrow_mut();
        let mut updater = updater.as_mut().unwrap();
        updater(msg);
    }
}

impl<S, F, V> View for Component<S, F, V>
where
    S: State,
    F: Fn(&S, ComponentLink<S>) -> V,
    V: View,
{
    type State = (ComponentLink<S>, Rc<RefCell<V::State>>);

    fn build(self) -> Self::State {
        let Component { mut model, view_fn } = self;
        let link = ComponentLink {
            updater: Default::default(),
        };
        let view = view_fn(&model, link.clone());
        let view_state = Rc::new(RefCell::new(view.build()));
        let updater = Box::new({
            let view_state = Rc::clone(&view_state);
            let link = link.clone();
            move |msg: S::Msg| {
                model.update(msg);
                let view = view_fn(&model, link.clone());
                let mut view_state = (*view_state).borrow_mut();
                V::rebuild(view, &mut view_state);
                Dom::flush();
            }
        });
        *link.updater.borrow_mut() = Some(updater);
        (link, view_state)
    }

    fn rebuild(self, state: &mut Self::State) {
        todo!()
    }

    fn mount(state: &mut Self::State, parent: Node) {
        let (_, view_state) = state;
        let mut view_state = (**view_state).borrow_mut();
        V::mount(&mut view_state, parent)
    }

    fn unmount(state: &mut Self::State) {
        let (_, view_state) = state;
        let mut view_state = (**view_state).borrow_mut();
        V::unmount(&mut view_state)
    }
}

pub trait State: Sized {
    type Msg;

    fn update(&mut self, msg: Self::Msg);
}
