use crate::dom::body;
use crate::dom::document;
use crate::dom::log;
use crate::html::attribute::*;
use crate::html::element::*;
use crate::html::event;
use crate::html::event::on;
use crate::hydration::Cursor;
use crate::view::strings::Static;
use crate::view::Position;
use crate::view::View;
use wasm_bindgen::JsCast;
use web_sys::HtmlTemplateElement;

#[derive(Debug)]
pub struct App<C: View>(C);

pub fn my_app() -> App<impl View> {
    let view = main(
        (),
        (
            ((), ()),
            div(
                (),
                (
                    p(StaticAttr::<Id<()>, "test">::new(), Static::<"Some text.">),
                    input(r#type("text")),
                    (),
                    p(
                        (),
                        (Static::<"Text. ">, Static::<"More text in another node.">),
                    ),
                ),
            ),
            button(on(event::click, |ev| log("clicked")), Static::<"click me">),
        ),
    );
    App::new(view)
}

impl<C: View> App<C> {
    pub fn new(view: C) -> Self {
        Self(view)
    }
}

impl<C: View> App<C> {
    pub fn hydrate(self) {
        let mut cursor = Cursor::new(crate::dom::body());
        // hydrate from HTML
        self.0.hydrate::<true>(&mut cursor, Position::FirstChild);
    }

    pub fn client_render(self) {
        // hydrate from <template>
        let mut html = String::new();
        C::to_template(&mut html, Position::Root);
        let tpl = document()
            .create_element("template")
            .unwrap()
            .unchecked_into::<HtmlTemplateElement>();
        tpl.set_inner_html(&html);
        let contents = tpl.content().clone_node_with_deep(true).unwrap();
        let mut cursor = Cursor::new(contents.clone().unchecked_into());
        self.0.hydrate::<false>(&mut cursor, Position::FirstChild);
        body().append_child(&contents);
    }

    pub fn to_html(&self) -> String {
        let mut buf = String::from(
            r#"<!DOCTYPE html>
<html>
	<head>
    <script>import('/pkg/leptos_start.js').then(m => m.default("/pkg/leptos_start.wasm").then(() => m.hydrate()));</script>
    </head>"#,
        );
        self.0.to_html(&mut buf, crate::view::Position::Root);
        buf.push_str("</body></html>");
        buf
    }
}
