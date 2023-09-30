use leptos_reactive::{
    create_effect, create_runtime, create_signal, SignalGet, SignalUpdate,
};
use tachy_maccy::view;
use tachydom::{
    html::{element::*, event, event::on},
    hydration::Cursor,
    renderer::dom::Dom,
    renderer::DomRenderer,
    view::{
        /* any_view::IntoAny, template::ViewTemplate, */ Position, Render,
        RenderHtml, ToTemplate,
    },
};

pub fn my_app() -> impl RenderHtml<Dom> {
    let rt = create_runtime();
    let (count, set_count) = create_signal(0);

    view! {
        <p
            class:bar=move || count.get() % 2 == 0
            class="foo"
            class:baz=true
            class:not=|| false
            style="font-weight: bold"
            style:color=move || if count.get() % 2 == 0 {
                "blue"
            } else {
                "red"
            }
            style:display="block"
        >
            This is <strong>"very"</strong> cool stuff.<span></span>
        </p>
        /* {move || if count() % 2 == 0 {
            view! { <div>"even"</div> }.into_any()
        } else {
            view! { <span>"odd"</span> }.into_any()
        }} */
        <button
            on:click=move |ev| {
                //tachydom::log("click");
                set_count.update(|n| *n += 1)
            }
        >
            {move || count.get().to_string()}
        </button>
        /* {move || (count() % 2 == 0).then(|| view! {
            <p>"Even"</p>
        })} */
    }
}
/*
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

    pub fn client_render(self) {
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
    }

    pub fn to_html(&self) -> String {
        let mut buf = String::from(
            r#"<!DOCTYPE html>
<html>
    <head>
    <script>import('/pkg/hydration_ex.js').then(m => m.default("/pkg/hydration_ex.wasm").then(() => m.hydrate()));</script>
    </head><body>"#,
        );
        self.0
            .to_html(&mut buf, &PositionState::new(Position::Root));
        buf.push_str("<script>__LEPTOS_PENDING_RESOURCES = [];__LEPTOS_RESOLVED_RESOURCES = new Map();__LEPTOS_RESOURCE_RESOLVERS = new Map();</script></body></html>");
        buf
    }
}
*/
