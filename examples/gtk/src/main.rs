use gtk::{prelude::*, Application, ApplicationWindow, Button, Orientation};
use leptos_reactive::*;
use tachydom::view::{strings::StrState, Mountable, Render};
use tachygtk::{button, r#box, Element, ElementState, TachyGtk};
mod tachygtk;

const APP_ID: &str = "dev.leptos.Counter";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    let _ = create_runtime();

    let value = RwSignal::new(0);
    let view = r#box(
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
    );
    let state: ElementState<_> = view.build();

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("TachyGTK")
        .child(&state.0 .0)
        .build();

    // Present window
    window.present();
}
