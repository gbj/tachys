use tachys::prelude::*;

#[component]
pub fn Nav() -> impl RenderHtml<Dom> {
    view! {
        <header class="header">
            <nav class="inner">
                // TODO use <A> component?
                <a href="/home">
                    <strong>"HN"</strong>
                </a>
                <a href="/new">
                    <strong>"New"</strong>
                </a>
                <a href="/show">
                    <strong>"Show"</strong>
                </a>
                <a href="/ask">
                    <strong>"Ask"</strong>
                </a>
                <a href="/job">
                    <strong>"Jobs"</strong>
                </a>
                <a class="github" href="http://github.com/leptos-rs/leptos" target="_blank" rel="noreferrer">
                    "Built with Leptos"
                </a>
            </nav>
        </header>
    }
}
