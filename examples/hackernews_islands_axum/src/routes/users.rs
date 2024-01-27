use crate::api::{self, User};
use tachy_route::route::MatchedRoute;
use tachys::{prelude::*, tachydom::view::either::Either};

pub fn User(matched: MatchedRoute) -> impl RenderHtml<Dom> {
    // There's no actual way to navigate from a User to another User,
    // so we're going to do non-reactive accesses here
    let id = matched.param("id").unwrap_or_default().to_owned();
    let user = async move {
        if id.is_empty() {
            None
        } else {
            api::fetch_api::<User>(&api::user(&id)).await
        }
    };
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
    }.suspend();
    view! {
        <div class="user-view">{user_view}</div>
    }
}
