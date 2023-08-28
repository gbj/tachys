use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{
    dom::{Dom, Node},
    view::View,
};

pub struct ComponentLink<C: Component> {
    updater: Rc<RefCell<Option<Box<dyn FnMut(C::Msg)>>>>,
}

impl<C: Component> Clone for ComponentLink<C> {
    fn clone(&self) -> Self {
        Self {
            updater: Rc::clone(&self.updater),
        }
    }
}

impl<C: Component> ComponentLink<C> {
    pub fn send(&self, msg: C::Msg) {
        let mut updater = self.updater.borrow_mut();
        let mut updater = updater.as_mut().unwrap();
        updater(msg);
    }
}

impl<C: Component + 'static> View for C {
    type State = (ComponentLink<C>, Rc<RefCell<<C::View as View>::State>>);

    fn build(self) -> Self::State {
        let link = ComponentLink {
            updater: Default::default(),
        };
        let view = self.view(&link);
        let view_state = Rc::new(RefCell::new(view.build()));
        let mut model = self;
        let updater = Box::new({
            let view_state = Rc::clone(&view_state);
            let link = link.clone();
            move |msg: C::Msg| {
                model.update(msg);
                let view = model.view(&link);
                let mut view_state = (*view_state).borrow_mut();
                <C::View as View>::rebuild(view, &mut view_state);
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
        <C::View as View>::mount(&mut view_state, parent)
    }

    fn unmount(state: &mut Self::State) {
        let (_, view_state) = state;
        let mut view_state = (**view_state).borrow_mut();
        <C::View as View>::unmount(&mut view_state)
    }
}

pub trait Component: Sized {
    type Msg;
    type View: View;

    fn update(&mut self, msg: Self::Msg);

    fn view(&self, link: &ComponentLink<Self>) -> Self::View;
}
