use async_recursion::async_recursion;
use futures::channel::mpsc::UnboundedSender;
use std::{borrow::Cow, collections::VecDeque, future::Future, pin::Pin};

pub enum StreamChunk {
    Sync(Cow<'static, str>),
    Async {
        chunks:
            Pin<Box<dyn Future<Output = VecDeque<StreamChunk>> + Send + Sync>>,
        should_block: bool,
    },
}

#[async_recursion]
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
}
