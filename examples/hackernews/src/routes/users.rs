use crate::api::{self, User};
use std::collections::HashMap;
use tachy_route::route::MatchedRoute;
use tachys::{
    prelude::*,
    tachydom::view::{
        any_view::IntoAny, either::Either, template::ViewTemplate,
    },
};

pub fn User(matched: MatchedRoute) -> impl RenderHtml<Dom> {
    let id = matched.param("id").map(|n| n.to_string());
    let id = id.unwrap_or_default();
    let user = send_wrapper::SendWrapper::new(async move {
        if id.is_empty() {
            None
        } else {
            api::fetch_api::<User>(&api::user(&id)).await
        }
    });
    let user_view = async move {
            match user.await {
                None => Either::Left(view! { <h1>"User not found."</h1> }),
                Some(user) => Either::Right(view! {
                    <div>
                        <h1>"User: " {user.id.clone()}</h1>
                        <ul class="meta">
                            <li>
                                <span class="label">"Created: "</span> {user.created}
                            </li>
                            <li>
                            <span class="label">"Karma: "</span> {user.karma}
                            </li>
                            <li inner_html={user.about.unwrap_or_default()} class="about"></li>
                        </ul>
                        <p class="links">
                            <a href=format!("https://news.ycombinator.com/submitted?id={}", user.id)>"submissions"</a>
                            " | "
                            <a href=format!("https://news.ycombinator.com/threads?id={}", user.id)>"comments"</a>
                        </p>
                    </div>
                }),
            }
        }
        .suspend()
        .with_fallback("Loading...");
    view! {
        <div class="user-view">{user_view}</div>
    }
}
