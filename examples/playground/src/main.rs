use std::mem;
use tachy_reaccy_macro::Store;
use tachys::{
    prelude::*,
    show::Show,
    tachy_reaccy::{
        render_effect::RenderEffect,
        store::{ArcStore, Store},
    },
    tachydom::{
        dom::{body, event_target_value, log},
        html::{
            element::{p, HtmlElement, Input},
            event,
        },
        node_ref::NodeRef,
        view::error_boundary::Try,
    },
};
use tracing_subscriber::prelude::*;

#[derive(Store, Clone, Default)]
struct SomeStoreStruct {
    pub name: String,
    pub count: usize,
}

pub fn app() -> impl Render<Dom> {
    let store = Store::new(SomeStoreStruct {
        name: "Bob".to_string(),
        count: 37,
    });

    // effects are canceled on drop; TODO better API here
    mem::forget(Effect::new(move |_| {
        log(&format!("count is {:?}", store.at().count().get()));
    }));

    view! {
        <button on:click=move |_| {
            store.at_mut().count().update(|n| *n += 1);
        }>
            {move || store.at().count().get()}
        </button>
        {move ||  store.at().name().get()}
    }
}

fn main() {
    //console_error_panic_hook::set_once();

    /* tracing_subscriber::fmt()
        // this level can be adjusted to filter out messages of different levels of importance
        .with_max_level(tracing::Level::TRACE)
        .without_time()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_writer(tracing_subscriber_wasm::MakeConsoleWriter::default())
        .with_ansi(false)
        .pretty()
        .finish()
        .init();
    tracing::info!("opening app"); */
    Root::global(|| {
        let view = app(); //fetch_example();
        let mut mountable = view.build();
        mountable.mount(&body(), None);
        // effects etc. will cancel on drop, so we forget initial state of app
        std::mem::forget(mountable);
    });
}
