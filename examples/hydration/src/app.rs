use std::{future::IntoFuture, rc::Rc, sync::Arc};
use tachy_maccy::view;
use tachy_reaccy::{
    async_signal::AsyncState,
    prelude::*,
    serialization::{Miniserde, SerdeJson, SerdeLite, Str},
};
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
        "str".to_string()
    });
    let value2 = Resource::serde(|| async {
        timer().await;
        "serde_json".to_string()
    });
    let value3 = Resource::miniserde(|| async {
        timer().await;
        "miniserde".to_string()
    });
    let value4 = Resource::serde_lite(|| async {
        timer().await;
        "serde_lite".to_string()
    });
    let value5 = Resource::rkyv(|| async {
        timer().await;
        "serde_lite".to_string()
    });

    let a: &str = "hello world";
    let b: Rc<str> = Rc::from(a);
    let c: Arc<str> = Arc::from(a);
    let d = String::from(a);

    /* view! {
        /* <button
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
        </button> */
        //{a} {b} {c} {d}
    } */
    (
        async { value.await }.suspend().with_fallback("FromStr..."),
        async { value2.await }
            .suspend()
            .with_fallback("serde_json..."),
        async { value3.await }
            .suspend()
            .with_fallback("miniserde..."),
        async { value4.await }
            .suspend()
            .with_fallback("serde_lite..."),
    )
}
