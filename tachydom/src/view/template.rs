use once_cell::unsync::{Lazy, OnceCell};
use wasm_bindgen::JsCast;
use web_sys::HtmlTemplateElement;

use super::{PositionState, Render, ToTemplate};
use crate::{dom::document, hydration::Cursor};

thread_local! {
    static TEMPLATE_ELEMENT: Lazy<HtmlTemplateElement> =
        Lazy::new(|| document().create_element("template").unwrap().unchecked_into());
}

pub struct ViewTemplate<V: Render + ToTemplate>(V);

impl<V: Render + ToTemplate> ViewTemplate<V> {
    pub fn new(view: V) -> Self {
        Self(view)
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

impl<V: Render + ToTemplate> Render for ViewTemplate<V> {
    type State = V::State;

    fn to_html(&self, buf: &mut String, position: &PositionState) {
        self.0.to_html(buf, position)
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        self.0.hydrate::<FROM_SERVER>(cursor, position)
    }

    fn build(self) -> Self::State {
        let tpl = Self::to_template();
        let contents = tpl.content().clone_node_with_deep(true).unwrap();
        self.0
            .hydrate::<false>(&Cursor::new(contents.unchecked_into()), &Default::default())
    }

    fn rebuild(self, state: &mut Self::State) {
        self.0.rebuild(state)
    }
}
