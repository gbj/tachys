use crate::api;
use tachy_reaccy::async_signal::AsyncState;
use tachy_route::reactive::ReactiveMatchedRoute;
use tachys::{prelude::*, tachydom::view::either::Either, Show};

fn category(from: &str) -> &'static str {
    match from {
        "new" => "newest",
        "show" => "show",
        "ask" => "ask",
        "job" => "jobs",
        _ => "news",
    }
}

pub fn Stories(matched: &ReactiveMatchedRoute) -> impl RenderHtml<Dom> {
    println!("Stories");
    let page = matched.search("page");
    let story_type = matched.param("stories");
    let page = move || {
        page.get()
            .and_then(|page| page.parse::<usize>().ok())
            .unwrap_or(1)
    };
    let story_type =
        move || story_type.get().unwrap_or_else(|| "top".to_string());
    let stories = AsyncDerived::new_unsync(move || {
        let page = page();
        let story_type = story_type();
        println!("starting to load stories");
        async move {
            
        println!("inside async to load stories");
            let path = format!("{}?page={}", category(&story_type), page);
            api::fetch_api::<Vec<api::Story>>(&api::story(&path)).await
        }
    });
    let pending = move || stories.with(AsyncState::loading);

    let hide_more_link = move || {
        stories
            .get()
            .current_value()
            .and_then(|value| value.as_ref().map(|value| value.len()))
            .unwrap_or_default()
            < 28
            || pending()
    };

    let stories = move || {
        async move {
            println!("Loading stories here");
            match stories.await {
                None => Either::Left(view! { <p>"Error loading stories."</p> }),
                Some(stories) => Either::Right(view! {
                    <ul>
                        {stories.into_iter().map(|story| {
                            view! { <Story story /> }
                        }).collect::<Vec<_>>()}
                    </ul>
                }),
            }
        }
        .suspend()
        .track()
        .transition()
        .with_fallback("Loading...")
    };

    view! {
        <div class="news-view">
            <div class="news-list-nav">
                <span>
                    <Show when=move || { page() > 1 }
                        fallback=|| view! {
                            <span class="page-link disabled" aria-hidden="true">
                                "< prev"
                            </span>
                        }
                    >
                        <a class="page-link"
                            href=move || format!("/{}?page={}", story_type(), page() - 1)
                            aria-label="Previous Page"
                        >
                            "< prev"
                        </a>
                    </Show>
                </span>
                <span>"page " {page}</span>
                <span class="page-link"
                    class:disabled=hide_more_link
                    aria-hidden=hide_more_link
                >
                    <a href=move || format!("/{}?page={}", story_type(), page() + 1)
                        aria-label="Next Page"
                    >
                        "more >"
                    </a>
                </span>
            </div>
            <main class="news-list">
                <div>
                    {stories}
                </div>
            </main>
        </div>
    }
}

#[component]
fn Story(story: api::Story) -> impl RenderHtml<Dom> {
    view! {
         <li class="news-item">
            <span class="score">{story.points}</span>
            <span class="title">
                {if !story.url.starts_with("item?id=") {
                    Either::Left(view! {
                        <span>
                            <a href=story.url target="_blank" rel="noreferrer">
                                {story.title.clone()}
                            </a>
                            <span class="host">"("{story.domain}")"</span>
                        </span>
                    })
                } else {
                    let title = story.title.clone();
                    // TODO <A>
                    Either::Right(view! { <a href=format!("/stories/{}", story.id)>{title.clone()}</a> })
                }}
            </span>
            <br />
            <span class="meta">
                {if story.story_type != "job" {
                    Either::Left(view! {
                        <span>
                            {"by "}
                            {story.user.map(|user| view ! { <a href=format!("/users/{user}")>{user.clone()}</a>})}
                            {format!(" {} | ", story.time_ago)}
                            <a href=format!("/stories/{}", story.id)>
                                {if story.comments_count.unwrap_or_default() > 0 {
                                    format!("{} comments", story.comments_count.unwrap_or_default())
                                } else {
                                    "discuss".into()
                                }}
                            </a>
                        </span>
                    })
                } else {
                    let title = story.title.clone();
                    Either::Right(view! {<a href=format!("/item/{}", story.id)>{title.clone()}</a>})
                }}
            </span>
            {(story.story_type != "link").then(|| view! {
                " "
                <span class="label">{story.story_type}</span>
            })}
        </li>
    }
}
