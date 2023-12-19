use std::mem;
use tachy_reaccy_macro::Store;
use tachys::{
    prelude::*,
    tachy_reaccy::store::Store,
    tachydom::{
        dom::{body, event_target_value, log},
        node_ref::NodeRef,
    },
};

#[derive(Store, Clone, Default)]
struct State {
    pub name: String,
    pub todos: Vec<Todo>,
}

#[derive(Store, Clone, Default, Debug)]
struct Todo {
    pub title: String,
    pub completed: bool,
}

pub fn app() -> impl Render<Dom> {
    let store = Store::new(State {
        name: "Greg".to_string(),
        todos: vec![Todo {
            title: "First task".to_string(),
            completed: false,
        }],
    });
    let input_ref = NodeRef::new();

    view! {
        <pre>{move || store.at().name().get()}</pre>
        <input
            type="text"
            prop:value=move || store.at().name().get()
            on:input=move |ev| store.at_mut().name().set(event_target_value(&ev))
        />
        <hr/>
        <form
            on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().unwrap();
                store.at_mut().todos().update(|n| n.push(Todo {
                    title: input.value(),
                    completed: false
                }));
            }
        >
            <input
                type="text"
                name="title"
                node_ref=input_ref
            />
            <input type="submit" value="Add Todo"/>
        </form>
        <ul>
            {move || {
                store.at().todos().with(|todos| {
                    todos.iter().enumerate().map(|(idx, todo)| {
                        view! {
                            <li style:text-decoration=move || store.at().todos().index(idx).completed().get().then_some("line-through").unwrap_or_default()>
                                {move || store.at().todos().index(idx).title().get()}
                                <input type="checkbox"
                                    prop:checked=move || store.at().todos().index(idx).completed().get()
                                    on:click=move |_| {
                                        store.at_mut().todos().index(idx).completed().set(!store.at().todos().index(idx).completed().get())
                                    }
                                />
                            </li>
                        }
                    }).collect::<Vec<_>>()
                })
            }}
        </ul>
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
