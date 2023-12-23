use crate::{
    async_views::Suspend,
    html::{attribute::AttributeValue, property::IntoProperty},
    hydration::Cursor,
    renderer::{DomRenderer, Renderer},
    ssr::StreamBuilder,
    view::{
        FallibleRender, InfallibleRender, Mountable, Position, PositionState,
        Render, RenderHtml, ToTemplate,
    },
};
use tachy_reaccy::{async_signal::ScopedFuture, render_effect::RenderEffect};

mod class;
pub mod node_ref;
mod style;

impl<F, V> ToTemplate for F
where
    F: Fn() -> V,
    V: ToTemplate,
{
    const TEMPLATE: &'static str = V::TEMPLATE;

    fn to_template(
        buf: &mut String,
        class: &mut String,
        style: &mut String,
        inner_html: &mut String,
        position: &mut Position,
    ) {
        // FIXME this seems wrong
        V::to_template(buf, class, style, inner_html, position)
    }
}

impl<F, V, R> Render<R> for F
where
    F: Fn() -> V + 'static,
    V: Render<R>,
    V::State: 'static,
    R: Renderer,
{
    type State = RenderEffect<V::State>;

    #[track_caller]
    fn build(self) -> Self::State {
        RenderEffect::new(move |prev| {
            let value = self();
            if let Some(mut state) = prev {
                value.rebuild(&mut state);
                state
            } else {
                value.build()
            }
        })
    }

    #[track_caller]
    fn rebuild(self, state: &mut Self::State) {
        state.with_value_mut_and_as_owner(|state| {
            let value = self();
            value.rebuild(state);
        });
    }
}

impl<F, V> InfallibleRender for F where F: Fn() -> V + 'static {}

/* impl<F, V, R> FallibleRender<R> for F
where
    F: Fn() -> V + 'static,
    V: FallibleRender<R>,
    V::State: 'static,
    R: Renderer,
{
    type FallibleState = V::FallibleState;
    type Error = V::Error;

    fn try_build(self) -> Result<Self::FallibleState, Self::Error> {
        todo!()
    }

    fn try_rebuild(
        self,
        state: &mut Self::FallibleState,
    ) -> Result<(), Self::Error> {
        todo!()
    }
} */

impl<F, V, R> RenderHtml<R> for F
where
    F: Fn() -> V + 'static,
    V: RenderHtml<R>,
    V::State: 'static,
    R: Renderer + 'static,
    R::Node: Clone,
    R::Element: Clone,
{
    const MIN_LENGTH: usize = 0;

    fn to_html_with_buf(self, buf: &mut String, position: &mut Position) {
        let value = self();
        value.to_html_with_buf(buf, position)
    }

    fn to_html_async_with_buf<const OUT_OF_ORDER: bool>(
        self,
        buf: &mut StreamBuilder,
        position: &mut Position,
    ) where
        Self: Sized,
    {
        let value = self();
        value.to_html_async_with_buf::<OUT_OF_ORDER>(buf, position);
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor<R>,
        position: &PositionState,
    ) -> Self::State {
        let cursor = cursor.clone();
        let position = position.clone();
        RenderEffect::new(move |prev| {
            let value = self();
            if let Some(mut state) = prev {
                value.rebuild(&mut state);
                state
            } else {
                value.hydrate::<FROM_SERVER>(&cursor, &position)
            }
        })
    }
}

impl<M, R> Mountable<R> for RenderEffect<M>
where
    M: Mountable<R> + 'static,
    R: Renderer,
{
    fn unmount(&mut self) {
        self.with_value_mut(|state| state.unmount());
    }

    fn mount(
        &mut self,
        parent: &<R as Renderer>::Element,
        marker: Option<&<R as Renderer>::Node>,
    ) {
        self.with_value_mut(|state| {
            state.mount(parent, marker);
        });
    }

    fn insert_before_this(
        &self,
        parent: &<R as Renderer>::Element,
        child: &mut dyn Mountable<R>,
    ) -> bool {
        self.with_value_mut(|value| value.insert_before_this(parent, child))
            .unwrap_or(false)
    }
}

// Extends to track suspense
impl<const TRANSITION: bool, Fal, Fut> Suspend<TRANSITION, Fal, Fut> {
    pub fn track(self) -> Suspend<TRANSITION, Fal, ScopedFuture<Fut>> {
        let Suspend { fallback, fut } = self;
        Suspend {
            fallback,
            fut: ScopedFuture::new(fut),
        }
    }
}

// Dynamic attributes
impl<F, V, R> AttributeValue<R> for F
where
    F: Fn() -> V + 'static,
    V: AttributeValue<R>,
    V::State: 'static,
    R: Renderer,
    R::Element: Clone + 'static,
{
    type State = RenderEffect<V::State>;

    fn to_html(self, key: &str, buf: &mut String) {
        let value = self();
        value.to_html(key, buf);
    }

    fn to_template(_key: &str, _buf: &mut String) {}

    fn hydrate<const FROM_SERVER: bool>(
        self,
        key: &str,
        el: &<R as Renderer>::Element,
    ) -> Self::State {
        let key = key.to_owned();
        let el = el.to_owned();
        RenderEffect::new(move |prev| {
            let value = self();
            if let Some(mut state) = prev {
                value.rebuild(&key, &mut state);
                state
            } else {
                value.hydrate::<FROM_SERVER>(&key, &el)
            }
        })
    }

    fn build(self, el: &<R as Renderer>::Element, key: &str) -> Self::State {
        let key = key.to_owned();
        let el = el.to_owned();
        RenderEffect::new(move |prev| {
            let value = self();
            if let Some(mut state) = prev {
                value.rebuild(&key, &mut state);
                state
            } else {
                value.build(&el, &key)
            }
        })
    }

    fn rebuild(self, key: &str, state: &mut Self::State) {
        state.with_value_mut_and_as_owner(|state| {
            let value = self();
            value.rebuild(key, state);
        });
    }

    /*     fn build(self) -> Self::State {
        RenderEffect::new(move |prev| {
            let value = self();
            if let Some(mut state) = prev {
                value.rebuild(&mut state);
                state
            } else {
                value.build()
            }
        })
    }

    #[track_caller]
    fn rebuild(self, state: &mut Self::State) {
        /* crate::log(&format!(
            "[REBUILDING EFFECT] Is this a mistake? {}",
            std::panic::Location::caller(),
        )); */
        let old_effect = std::mem::replace(state, self.build());
    } */
}

// Dynamic properties
// These do update during hydration because properties don't exist in the DOM
impl<F, V, R> IntoProperty<R> for F
where
    F: Fn() -> V + 'static,
    V: IntoProperty<R>,
    V::State: 'static,
    R: DomRenderer,
    R::Element: Clone + 'static,
{
    type State = RenderEffect<V::State>;

    fn hydrate<const FROM_SERVER: bool>(
        self,
        el: &<R as Renderer>::Element,
        key: &str,
    ) -> Self::State {
        let key = key.to_owned();
        let el = el.to_owned();
        RenderEffect::new(move |prev| {
            let value = self();
            if let Some(mut state) = prev {
                value.rebuild(&mut state, &key);
                state
            } else {
                value.hydrate::<FROM_SERVER>(&el, &key)
            }
        })
    }

    fn build(self, el: &<R as Renderer>::Element, key: &str) -> Self::State {
        let key = key.to_owned();
        let el = el.to_owned();
        RenderEffect::new(move |prev| {
            let value = self();
            if let Some(mut state) = prev {
                value.rebuild(&mut state, &key);
                state
            } else {
                value.build(&el, &key)
            }
        })
    }

    fn rebuild(self, state: &mut Self::State, key: &str) {
        state.with_value_mut_and_as_owner(|state| {
            let value = self();
            value.rebuild(state, key);
        });
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::{
        html::element::{button, main, HtmlElement},
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

    #[test]
    fn update_dynamic_element_among_siblings() {
        let rt = create_runtime();
        let count = RwSignal::new(0);
        let app: HtmlElement<_, _, _, MockDom> = main(
            (),
            button(
                (),
                ("Hello, my ", move || count.get().to_string(), " friends."),
            ),
        );
        let el = app.build();
        assert_eq!(
            el.el.to_debug_html(),
            "<main><button>Hello, my 0 friends.</button></main>"
        );
        count.set(42);
        assert_eq!(
            el.el.to_debug_html(),
            "<main><button>Hello, my 42 friends.</button></main>"
        );
        rt.dispose();
    }
}
 */
