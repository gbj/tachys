mod dom;
mod view;

use dom::{Dom, *};
use leptos_reactive::*;
use view::{html::attribute::on, keyed::Keyed, Mount, View};

pub fn mount<T: View>(view: T) {
    let parent = Dom::body();
    let mut state = view.build();
    T::mount(&mut state, Mount::OnlyChild { parent });
    Dom::flush();
}

const MANY_COUNTERS: usize = 1000;

type CounterHolder = Vec<(usize, (ReadSignal<i32>, WriteSignal<i32>))>;

#[derive(Copy, Clone)]
struct CounterUpdater {
    set_counters: WriteSignal<CounterHolder>,
}

fn main() {
    console_error_panic_hook::set_once();
    let rt = create_runtime();
    rt.add_post_effects_hook(Dom::flush);

    mount({
        let (next_counter_id, set_next_counter_id) = create_signal(0);
        let (counters, set_counters) = create_signal::<CounterHolder>(vec![]);
        provide_context(CounterUpdater { set_counters });

        let add_counter = move |_| {
            let id = next_counter_id();
            let sig = create_signal(0);
            set_counters.update(move |counters| counters.push((id, sig)));
            set_next_counter_id.update(|id| *id += 1);
        };

        let add_many_counters = move |_| {
            let next_id = next_counter_id();
            let new_counters = (next_id..next_id + MANY_COUNTERS).map(|id| {
                let signal = create_signal(0);
                (id, signal)
            });

            set_counters.update(move |counters| counters.extend(new_counters));
            set_next_counter_id.update(|id| *id += MANY_COUNTERS);
        };

        let clear_counters = move |_| {
            set_counters.update(|counters| counters.clear());
        };

        div(
            (),
            (
                button(on("click", add_counter), "Add Counter"),
                button(
                    on("click", add_many_counters),
                    format!("Add {MANY_COUNTERS} Counters"),
                ),
                button(on("click", clear_counters), "Clear Counters"),
                p(
                    (),
                    (
                        "Total: ",
                        move || {
                            counters
                                .get()
                                .iter()
                                .map(|(_, (count, _))| count())
                                .sum::<i32>()
                                .to_string()
                        },
                        " from ",
                        move || counters().len().to_string(),
                        " counters",
                    ),
                ),
                ul(
                    (),
                    Keyed {
                        each_signal: counters,
                        key_fn: |(key, _)| *key,
                        view_fn: |item: &(usize, (ReadSignal<i32>, WriteSignal<i32>))| {
                            let (id, (value, set_value)) = *item;

                            let CounterUpdater { set_counters } = use_context().unwrap();
                            on_cleanup(|| {
                                web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(
                                    "deleted a row",
                                ))
                            });

                            li(
                                (),
                                (
                                    button(
                                        on("click", move |_| {
                                            set_value.update(move |value| *value -= 1)
                                        }),
                                        "-1",
                                    ),
                                    span((), move || value.get()),
                                    button(
                                        on("click", move |_| {
                                            set_value.update(move |value| *value += 1)
                                        }),
                                        "+1",
                                    ),
                                    button(
                                        on("click", move |_| {
                                            set_counters.update(move |counters| {
                                                counters.retain(|(counter_id, _)| counter_id != &id)
                                            })
                                        }),
                                        "x",
                                    ),
                                ),
                            )
                        },
                    },
                ),
            ),
        )
    });
}
