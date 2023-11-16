use tachy_maccy::view;
use tachy_reaccy::prelude::*;
use tachydom::{
    dom::log,
    html::{attribute::global::OnAttribute, element::ElementChild},
    renderer::dom::Dom,
    view::RenderHtml,
};

pub fn my_app() -> impl RenderHtml<Dom> {
    let count = Signal::new(0);
    view! {
        <button
            on:click=move |_| {
                log("clicked");
                count.update(|n| *n += 1)
            }
        >
            {move || count.get().to_string()}
        </button>
    }
}
