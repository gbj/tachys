//use async_recursion::async_recursion;
use futures::Stream;
use parking_lot::RwLock;
use std::{
    collections::VecDeque,
    fmt::Debug,
    future::Future,
    mem,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

#[derive(Clone, Debug, Default)]
pub struct StreamBuilder(Arc<RwLock<StreamBuilderInner>>);

type PinnedFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync>>;

impl StreamBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push_sync(&self, string: &str) {
        self.0.write().sync_buf.push_str(string);
    }

    pub fn push_async(
        &self,
        should_block: bool,
        fut: impl Future<Output = VecDeque<StreamChunk>> + Send + Sync + 'static,
    ) {
        // flush sync chunk
        let mut lock = self.0.write();
        let sync = mem::take(&mut lock.sync_buf);
        if !sync.is_empty() {
            lock.chunks.push_back(StreamChunk::Sync(sync));
        }
        lock.chunks.push_back(StreamChunk::Async {
            chunks: Box::pin(fut) as PinnedFuture<VecDeque<StreamChunk>>,
            should_block,
        });
    }

    pub fn with_buf(&self, fun: impl FnOnce(&mut String)) {
        fun(&mut self.0.write().sync_buf)
    }

    pub fn take_chunks(&self) -> VecDeque<StreamChunk> {
        let mut lock = self.0.write();
        let sync = mem::take(&mut lock.sync_buf);
        if !sync.is_empty() {
            lock.chunks.push_back(StreamChunk::Sync(sync));
        }
        mem::take(&mut lock.chunks)
    }
}

#[derive(Default)]
struct StreamBuilderInner {
    sync_buf: String,
    chunks: VecDeque<StreamChunk>,
    pending: Option<PinnedFuture<VecDeque<StreamChunk>>>,
}

impl Debug for StreamBuilderInner {
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
            Self::Async {
                chunks,
                should_block,
            } => f
                .debug_struct("Async")
                .field("should_block", should_block)
                .finish_non_exhaustive(),
        }
    }
}

impl Stream for StreamBuilder {
    type Item = String;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let pending = self.0.write().pending.take();
        if let Some(mut pending) = pending {
            match pending.as_mut().poll(cx) {
                Poll::Pending => {
                    self.0.write().pending = Some(pending);
                    Poll::Pending
                }
                Poll::Ready(chunks) => {
                    let mut lock = self.0.write();
                    for chunk in chunks {
                        lock.chunks.push_front(chunk);
                    }
                    drop(lock);
                    self.poll_next(cx)
                }
            }
        } else {
            let next_chunk = self.0.write().chunks.pop_front();
            match next_chunk {
                None => {
                    let sync_buf = mem::take(&mut self.0.write().sync_buf);
                    if sync_buf.is_empty() {
                        Poll::Ready(None)
                    } else {
                        Poll::Ready(Some(sync_buf))
                    }
                }
                Some(StreamChunk::Sync(value)) => Poll::Ready(Some(value)),
                Some(StreamChunk::Async {
                    mut chunks,
                    should_block,
                }) => {
                    self.0.write().pending = Some(chunks);
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
        /* let el: HtmlElement<Main, _, _, Dom> = main().child((
            p().child(("Hello, ", em().child("beautiful"), " world!")),
            async {
                sleep(Duration::from_millis(250)).await;
                p().child("Suspended")
            }
            .suspend(),
        )); */
        let el = async {
            sleep(Duration::from_millis(250)).await;
            "Suspended"
            //p().child("Suspended")
        }
        .suspend();
        let mut stream =
            <Suspend<false, _, _> as RenderHtml<Dom>>::to_html_stream_in_order(
                el,
            );

        /*         let html = stream.next().await.unwrap();
        assert_eq!(
            html,
            "<main><p>Hello, <em>beautiful</em> world!</p></main>"
        ); */

        let html = stream.next().await.unwrap();
        assert_eq!(html, "Suspended");
    }
}

/* #[async_recursion]
async fn handle_blocking_chunks(
    tx: UnboundedSender<String>,
    mut queued_chunks: VecDeque<StreamChunk>,
) -> VecDeque<StreamChunk> {
    let mut buffer = String::new();
    while let Some(chunk) = queued_chunks.pop_front() {
        match chunk {
            StreamChunk::Sync(sync) => buffer.push_str(&sync),
            StreamChunk::Async {
                chunks,
                should_block,
            } => {
                if should_block {
                    // add static HTML before the Suspense and stream it down
                    tx.unbounded_send(std::mem::take(&mut buffer))
                        .expect("failed to send async HTML chunk");

                    // send the inner stream
                    let suspended = chunks.await;
                    handle_blocking_chunks(tx.clone(), suspended).await;
                } else {
                    // TODO: should probably first check if there are any *other* blocking chunks
                    queued_chunks.push_front(StreamChunk::Async {
                        chunks,
                        should_block: false,
                    });
                    break;
                }
            }
        }
    }

    // send final sync chunk
    tx.unbounded_send(std::mem::take(&mut buffer))
        .expect("failed to send final HTML chunk");

    queued_chunks
}

#[async_recursion]
pub(crate) async fn handle_chunks(
    tx: UnboundedSender<String>,
    chunks: VecDeque<StreamChunk>,
) {
    let mut buffer = String::new();
    for chunk in chunks {
        match chunk {
            StreamChunk::Sync(sync) => buffer.push_str(&sync),
            StreamChunk::Async { chunks, .. } => {
                // add static HTML before the Suspense and stream it down
                tx.unbounded_send(std::mem::take(&mut buffer))
                    .expect("failed to send async HTML chunk");

                // send the inner stream
                let suspended = chunks.await;

                handle_chunks(tx.clone(), suspended).await;
            }
        }
    }
    // send final sync chunk
    tx.unbounded_send(std::mem::take(&mut buffer))
        .expect("failed to send final HTML chunk");
} */
