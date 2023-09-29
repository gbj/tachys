use super::{Mountable, PositionState, Render, RenderHtml, ToTemplate};
use crate::{
    dom::document,
    hydration::Cursor,
    renderer::{dom::Dom, Renderer},
};
use once_cell::unsync::{Lazy, OnceCell};
use std::marker::PhantomData;
use wasm_bindgen::JsCast;
use web_sys::HtmlTemplateElement;

thread_local! {
    static TEMPLATE_ELEMENT: Lazy<HtmlTemplateElement> =
        Lazy::new(|| document().create_element("template").unwrap().unchecked_into());
}

pub struct ViewTemplate<V: Render<R> + ToTemplate, R: Renderer> {
    view: V,
    rndr: PhantomData<R>,
}

impl<V: Render<R> + ToTemplate, R: Renderer> ViewTemplate<V, R> {
    pub fn new(view: V) -> Self {
        Self {
            view,
            rndr: PhantomData,
        }
    }

    fn to_template() -> HtmlTemplateElement {
        thread_local! {
            static TEMPLATE: OnceCell<HtmlTemplateElement> = OnceCell::new();
        }

        TEMPLATE.with(|t| {
            t.get_or_init(|| {
                let tpl = TEMPLATE_ELEMENT.with(|t| {
                    t.clone_node()
                        .unwrap()
                        .unchecked_into::<HtmlTemplateElement>()
                });
                let mut buf = String::new();
                V::to_template(&mut buf, &mut Default::default());
                tpl.set_inner_html(&buf);
                tpl
            })
            .clone()
        })
    }
}

impl<V, R> Render<Dom> for ViewTemplate<V, R>
where
    V: RenderHtml<R> + ToTemplate,
    R: Renderer,
    R::Node: Clone,
    R::Element: Clone,
    V::State: Mountable<R>,
{
    type State = V::State;

    fn build(self) -> Self::State {
        todo!()
        /* let tpl = Self::to_template();
        let contents = tpl.content().clone_node_with_deep(true).unwrap();
        self.view.hydrate::<false>(
            &Cursor::new(contents.unchecked_into()),
            &Default::default(),
        ) */
    }

    fn rebuild(self, state: &mut Self::State) {
        self.view.rebuild(state)
    }
}

impl<V, R> RenderHtml<R> for ViewTemplate<V, R>
where
    V: RenderHtml<R> + ToTemplate,
    R: Renderer,
    R::Node: Clone,
    R::Element: Clone,
    V::State: Mountable<R>,
{
    fn to_html(&mut self, buf: &mut String, position: &PositionState) {
        self.view.to_html(buf, position)
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor<R>,
        position: &PositionState,
    ) -> Self::State {
        self.view.hydrate::<FROM_SERVER>(cursor, position)
    }
}
