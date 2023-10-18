use super::{Mountable, PositionState, Render, RenderHtml, ToTemplate};
use crate::{dom::document, hydration::Cursor, renderer::dom::Dom};
use once_cell::unsync::{Lazy, OnceCell};
use wasm_bindgen::JsCast;
use web_sys::HtmlTemplateElement;

thread_local! {
    static TEMPLATE_ELEMENT: Lazy<HtmlTemplateElement> =
        Lazy::new(|| document().create_element("template").unwrap().unchecked_into());
}

pub struct ViewTemplate<V: Render<Dom> + ToTemplate> {
    view: V,
}

impl<V: Render<Dom> + ToTemplate> ViewTemplate<V> {
    pub fn new(view: V) -> Self {
        Self { view }
    }

    fn to_template() -> HtmlTemplateElement {
        thread_local! {
            static TEMPLATE: OnceCell<HtmlTemplateElement> = OnceCell::new();
        }

        let tpl = TEMPLATE.with(|t| {
            t.get_or_init(|| {
                let tpl = TEMPLATE_ELEMENT.with(|t| {
                    t.clone_node()
                        .unwrap()
                        .unchecked_into::<HtmlTemplateElement>()
                });
                let mut buf = String::new();
                let mut class = String::new();
                let mut style = String::new();
                V::to_template(
                    &mut buf,
                    &mut class,
                    &mut style,
                    &mut Default::default(),
                );
                tpl.set_inner_html(&buf);
                tpl
            })
            .clone()
        });
        #[cfg(debug_assertions)]
        web_sys::console::log_1(&tpl);
        tpl
    }
}

impl<V> Render<Dom> for ViewTemplate<V>
where
    V: RenderHtml<Dom> + ToTemplate,
    V::State: Mountable<Dom>,
{
    type State = V::State;

    fn build(self) -> Self::State {
        let tpl = Self::to_template();
        let contents = tpl.content().clone_node_with_deep(true).unwrap();
        self.view.hydrate::<false>(
            &Cursor::new(contents.unchecked_into()),
            &Default::default(),
        )
    }

    fn rebuild(self, state: &mut Self::State) {
        self.view.rebuild(state)
    }
}

impl<V> RenderHtml<Dom> for ViewTemplate<V>
where
    V: RenderHtml<Dom> + ToTemplate,
    V::State: Mountable<Dom>,
{
    fn to_html(self, buf: &mut String, position: &PositionState) {
        self.view.to_html(buf, position)
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor<Dom>,
        position: &PositionState,
    ) -> Self::State {
        self.view.hydrate::<FROM_SERVER>(cursor, position)
    }
}
