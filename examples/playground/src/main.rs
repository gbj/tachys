use tachys::{
    prelude::*,
    show::Show,
    tachy_reaccy::render_effect::RenderEffect,
    tachydom::{
        dom::{body, event_target_value},
        html::{
            element::{p, HtmlElement, Input},
            event,
        },
        log,
        node_ref::NodeRef,
        view::error_boundary::Try,
    },
};
use tracing_subscriber::prelude::*;

pub fn app() -> impl Render<Dom> {
    let value = RwSignal::new("123".to_string());
    view! {
        <Show when=move || value.get() == "123">
            <p>"Got it!"</p>
        </Show>
    }
    /* (
        view! {
            <math mathcolor="red">
                <mrow>
                    <msup>
                    <mi>a</mi>
                    <mn>2</mn>
                    </msup>
                    <mo>+</mo>
                    <msup>
                    <mi>b</mi>
                    <mn>2</mn>
                    </msup>
                    <mo>=</mo>
                    <msup>
                    <mi>c</mi>
                    <mn>2</mn>
                    </msup>
                </mrow>
            </math>
            <svg height="150" width="500">
                <ellipse cx="240" cy="100" rx="220" ry="30" style="fill:purple" />
                <ellipse cx="220" cy="70" rx="190" ry="20" style="fill:lime" />
                <ellipse cx="210" cy="45" rx="170" ry="15" style="fill:yellow" />
            </svg>
            <custom-element custom-attribute=move || value.get()/>
            <input
                node_ref=&mut el
                on:input=move |ev| {
                    value.set(event_target_value(&ev))
                }
                prop:value=move || value.get()
                data-something="test"
                hx-get="bar"
                id="test"
            />
        },
        // this version uses a TryCatchError extension trait that lets us
        // .catch() an Err on any view
        move || {
            view! {
                <pre>
                    "f32: " {value.get().parse::<f32>()} "\n"
                    "u32: " {value.get().parse::<u32>()}
                </pre>
            }
            .catch(|err| {
                view! {
                    <pre style="border: 1px solid red; color: red">
                        "error"
                        //{err.to_string()}
                    </pre>
                }
            })
        },
        // however, note that it breaks if we make the errors more
        // fine-grained
        // the one above is doing a lightweight diff, but it's still a diff
        move || {
            view! {
                <pre>
                    "f32: " {move || value.get().parse::<f32>()} "\n"
                    "u32: " {move || value.get().parse::<u32>()}
                </pre>
            }
            .catch(|err| {
                view! {
                    <pre style="border: 1px solid red; color: red">
                        "error"
                        //{err.to_string()}
                    </pre>
                }
            })
        },
    ) */
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
