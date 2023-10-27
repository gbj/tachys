#![feature(return_position_impl_trait_in_trait)]

mod arena;
pub mod context;
pub mod effect;
pub mod memo;
mod notify;
pub mod render_effect;
//#[cfg(feature = "serde")]
//mod serde;
pub mod signal;
pub mod signal_traits;
mod source;
pub mod spawn;
use crate::source::AnySubscriber;
pub use arena::{global_root, Root};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::sync::Arc;

pub mod prelude {
    pub use crate::{
        context::{provide_context, use_context},
        effect::Effect,
        global_root,
        memo::{ArcMemo, Memo},
        signal::{ArcSignal, Signal},
        signal_traits::*,
        Root,
    };
}

lazy_static! {
    static ref OBSERVER: RwLock<Option<AnySubscriber>> = RwLock::new(None);
}

pub(crate) struct Observer {}

impl Observer {
    fn get() -> Option<AnySubscriber> {
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
