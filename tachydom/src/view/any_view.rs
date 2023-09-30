use super::{Mountable, PositionState, Render};
use crate::hydration::Cursor;
use std::any::{Any, TypeId};
use wasm_bindgen::JsCast;
use web_sys::{Element, Node};

pub struct AnyView {
    type_id: TypeId,
    value: Box<dyn Any>,
    to_html: fn(&mut dyn Any, &mut String, &PositionState),
    build: fn(Box<dyn Any>) -> AnyViewState,
    rebuild: fn(TypeId, Box<dyn Any>, &mut AnyViewState),
    hydrate_from_server:
        fn(Box<dyn Any>, &Cursor, &PositionState) -> AnyViewState,
    hydrate_from_template:
        fn(Box<dyn Any>, &Cursor, &PositionState) -> AnyViewState,
}

pub struct AnyViewState {
    type_id: TypeId,
    state: Box<dyn Any>,
    unmount: fn(&mut dyn Any),
    as_mountable: fn(&dyn Any) -> Option<Node>,
}

pub trait IntoAny {
    fn into_any(self) -> AnyView;
}

impl<T> IntoAny for T
where
    T: Render + 'static,
    T::State: 'static,
{
    fn into_any(self) -> AnyView {
        let value = Box::new(self) as Box<dyn Any>;

        let to_html = |value: &mut dyn Any,
                       buf: &mut String,
                       position: &PositionState| {
            let mut value = value
                .downcast_mut::<T>()
                .expect("AnyView::to_html could not be downcast");
            value.to_html(buf, position)
        };
        let build = |value: Box<dyn Any>| {
            let value = value
                .downcast::<T>()
                .expect("AnyView::build couldn't downcast");
            let state = Box::new(value.build());
            let unmount = |state: &mut dyn Any| {
                let state = state
                    .downcast_mut::<T::State>()
                    .expect("AnyViewState::unmount couldn't downcast state");
                state.unmount();
            };
            let as_mountable = |state: &dyn Any| {
                let state = state.downcast_ref::<T::State>().expect(
                    "AnyViewState::as_mountable couldn't downcast state",
                );
                state.as_mountable()
            };
            AnyViewState {
                type_id: TypeId::of::<T>(),
                state,
                unmount,
                as_mountable,
            }
        };
        let hydrate_from_server =
            |value: Box<dyn Any>,
             cursor: &Cursor<R>,
             position: &PositionState| {
                let value = value
                    .downcast::<T>()
                    .expect("AnyView::hydrate_from_server couldn't downcast");
                let state = Box::new(value.hydrate::<true>(cursor, position));
                let unmount = |state: &mut dyn Any| {
                    let state = state.downcast_mut::<T::State>().expect(
                        "AnyViewState::unmount couldn't downcast state",
                    );
                    state.unmount();
                };
                let as_mountable = |state: &dyn Any| {
                    let state = state.downcast_ref::<T::State>().expect(
                        "AnyViewState::as_mountable couldn't downcast state",
                    );
                    state.as_mountable()
                };
                AnyViewState {
                    type_id: TypeId::of::<T>(),
                    state,
                    unmount,
                    as_mountable,
                }
            };
        let hydrate_from_template =
            |value: Box<dyn Any>,
             cursor: &Cursor<R>,
             position: &PositionState| {
                let value = value
                    .downcast::<T>()
                    .expect("AnyView::hydrate_from_server couldn't downcast");
                let state = Box::new(value.hydrate::<true>(cursor, position));

                let unmount = |state: &mut dyn Any| {
                    let state = state.downcast_mut::<T::State>().expect(
                        "AnyViewState::unmount couldn't downcast state",
                    );
                    state.unmount();
                };
                let as_mountable = |state: &dyn Any| {
                    let state = state.downcast_ref::<T::State>().expect(
                        "AnyViewState::as_mountable couldn't downcast state",
                    );
                    state.as_mountable()
                };
                AnyViewState {
                    type_id: TypeId::of::<T>(),
                    state,
                    unmount,
                    as_mountable,
                }
            };
        let rebuild = |new_type_id: TypeId,
                       value: Box<dyn Any>,
                       state: &mut AnyViewState| {
            let value = value
                .downcast::<T>()
                .expect("AnyView::rebuild couldn't downcast value");
            if new_type_id == state.type_id {
                let state = state
                    .state
                    .downcast_mut()
                    .expect("AnyView::rebuild couldn't downcast state");
                value.rebuild(state);
            } else {
                // FIXME swapping types more generally
                // unmount previous view
                let prev_node = state.as_mountable();
                // build new view and mount it
                let new = value.into_any().build();
                match (prev_node, new.as_mountable()) {
                    (Some(prev), Some(new)) => {
                        prev.unchecked_ref::<Element>()
                            .replace_with_with_node_1(&new);
                    }
                    _ => {} // FIXME
                }
                *state = new;
            }
        };
        AnyView {
            type_id: TypeId::of::<T>(),
            value,
            to_html,
            build,
            rebuild,
            hydrate_from_server,
            hydrate_from_template,
        }
    }
}

impl Render for AnyView {
    type State = AnyViewState;

    fn to_html(&self, buf: &mut String, position: &PositionState) {
        (self.to_html)(&mut *self.value, buf, position)
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor<R>,
        position: &PositionState,
    ) -> Self::State {
        if FROM_SERVER {
            (self.hydrate_from_server)(self.value, cursor, position)
        } else {
            (self.hydrate_from_template)(self.value, cursor, position)
        }
    }

    fn build(self) -> Self::State {
        (self.build)(self.value)
    }

    fn rebuild(self, state: &mut Self::State) {
        (self.rebuild)(self.type_id, self.value, state)
    }
}

impl Mountable for AnyViewState {
    fn unmount(&mut self) {
        (self.unmount)(&mut self.state)
    }

    fn as_mountable(&self) -> Option<Node> {
        (self.as_mountable)(&*self.state)
    }
}

#[cfg(test)]
mod tests {
    use super::IntoAny;
    use crate::{
        html::element::{p, span},
        view::Render,
    };

    #[test]
    fn should_handle_html_creation() {
        let x = 1;
        let mut buf = String::new();
        let mut view = if x == 0 {
            p((), "foo").into_any()
        } else {
            span((), "bar").into_any()
        };
        view.to_html(&mut buf, &Default::default());
        assert_eq!(buf, "<span>bar</span>");
    }
}
