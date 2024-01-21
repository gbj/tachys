use cfg_if::cfg_if;
use std::future::Future;

pub fn spawn_local<F>(fut: F)
where
    F: Future<Output = ()> + 'static,
{
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            wasm_bindgen_futures::spawn_local(fut)
        } else if #[cfg(feature = "glib")] {
            let main_context = glib::MainContext::default();
            main_context.spawn_local(fut);
        } else if #[cfg(any(test, doctest, feature = "tokio"))] {
            tokio::task::spawn_local(fut);
        }  else {
            use std::cell::OnceCell;
            use futures::executor::LocalPool;
            use futures::task::LocalSpawnExt;
            thread_local! {
                static POOL: OnceCell<LocalPool> = OnceCell::new()
            }
            POOL.with(|pool| pool.get_or_init(|| LocalPool::new()).spawner().spawn_local(fut));
        }
    }
}

pub fn spawn<F>(fut: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            wasm_bindgen_futures::spawn_local(fut)
        } else if #[cfg(feature = "glib")] {
            let main_context = glib::MainContext::default();
            main_context.spawn(fut);
        } else if #[cfg(any(test, doctest, feature = "tokio"))] {
            tokio::task::spawn(fut);
        }  else {
            spawn_local(fut);
        }
    }
}
