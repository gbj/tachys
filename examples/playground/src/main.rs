use std::{mem, sync::atomic::AtomicUsize};
//use tachy_reaccy_macro::Store;
use tachys::{
    prelude::*,
    show::Show,
    tachy_reaccy::store::{
        ArcStore, AtIndex, AtKey, KeyedStoreFieldIterator, Store, Subfield,
    },
    tachydom::{
        dom::{body, event_target_value, log},
        node_ref::NodeRef,
        view::keyed::keyed,
    },
};

/*static NEXT_ID: AtomicUsize = AtomicUsize::new(3);

fn next_id() -> usize {
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

#[derive(Store)]
struct TupleExample(String, usize, String);

#[derive(Store, Clone, Default, Debug)]
struct State {
    pub name: String,
    #[store(key = id: usize)]
    pub todos: Vec<Todo>,
}

#[derive(Store, Clone, Default, Debug)]
struct Todo {
    pub id: usize,
    pub title: String,
    pub completed: bool,
} */

/* pub fn app() -> impl Render<Dom> {
    let store = Store::new(State {
        name: "Greg".to_string(),
        todos: vec![
            Todo {
                id: 0,
                title: "Fine-grained reactive stores.".to_string(),
                completed: false,
            },
            Todo {
                id: 1,
                title: "???".to_string(),
                completed: false,
            },
            Todo {
                id: 2,
                title: "Profit!!!".to_string(),
                completed: false,
            },
        ],
    });

    let input_ref = NodeRef::new();

    let todos_keyed = move || {
        keyed(
            store.todos().iter_keyed(|row: &Todo| row.id),
            |item| item.id().get_untracked(),
            move |row| TodoRow(store, row),
        )
    };

    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            store.todos().update(|n| n.push(Todo {
                id: next_id(),
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
        <div style="display: flex">
            <ol>{todos_keyed}</ol>
        </div>
    }
}

fn TodoRow(
    store: Store<State>,
    todo: AtKey<
        Subfield<Store<State>, State, Vec<Todo>>,
        Vec<Todo>,
        Todo,
        usize,
    >,
) -> impl Render<Dom> {
    let completed = todo.completed();
    let title = todo.title();

    let editing = RwSignal::new(false);

    view! {
        <li style:text-decoration={
            move || completed.get().then_some("line-through").unwrap_or_default()
        }
            class:foo=move || completed.get()
        >
            <p class:hidden=move || editing.get()
                on:click=move |_| {
                    editing.update(|n| *n = !*n);
                }
            >
                {move || title.get()}
            </p>
            <input
                class:hidden=move || !(editing.get())
                type="text"
                prop:value=move || title.get()
                on:change=move |ev| {
                    title.set(event_target_value(&ev));
                    editing.set(false);
                }
                on:blur=move |_| editing.set(false)
                autofocus
            />
            <input type="checkbox"
                prop:checked=move || completed.get()
                on:click=move |_| {
                    completed.update(|n| *n = !*n)
                }
            />
            <button on:click=move |_| {
                let id = todo.id().get_untracked();
                store.todos().update(|todos| {
                    todos.retain(|item| item.id != id);
                });
            }>
                "X"
            </button>
        </li>
    }
} */

pub fn app() -> impl Render<Dom> {
    let text = ArcRwSignal::new(String::from("Hi"));
    view! {
        <button on:click={
            move |_| text.update(|n| n.push_str("!!!"))
        }>
            {let text = text.clone(); move || text.get()}
        </button>
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
