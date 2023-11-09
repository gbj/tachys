use futures::Stream;
use std::{
    collections::VecDeque,
    fmt::Debug,
    future::Future,
    mem,
    pin::Pin,
    task::{Context, Poll},
};

#[derive(Default)]
pub struct StreamBuilder {
    sync_buf: String,
    chunks: VecDeque<StreamChunk>,
    pending: Option<PinnedFuture<VecDeque<StreamChunk>>>,
}

type PinnedFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync>>;

impl StreamBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push_sync(&mut self, string: &str) {
        self.sync_buf.push_str(string);
    }

    pub fn push_async(
        &mut self,
        should_block: bool,
        fut: impl Future<Output = VecDeque<StreamChunk>> + Send + Sync + 'static,
    ) {
        // flush sync chunk
        let sync = mem::take(&mut self.sync_buf);
        if !sync.is_empty() {
            self.chunks.push_back(StreamChunk::Sync(sync));
        }
        self.chunks.push_back(StreamChunk::Async {
            chunks: Box::pin(fut) as PinnedFuture<VecDeque<StreamChunk>>,
            should_block,
        });
    }

    pub fn with_buf(&mut self, fun: impl FnOnce(&mut String)) {
        fun(&mut self.sync_buf)
    }

    pub fn take_chunks(&mut self) -> VecDeque<StreamChunk> {
        let sync = mem::take(&mut self.sync_buf);
        if !sync.is_empty() {
            self.chunks.push_back(StreamChunk::Sync(sync));
        }
        mem::take(&mut self.chunks)
    }

    pub fn finish(mut self) -> Self {
        let sync_buf_remaining = mem::take(&mut self.sync_buf);
        if sync_buf_remaining.is_empty() {
            return self;
        } else if let Some(StreamChunk::Sync(buf)) = self.chunks.back_mut() {
            buf.push_str(&sync_buf_remaining);
        } else {
            self.chunks.push_back(StreamChunk::Sync(sync_buf_remaining));
        }
        self
    }
}

impl Debug for StreamBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamBuilderInner")
            .field("sync_buf", &self.sync_buf)
            .field("chunks", &self.chunks)
            .field("pending", &self.pending.is_some())
            .finish()
    }
}

pub enum StreamChunk {
    Sync(String),
    Async {
        chunks: PinnedFuture<VecDeque<StreamChunk>>,
        should_block: bool,
    },
}

impl Debug for StreamChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sync(arg0) => f.debug_tuple("Sync").field(arg0).finish(),
            Self::Async { should_block, .. } => f
                .debug_struct("Async")
                .field("should_block", should_block)
                .finish_non_exhaustive(),
        }
    }
}

impl Stream for StreamBuilder {
    type Item = String;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let mut this = self.as_mut();
        let pending = this.pending.take();
        if let Some(mut pending) = pending {
            match pending.as_mut().poll(cx) {
                Poll::Pending => {
                    this.pending = Some(pending);
                    Poll::Pending
                }
                Poll::Ready(chunks) => {
                    for chunk in chunks.into_iter().rev() {
                        this.chunks.push_front(chunk);
                    }
                    self.poll_next(cx)
                }
            }
        } else {
            let next_chunk = this.chunks.pop_front();
            match next_chunk {
                None => {
                    let sync_buf = mem::take(&mut this.sync_buf);
                    if sync_buf.is_empty() {
                        Poll::Ready(None)
                    } else {
                        Poll::Ready(Some(sync_buf))
                    }
                }
                Some(StreamChunk::Sync(mut value)) => {
                    loop {
                        match this.chunks.pop_front() {
                            None => break,
                            Some(StreamChunk::Async {
                                chunks,
                                should_block,
                            }) => {
                                this.chunks.push_front(StreamChunk::Async {
                                    chunks,
                                    should_block,
                                });
                                break;
                            }
                            Some(StreamChunk::Sync(next)) => {
                                value.push_str(&next);
                            }
                        }
                    }

                    let sync_buf = mem::take(&mut this.sync_buf);
                    value.push_str(&sync_buf);
                    Poll::Ready(Some(value))
                }
                Some(StreamChunk::Async {
                    chunks,
                    should_block,
                }) => {
                    this.pending = Some(chunks);
                    self.poll_next(cx)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        async_views::{FutureViewExt, Suspend},
        html::element::{em, main, p, ElementChild, HtmlElement, Main},
        renderer::dom::Dom,
        view::RenderHtml,
    };
    use futures::StreamExt;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn stream_of_sync_content_ready_immediately() {
        let el: HtmlElement<Main, _, _, Dom> = main().child(p().child((
            "Hello, ",
            em().child("beautiful"),
            " world!",
        )));
        let mut stream = el.to_html_stream_in_order();

        let html = stream.next().await.unwrap();
        assert_eq!(
            html,
            "<main><p>Hello, <em>beautiful</em> world!</p></main>"
        );
    }

    #[tokio::test]
    async fn single_async_block_in_stream() {
        let el = async {
            sleep(Duration::from_millis(250)).await;
            "Suspended"
        }
        .suspend();
        let mut stream =
            <Suspend<false, _, _> as RenderHtml<Dom>>::to_html_stream_in_order(
                el,
            );

        let html = stream.next().await.unwrap();
        assert_eq!(html, "Suspended");
    }

    #[tokio::test]
    async fn async_with_siblings_in_stream() {
        let el = (
            "Before Suspense",
            async {
                sleep(Duration::from_millis(250)).await;
                "Suspended"
            }
            .suspend(),
        );
        let mut stream =
            <(&str, Suspend<false, _, _>) as RenderHtml<Dom>>::to_html_stream_in_order(
                el,
            );

        assert_eq!(stream.next().await.unwrap(), "Before Suspense");
        assert_eq!(stream.next().await.unwrap(), "<!>Suspended");
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn async_inside_element_in_stream() {
        let el: HtmlElement<_, _, _, Dom> = p().child((
            "Before Suspense",
            async {
                sleep(Duration::from_millis(250)).await;
                "Suspended"
            }
            .suspend(),
        ));
        let mut stream = el.to_html_stream_in_order();

        assert_eq!(stream.next().await.unwrap(), "<p>Before Suspense");
        assert_eq!(stream.next().await.unwrap(), "<!>Suspended</p>");
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn nested_async_blocks() {
        let el: HtmlElement<_, _, _, Dom> = main().child((
            "Before Suspense",
            async {
                sleep(Duration::from_millis(250)).await;
                p().child((
                    "Before inner Suspense",
                    async {
                        sleep(Duration::from_millis(250)).await;
                        "Inner Suspense"
                    }
                    .suspend(),
                ))
            }
            .suspend(),
        ));
        let mut stream = el.to_html_stream_in_order();

        assert_eq!(stream.next().await.unwrap(), "<main>Before Suspense");
        assert_eq!(stream.next().await.unwrap(), "<p>Before inner Suspense");
        assert_eq!(
            stream.next().await.unwrap(),
            "<!>Inner Suspense</p></main>"
        );
    }
}
