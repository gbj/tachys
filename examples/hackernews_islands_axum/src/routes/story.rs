use crate::api;
use tachy_route::{reactive::ReactiveMatchedRoute, route::MatchedRoute};
//use leptos_meta::*;
//use leptos_router::*;
use tachys::{
    children::Children,
    island,
    prelude::*,
    tachydom::view::{any_view::IntoAny, either::Either},
};

pub fn Story(route: MatchedRoute) -> impl RenderHtml<Dom> {
    let mut path = String::from("item/");
    let id = route.param("id").unwrap_or_default();
    let id_is_empty = id.is_empty();
    path.push_str(id);
    let story = async move {
        if id_is_empty {
            None
        } else {
            api::fetch_api::<api::Story>(&api::story(&path)).await
        }
    };

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
                            {story.comments
                                .into_iter()
                                .flatten()
                                .map(|comment| view! { <Comment comment/> })
                                .collect::<Vec<_>>()
                            }
                        </ul>
                    </div>
                </div>
            })
        }
    }.suspend()
}

#[island]
pub fn Toggle(children: Children) -> impl RenderHtml<Dom> {
    let open = ArcRwSignal::new(true);
    view! {
        <div class="toggle" class:open={
            let open = open.clone();
            move || open.get()
        }>
            <a
                on:click={
                    let open = open.clone();
                    move |_| open.update(|n| *n = !*n)
                }
            >
                {let open = open.clone();
                move || if open.get() {
                    "[-]"
                } else {
                    "[+] comments collapsed"
                }}
            </a>
        </div>
        <ul
            class="comment-children"
            style:display=move || if open.get() {
                "block"
            } else {
                "none"
            }
        >
            {children()}
        </ul>
    }
}

#[component]
pub fn Comment(comment: api::Comment) -> impl RenderHtml<Dom> {
    view! {
        <li class="comment">
            <div class="by">
                <a href=format!("/users/{}", comment.user.clone().unwrap_or_default())>{comment.user.clone()}</a>
                {format!(" {}", comment.time_ago)}
            </div>
            <div class="text" inner_html=comment.content.unwrap_or_default()></div>
            {(!comment.comments.is_empty()).then(|| {
                view! {
                    <Toggle>
                        {comment.comments.into_iter()
                            .map(|comment: api::Comment| view! { <Comment comment /> })
                            .collect::<Vec<_>>()}
                    </Toggle>
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
