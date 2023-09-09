use leptos_reactive::Effect;
use wasm_bindgen::JsCast;

use crate::hydration::Cursor;

use super::{Position, ToTemplate, View};

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

    fn to_html(&self, buf: &mut String, position: &mut Position) {
        let value = self();
        value.to_html(buf, position)
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &mut Cursor,
        position: &mut Position,
    ) -> Self::State {
        let cursor = cursor.current().to_owned();
        let pos = *position;
        Effect::new(move |prev| {
            let value = self();
            if let Some(mut state) = prev {
                value.rebuild(&mut state);
                state
            } else {
                let mut cursor = Cursor::new(cursor.clone().unchecked_into());
                let mut position = pos;
                value.hydrate::<FROM_SERVER>(&mut cursor, &mut position)
            }
        })
    }

    fn rebuild(self, state: &mut Self::State) {
        todo!()
    }
}
