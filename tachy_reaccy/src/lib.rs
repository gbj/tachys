mod arena;
pub mod context;
pub mod effect;
pub mod render_effect;
#[cfg(feature = "serde")]
mod serde;
pub mod signal;
pub mod signal_traits;
pub mod spawn;
mod waker;
pub use arena::{global_root, Root};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::sync::Arc;
use waker::Notifier;

pub mod prelude {
    pub use crate::{
        context::{provide_context, use_context},
        effect::Effect,
        global_root,
        signal::{ArcSignal, Signal},
        signal_traits::*,
        Root,
    };
}

lazy_static! {
    static ref OBSERVER: RwLock<Option<Notifier>> = RwLock::new(None);
}

pub(crate) struct Observer {}

impl Observer {
    fn get() -> Option<Notifier> {
        OBSERVER.read().clone()
    }
}

pub(crate) type Queue<T> = Arc<RwLock<Vec<T>>>;

#[cfg(feature = "web")]
pub fn log(s: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(s));
}

#[cfg(not(feature = "web"))]
pub fn log(s: &str) {
    println!("{s}");
}
