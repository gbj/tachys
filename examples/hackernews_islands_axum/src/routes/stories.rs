use crate::api;
use tachy_route::route::MatchedRoute;
use tachys::{
    prelude::*,
    tachydom::view::{any_view::IntoAny, either::Either},
};

fn category(from: &str) -> &'static str {
    match from {
        "new" => "newest",
        "show" => "show",
        "ask" => "ask",
        "job" => "jobs",
        _ => "news",
    }
}

pub fn Stories(matched: MatchedRoute) -> impl RenderHtml<Dom> {
    let page = matched.search("page");
    let story_type = matched.param("stories");
    let page = page
        .and_then(|page| page.parse::<usize>().ok())
        .unwrap_or(1);
    let story_type = story_type.unwrap_or("top");
    let stories = {
        let story_type = story_type.to_string();
        async move {
            let path = format!("{}?page={}", category(&story_type), page);
            api::fetch_api::<Vec<api::Story>>(&api::story(&path)).await
        }
    };

    let stories = async move {
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
    .with_fallback("Loading...");

    view! {
        <div class="news-view">
            <div class="news-list-nav">
                <span>
                    {if page == 1 {
                        view! {
                            <span class="page-link disabled" aria-hidden="true">
                                "< prev"
                            </span>
                        }.into_any()
                    } else {
                        view! {
                            <a class="page-link"
                                href=format!("/{}?page={}", story_type, page - 1)
                                aria-label="Previous Page"
                            >
                                "< prev"
                            </a>
                        }.into_any()
                    }}
                </span>
                <span>"page " {page}</span>
                <span class="page-link" >
                    <a href=format!("/{}?page={}", story_type, page + 1)
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
