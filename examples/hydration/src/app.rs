use tachy_maccy::view;
use tachy_reaccy::{async_signal::AsyncState, prelude::*};
use tachydom::{
    async_views::FutureViewExt,
    dom::log,
    html::{attribute::global::OnAttribute, element::ElementChild},
    renderer::dom::Dom,
    view::RenderHtml,
};

#[cfg(feature = "ssr")]
async fn timer() {
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
}
#[cfg(not(feature = "ssr"))]
async fn timer() {}

pub fn my_app() -> impl RenderHtml<Dom> {
    let count = Signal::new(0);
    let value = Resource::new(|| async {
        timer().await;
        42
    });

    view! {
        <button
            on:click=move |_| {
                log("clicked");
                count.update(|n| *n += 1)
            }
            disabled={
                let value = value.clone();
                move || value.with(|state| matches!(state, AsyncState::Loading))
            }
        >
            {move || {
                let value = value.clone();
                async move { (count.get() + value.await).to_string() }
                    .suspend()
                    .with_fallback("Loading...")
                    .track()
            }}
        </button>
    }
}
