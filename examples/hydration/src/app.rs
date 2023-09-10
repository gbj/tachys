use leptos_reactive::create_effect;
use leptos_reactive::create_runtime;
use leptos_reactive::create_signal;
use leptos_reactive::SignalGet;
use leptos_reactive::SignalUpdate;
use tachy_maccy::view;
use tachydom::html::element::*;
use tachydom::html::event;
use tachydom::html::event::on;
use tachydom::hydration::Cursor;
use tachydom::view::Position;
use tachydom::view::ToTemplate;
use tachydom::view::View;

#[derive(Debug)]
pub struct App<C: View>(C);

pub fn my_app() -> App<impl View> {
    let rt = create_runtime();
    let (count, set_count) = create_signal(0);
    create_effect(move |_| {
        tachydom::log(&count.get().to_string());
    });
    let view = view! {
        <p class:bar=move || count.get() % 2 == 0 class="foo" class:baz=true class:not=|| false>
            This is <strong>"very"</strong> cool stuff.<span></span>
        </p>
        <button
            on:click=move |ev| set_count.update(|n| *n += 1)
        >
            {move || count.get().to_string()}
        </button>
    };

    /* button(
        on(event::click, move |ev| set_count.update(|n| *n += 1)),
        move || count.get().to_string(),
    ); */
    App::new(view)
}

impl<C: View> App<C> {
    pub fn new(view: C) -> Self {
        Self(view)
    }
}

impl<C: View> App<C> {
    pub fn hydrate(self) {
        let mut cursor = Cursor::new(tachydom::dom::body());
        // hydrate from HTML
        let mut position = Position::FirstChild;
        self.0.hydrate::<true>(&mut cursor, &mut position);
    }

    /* pub fn client_render(self) {
        // hydrate from <template>
        let mut html = String::new();
        let mut position = Position::Root;
        C::to_template(&mut html, &mut position);
        let tpl = document()
            .create_element("template")
            .unwrap()
            .unchecked_into::<HtmlTemplateElement>();
        tpl.set_inner_html(&html);
        let contents = tpl.content().clone_node_with_deep(true).unwrap();
        let mut cursor = Cursor::new(contents.clone().unchecked_into());
        let mut position = Position::FirstChild;
        self.0.hydrate::<false>(&mut cursor, &mut position);
        body().append_child(&contents);
    } */

    pub fn to_html(&self) -> String {
        let mut buf = String::from(
            r#"<!DOCTYPE html>
<html>
	<head>
    <script>import('/pkg/hydration_ex.js').then(m => m.default("/pkg/hydration_ex.wasm").then(() => m.hydrate()));</script>
    </head>"#,
        );
        let mut position = Position::Root;
        self.0.to_html(&mut buf, &mut position);
        buf.push_str("<script>__LEPTOS_PENDING_RESOURCES = [];__LEPTOS_RESOLVED_RESOURCES = new Map();__LEPTOS_RESOURCE_RESOLVERS = new Map();</script></body></html>");
        buf
    }
}
