use crate::api::{self, User};
use std::collections::HashMap;
use tachys::{
    prelude::*,
    tachydom::view::{
        any_view::IntoAny, either::Either, template::ViewTemplate,
    },
};

#[component]
pub fn User() -> impl RenderHtml<Dom> {
    let params = RwSignal::new({
        // TODO route params
        let mut map = HashMap::new();
        map.insert("id", "1234");
        map
    }); // use_params_map();
    let user = AsyncDerived::new(move || {
        let id = params.get().get("id").cloned().unwrap_or_default();
        send_wrapper::SendWrapper::new(async move {
            if id.is_empty() {
                None
            } else {
                api::fetch_api::<User>(&api::user(id)).await
            }
        })
    });
    let user_view = move || {
        async move {
            match user.await {
                None => view! { <h1>"User not found."</h1> }.into_any(),
                Some(user) => view! {
                    <div>
                        <h1>"User: " {user.id.clone()}</h1>
                        <ul class="meta">
                            <li>
                                <span class="label">"Created: "</span> {user.created}
                            </li>
                            <li>
                            <span class="label">"Karma: "</span> {user.karma}
                            </li>
                            // TODO inner_html
                            <li /* inner_html={user.about} */ class="about"></li>
                        </ul>
                        <p class="links">
                            <a href=format!("https://news.ycombinator.com/submitted?id={}", user.id)>"submissions"</a>
                            " | "
                            <a href=format!("https://news.ycombinator.com/threads?id={}", user.id)>"comments"</a>
                        </p>
                    </div>
                }.into_any(),
            }
        }
        .suspend()
        .with_fallback("Loading...")
    };
    view! {
        <div class="user-view">{user_view}</div>
    }
}
