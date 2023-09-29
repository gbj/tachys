use super::{Mountable, PositionState, Render, RenderHtml, ToTemplate};
use crate::{hydration::Cursor, renderer::Renderer};
use leptos_reactive::{create_render_effect, Effect};
use web_sys::Node;

impl<F, V> ToTemplate for F
where
    F: Fn() -> V,
    V: ToTemplate,
{
    fn to_template(buf: &mut String, position: &mut super::Position) {
        V::to_template(buf, position)
    }
}

impl<F, V, R> Render<R> for F
where
    F: Fn() -> V + 'static,
    V: Render<R>,
    V::State: 'static,
    R: Renderer,
{
    type State = Effect<V::State>;

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

impl<F, V, R> RenderHtml<R> for F
where
    F: Fn() -> V + 'static,
    V: RenderHtml<R>,
    V::State: 'static,
    R: Renderer,
    R::Node: Clone,
    R::Element: Clone,
{
    fn to_html(&mut self, buf: &mut String, position: &PositionState) {
        let mut value = self();
        value.to_html(buf, position)
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor<R>,
        position: &PositionState,
    ) -> Self::State {
        todo!()
        /* let cursor = cursor.clone();
        let position = position.clone();
        create_render_effect(move |prev| {
            let value = self();
            if let Some(mut state) = prev {
                value.rebuild(&mut state);
                state
            } else {
                value.hydrate::<FROM_SERVER>(&cursor, &position)
            }
        }) */
    }
}

impl<M, R> Mountable<R> for Effect<M>
where
    M: Mountable<R> + 'static,
    R: Renderer,
{
    fn unmount(&mut self) {
        self.with_value_mut(|value| {
            if let Some(value) = value {
                value.unmount()
            }
        });
    }

    fn as_mountable(&self) -> Option<R::Node> {
        self.with_value_mut(|value| {
            value.as_ref().and_then(|n| n.as_mountable())
        })
        .flatten()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        html::element::{button, em, main, p, HtmlElement},
        renderer::mock_dom::MockDom,
        view::Render,
    };
    use leptos_reactive::{create_runtime, RwSignal, SignalGet, SignalSet};

    #[test]
    fn create_dynamic_element() {
        let rt = create_runtime();
        let count = RwSignal::new(0);
        let app: HtmlElement<_, _, _, MockDom> =
            button((), move || count.get().to_string());
        let el = app.build();
        assert_eq!(el.el.to_debug_html(), "<button>0</button>");
        rt.dispose();
    }

    #[test]
    fn update_dynamic_element() {
        let rt = create_runtime();
        let count = RwSignal::new(0);
        let app: HtmlElement<_, _, _, MockDom> =
            button((), move || count.get().to_string());
        let el = app.build();
        assert_eq!(el.el.to_debug_html(), "<button>0</button>");
        count.set(1);
        assert_eq!(el.el.to_debug_html(), "<button>1</button>");
        rt.dispose();
    }
}
