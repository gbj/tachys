use leptos_reactive::{create_render_effect, Effect};
use web_sys::Node;

use crate::hydration::Cursor;

use super::{Mountable, PositionState, ToTemplate, View};

impl<F, V> ToTemplate for F
where
    F: Fn() -> V,
    V: ToTemplate,
{
    fn to_template(buf: &mut String, position: &mut super::Position) {
        V::to_template(buf, position)
    }
}

impl<F, V> View for F
where
    F: Fn() -> V + 'static,
    V: View,
    V::State: 'static,
{
    type State = Effect<V::State>;

    fn to_html(&self, buf: &mut String, position: &PositionState) {
        let value = self();
        value.to_html(buf, position)
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        let cursor = cursor.clone();
        let position = position.clone();
        create_render_effect(move |prev| {
            let value = self();
            if let Some(mut state) = prev {
                value.rebuild(&mut state);
                state
            } else {
                web_sys::console::log_3(
                    &wasm_bindgen::JsValue::from_str("dynamic hydration starting at "),
                    &cursor.current(),
                    &wasm_bindgen::JsValue::from_str(&format!("and position {:?}", position.get())),
                );
                value.hydrate::<FROM_SERVER>(&cursor, &position)
            }
        })
    }

    fn build(self) -> Self::State {
        create_render_effect(move |prev| {
            let value = self();
            if let Some(mut state) = prev {
                value.rebuild(&mut state);
                state
            } else {
                value.build()
            }
        })
    }

    fn rebuild(self, state: &mut Self::State) {
        todo!()
    }
}

impl<M: Mountable + 'static> Mountable for Effect<M> {
    fn unmount(&mut self) {
        self.with_value_mut(|value| {
            if let Some(value) = value {
                value.unmount()
            }
        });
    }

    fn as_mountable(&self) -> Option<Node> {
        self.with_value_mut(|value| value.as_ref().and_then(|n| n.as_mountable()))
            .flatten()
    }
}
