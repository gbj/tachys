use gtk::{prelude::*, Application, ApplicationWindow, Button, Orientation};
use std::future::pending;
use tachy_reaccy::prelude::*;
use tachydom::view::{keyed::keyed, strings::StrState, Mountable, Render};
use tachygtk::{button, r#box, Box_, Element, ElementState, TachyGtk};
mod tachygtk;

const APP_ID: &str = "dev.leptos.Counter";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        println!("X");
        let view = ui();

        // Connect to "activate" signal of `app`
        let mut state = view.build();

        let window = ApplicationWindow::builder()
            .application(app)
            .title("TachyGTK")
            .child(&state.0 .0)
            .build();

        // Present window
        window.present();

        println!("mounting");
    });
    app.run();
}

fn ui() -> Box_<impl Render<TachyGtk>> {
    let value = Signal::new(0);
    let rows = Signal::new(vec![1, 2, 3, 4, 5]);

    Effect::new(move |_| {
        println!("value = {}", value.get());
    });

    r#box(
        Orientation::Vertical,
        12,
        (
            r#box(
                Orientation::Vertical,
                12,
                (
                    r#box(
                        Orientation::Horizontal,
                        12,
                        (
                            button("-1", move |_| value.update(|n| *n -= 1)),
                            move || value.get().to_string(),
                            button("+1", move |_| value.update(|n| *n += 1)),
                        ),
                    ),
                    move || (value.get() % 2 == 0).then_some("Even!"),
                ),
            ),
            r#box(
                Orientation::Vertical,
                12,
                (
                    button("Swap", move |_| {
                        rows.update(|items| {
                            items.swap(1, 3);
                            println!("{items:?}");
                        })
                    }),
                    r#box(Orientation::Vertical, 12, move || {
                        keyed(rows.get(), |k| *k, |v| v.to_string())
                    }),
                ),
            ),
        ),
    )
}
