use std::mem;
use tachy_reaccy_macro::Store;
use tachys::{
    prelude::*,
    show::Show,
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
        todos: vec![
            Todo {
                title: "Fine-grained reactive stores.".to_string(),
                completed: false,
            },
            Todo {
                title: "???".to_string(),
                completed: false,
            },
            Todo {
                title: "Profit!!!".to_string(),
                completed: false,
            },
        ],
    });

    let input_ref = NodeRef::new();

    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            store.at().todos().arc_write().update(|n| n.push(Todo {
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
        <ol>
            {move || {
                store
                    .at()
                    .todos()
                    .iter()
                    .enumerate()
                    .map(|(idx, todo)| {
                        let completed = todo.clone().completed().arc_rw();
                        let title = todo.title().arc_rw();

                        let (editing, set_editing) = signal(false);

                        view! {
                            <li style:text-decoration={
                                let completed = completed.clone();
                                move || completed.get().then_some("line-through").unwrap_or_default()
                            }
                                class:foo=move || completed.get()
                            >
                                <p class:hidden=move || editing.get()
                                    on:click=move |_| set_editing.set(true)
                                >
                                    {let title = title.clone();  move || title.get()}
                                </p>
                                <input
                                    class:hidden=move || !(editing.get())
                                    type="text"
                                    prop:disabled=move || !(editing.get())
                                    prop:value={let title = title.clone(); move || title.get()}
                                    on:change={
                                        let title = title.clone();
                                        move |ev| {
                                            title.set(event_target_value(&ev));
                                            set_editing.set(false);
                                        }
                                    }
                                    on:blur=move |_| set_editing.set(false)
                                />
                                <input type="checkbox"
                                    prop:checked={
                                        let completed = completed.clone();
                                        move || completed.get()
                                    }
                                    on:click={let completed = completed.clone(); move |_| {
                                        completed.update(|n| *n = !*n)
                                    }}
                                />
                                <button on:click=move |_| {
                                    store.at().todos().arc_write().update(|n| {
                                        n.remove(idx);
                                    });
                                }>
                                    "X"
                                </button>
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()
            }}
        </ol>
        <pre>
            {move || format!("{:#?}", store.at().todos().arc_read().get())}
        </pre>
    }
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
