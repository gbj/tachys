use std::mem;
use tachy_reaccy_macro::Store;
use tachys::{
    prelude::*,
    tachy_reaccy::store::{ArcStore, Store},
    tachydom::{
        dom::{body, event_target_value, log},
        node_ref::NodeRef,
    },
};

#[derive(Store, Clone, Default, Debug)]
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
        <ul>
            <form on:submit=move |ev| {
                ev.prevent_default();
                store.at().todos().write().update(|n| n.push(Todo {
                    title: input_ref.get().unwrap().value(),
                    completed: false
                }));
            }>
                <label>
                    "Add a Todo"
                    <input type="text" node_ref=input_ref/>
                </label>
                <input type="submit"/>
            </form>
            {move || {
                store
                    .at()
                    .todos()
                    .iter()
                    .map(|todo| {
                        let completed = todo.clone().completed().rw();
                        let title = todo.title().rw();
                        view! {
                            <li style:text-decoration={
                                move || completed.get().then_some("line-through").unwrap_or_default()
                            }>
                                {move || title.get()}
                                <input type="checkbox"
                                    prop:checked={
                                        move || completed.get()
                                    }
                                    on:click=move |_| {
                                        completed.update(|n| *n = !*n)
                                    }
                                />
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()
            }}
        </ul>
    }

    //let input_ref = NodeRef::new();

    /* let todos = move || {
        store
            .at()
            .todos()
            .iter_mut()
            .map(|todo| {
                view! {
                    <li style:text-decoration={
                        let todo = todo.clone();
                        move || todo.clone().completed().get().then_some("line-through").unwrap_or_default()
                    }>
                        {
                            let todo = todo.clone();
                            move || todo.clone().title().get()
                        }
                        <input type="checkbox"
                            prop:checked={
                                let todo = todo.clone();
                                move || todo.clone().completed().get()
                            }
                            on:click=move |_| {
                                //todo.completed().set(!store.at().todos().index(idx).completed().get())
                            }
                        />
                    </li>
                }
            })
            .collect::<Vec<_>>()
    }; */

    /*view! {
        <input
            type="text"
            on:input=move |ev| name.set(event_target_value(&ev))
        />
        <pre>{move || name.get()}</pre>
        <pre>"second: " {move || second.get()}</pre>
        //<pre>{move || store.at().name().rw().get()}</pre>
        <input
            type="text"
            prop:value=move || store.at().name().read().get()
            on:input=move |ev| store.at_mut().name().set(event_target_value(&ev))
        />
        <hr/>
        <form
            on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().unwrap();
                store.at().todos().update(|n| n.push(Todo {
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
            //{todos}
        </ul>
    }*/
}

fn main() {
    console_error_panic_hook::set_once();

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
