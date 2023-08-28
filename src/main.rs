use std::fmt::Debug;
mod component;
mod dom;
mod view;

use component::{Component, ComponentLink};
use dom::{Attr, Dom, El};
use view::{
    html::{
        attribute::{on, On},
        Html,
    },
    Static, View,
};

#[derive(Debug)]
struct Counter(i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Msg {
    Decrement,
    Increment,
}

impl Component for Counter {
    type Msg = Msg;
    type View = (
        Static<&'static str>,
        String,
        Html<((Attr, String), On), Static<&'static str>>,
    );

    fn update(&mut self, msg: Self::Msg) {
        match msg {
            Msg::Decrement => self.0 -= 1,
            Msg::Increment => self.0 += 1,
        }
    }

    fn view(&self, link: &ComponentLink<Self>) -> Self::View {
        let link = link.clone();
        (
            Static("hello, world!"),
            self.0.to_string(),
            Html {
                tag: El::button,
                attributes: (
                    (Attr::id, self.0.to_string()),
                    on("click", move |_| {
                        link.send(Msg::Increment);
                    }),
                ),
                children: Static("click me"),
            },
        )
    }
}

fn main() {
    let counter = Counter(0);
    let mut view = counter.build();
    Counter::mount(&mut view, Dom::body());
    Dom::flush();
}
