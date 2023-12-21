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

struct StateFields {}
impl StateFields {
    fn name() -> () {}
}
trait StateReadStoreFields<OriginTy, Path, Reader, Writer> {
    fn name(
        self,
    ) -> ::tachys::tachy_reaccy::store::StorePathBuilder<
        OriginTy,
        String,
        impl Iterator<Item = ::tachys::tachy_reaccy::store::StorePathSegment>,
        Box<dyn Fn(&OriginTy) -> &String>,
        impl Fn(&mut OriginTy) -> &mut String + 'static,
    >
    where
        OriginTy: 'static,
        Path: Iterator<Item = ::tachys::tachy_reaccy::store::StorePathSegment>,
        Reader: Fn(&OriginTy) -> &State + 'static,
        Writer: Fn(&mut OriginTy) -> &mut State + 'static;
}
impl<OriginTy, Path, Reader, Writer>
    StateReadStoreFields<OriginTy, Path, Reader, Writer>
    for ::tachys::tachy_reaccy::store::StorePathBuilder<
        OriginTy,
        State,
        Path,
        Reader,
        Writer,
    >
where
    OriginTy: 'static,
{
    #[inline(always)]
    fn name(
        self,
    ) -> ::tachys::tachy_reaccy::store::StorePathBuilder<
        OriginTy,
        String,
        impl Iterator<Item = ::tachys::tachy_reaccy::store::StorePathSegment>,
        Box<dyn Fn(&OriginTy) -> &String>,
        impl Fn(&mut OriginTy) -> &mut String + 'static,
    >
    where
        OriginTy: 'static,
        Path: Iterator<Item = ::tachys::tachy_reaccy::store::StorePathSegment>,
        Reader: Fn(&OriginTy) -> &State + 'static,
        Writer: Fn(&mut OriginTy) -> &mut State + 'static,
    {
        let ::tachys::tachy_reaccy::store::StorePathBuilder {
            data,
            path,
            reader,
            writer,
            ty,
        } = self;
        let reader = Box::new(map_prev(reader, |prev| &prev.name));
        let writer = map_prev_mut(writer, |prev| &mut prev.name);
        ::tachys::tachy_reaccy::store::StorePathBuilder {
            data,
            path: path.chain(::std::iter::once(
                ::tachys::tachy_reaccy::store::StorePathSegment::from(
                    StateFields::name as usize,
                ),
            )),
            reader, //move |orig| &(reader(orig)).name,
            writer,
            ty: ::std::marker::PhantomData,
        }
    }
}

fn map_prev<'a, Orig, Prev, PrevT, Next, T>(
    prev: Prev,
    next: Next,
) -> impl Fn(&'a Orig) -> &'a T
where
    Orig: 'a,
    PrevT: 'a,
    T: 'a,
    Prev: Fn(&'a Orig) -> &'a PrevT,
    Next: Fn(&'a PrevT) -> &'a T,
{
    move |orig| {
        let prev = prev(orig);
        next(prev)
    }
}

fn map_prev_mut<'a, Orig, Prev, PrevT, Next, T>(
    prev: Prev,
    next: Next,
) -> impl Fn(&'a mut Orig) -> &'a mut T
where
    Orig: 'a,
    PrevT: 'a,
    T: 'a,
    Prev: Fn(&'a mut Orig) -> &'a mut PrevT,
    Next: Fn(&'a mut PrevT) -> &'a mut T,
{
    move |orig| {
        let prev = prev(orig);
        next(prev)
    }
}

#[derive(/* Store, */ Clone, Default)]
struct State {
    pub name: String,
    //pub todos: Vec<Todo>,
} /*

  #[derive(Store, Clone, Default, Debug)]
  struct Todo {
      pub title: String,
      pub completed: bool,
  } */

pub fn app() -> impl Render<Dom> {
    let store = ArcStore::new(State {
        name: String::new(),
    });
    let name = store.at();
    let name = name.name();
    let name = name.end();
    /* let store = ArcStore::new(State {
        name: "Greg".to_string(),
        todos: vec![Todo {
            title: "First task".to_string(),
            completed: false,
        }],
    });
    let name = store.at().name();
    let name = name.end(); */
    /*     let store = Store::new(State {
        name: "Greg".to_string(),
        todos: vec![Todo {
            title: "First task".to_string(),
            completed: false,
        }],
    });
    let input_ref = NodeRef::new();

    let todos = move || {
        store
            .at_mut()
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
    };

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
            {todos}
        </ul>
    } */
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
