use std::fmt::Debug;
mod component;
mod dom;
mod view;

use component::{Component, ComponentLink, State};
use dom::{Attr, Dom, El};
use view::{
    html::{attribute::on, Html},
    Static, View,
};

#[derive(Debug)]
struct Counter(i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Msg {
    Decrement,
    Increment,
}

impl State for Counter {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) {
        match msg {
            Msg::Decrement => self.0 -= 1,
            Msg::Increment => self.0 += 1,
        }
    }
}

fn counter_view(state: &Counter, link: ComponentLink<Counter>) -> impl View {
    let link = link.clone();
    (
        Static("hello, world!"),
        state.0.to_string(),
        Html {
            tag: El::button,
            attributes: (
                (Attr::id, state.0.to_string()),
                on("click", move |_| {
                    link.send(Msg::Increment);
                }),
            ),
            children: Static("click me"),
        },
    )
}

fn main() {
    let counter = Component::new(Counter(0), counter_view);
    let mut view = counter.build();
    (ComponentLink::<Counter>, Rc::<RefCell::<<impl View as View>::State>>)::mount(view, Dom::body());
    let link = view.0.clone();
    Dom::flush();

    /*     let button = document.create_element("button").unwrap();
    button.set_inner_html("-1");
    body.append_child(&button);
    button.add_event_listener_with_callback(
        "click",
        wasm_bindgen::prelude::Closure::wrap(Box::new({
            let link = link.clone();
            move |_| {
                web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("Decrement"));
                link.send(Msg::Decrement);
            }
        }) as Box<dyn FnMut(web_sys::Event)>)
        .into_js_value()
        .as_ref()
        .unchecked_ref(),
    );
    let button = document.create_element("button").unwrap();
    button.set_inner_html("+1");
    body.append_child(&button);
    button.add_event_listener_with_callback(
        "click",
        wasm_bindgen::prelude::Closure::wrap(Box::new({
            move |_| {
                web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("Increment"));
                link.send(Msg::Increment);
            }
        }) as Box<dyn FnMut(web_sys::Event)>)
        .into_js_value()
        .as_ref()
        .unchecked_ref(),
    ); */
}
