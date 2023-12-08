use crate::api;
use tachy_route::reactive::ReactiveMatchedRoute;
//use leptos_meta::*;
//use leptos_router::*;
use tachys::{prelude::*, tachydom::view::either::Either};

pub fn Story(matched: &ReactiveMatchedRoute) -> impl RenderHtml<Dom> {
    let id = matched.param("id");
    let story = AsyncDerived::new_unsync(move || {
        let id = id.get().unwrap_or_default();
        async move {
            if id.is_empty() {
                None
            } else {
                api::fetch_api::<api::Story>(&api::story(&format!("item/{id}")))
                    .await
            }
        }
    });
    /* let meta_description = move || {
        story
            .get()
            .and_then(|story| story.map(|story| story.title))
            .unwrap_or_else(|| "Loading story...".to_string())
    }; */

    move || {
        async move {
        match story.await {
            None => Either::Left(view! {  <div class="item-view">"Error loading this story."</div> }),
            Some(story) => Either::Right(view! {
                    <div class="item-view">
                        <div class="item-view-header">
                        <a href=story.url target="_blank">
                            <h1>{story.title}</h1>
                        </a>
                        <span class="host">
                            "("{story.domain}")"
                        </span>
                        {story.user.map(|user| view! {  <p class="meta">
                            {story.points}
                            " points | by "
                            <a href=format!("/users/{user}")>{user.clone()}</a>
                            {format!(" {}", story.time_ago)}
                        </p>})}
                        </div>
                        <div class="item-view-comments">
                        <p class="item-view-comments-header">
                            {if story.comments_count.unwrap_or_default() > 0 {
                                format!("{} comments", story.comments_count.unwrap_or_default())
                            } else {
                                "No comments yet.".into()
                            }}
                        </p>
                        <ul class="comment-children">
                            /* <For
                                each=move || story.comments.clone().unwrap_or_default()
                                key=|comment| comment.id
                                let:comment
                            >
                                <Comment comment />
                            </For> */
                        </ul>
                    </div>
                </div>
            })
        }
    }.suspend().with_fallback("Loading...")
    }
}

#[component]
pub fn Comment(comment: api::Comment) -> impl RenderHtml<Dom> {
    let open = RwSignal::new(true);

    view! {
        <li class="comment">
        <div class="by">
            <a href=format!("/users/{}", comment.user.clone().unwrap_or_default())>{comment.user.clone()}</a>
            {format!(" {}", comment.time_ago)}
        </div>
        <div class="text" inner_html=comment.content.unwrap_or_default()></div>
        {(!comment.comments.is_empty()).then(|| {
            view! {
                <div>
                    <div class="toggle" class:open=move || open.get()>
                        <a on:click=move |_| open.update(|n| *n = !*n)>
                            {
                                let comments_len = comment.comments.len();
                                move || if open.get() {
                                    "[-]".into()
                                } else {
                                    format!("[+] {}{} collapsed", comments_len, pluralize(comments_len))
                                }
                            }
                        </a>
                    </div>
                    {move || open.get().then({
                        let comments = comment.comments.clone();
                        move || view! {
                            <ul class="comment-children">
                                {comments.into_iter().map(|comment| {
                                    "nested"
                                    //view! { <Comment comment /> }
                                }).collect::<Vec<_>>()}
                            </ul>
                        }
                    })}
                </div>
            }
        })}
        </li>
    }
}

fn pluralize(n: usize) -> &'static str {
    if n == 1 {
        " reply"
    } else {
        " replies"
    }
}
