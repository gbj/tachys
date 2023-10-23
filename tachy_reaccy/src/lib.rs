mod arena;
pub mod context;
mod effect;
mod render_effect;
#[cfg(feature = "serde")]
mod serde;
mod signal;
pub mod signal_traits;
mod waker;
use crate::waker::MaybeWaker;
pub use arena::{global_root, Root};
pub use effect::*;
use lazy_static::lazy_static;
use parking_lot::RwLock;
pub use render_effect::*;
pub use signal::*;

pub mod prelude {
    pub use crate::signal_traits::*;
}

lazy_static! {
    static ref OBSERVER: RwLock<Option<MaybeWaker>> = RwLock::new(None);
}

pub(crate) struct Observer {}

impl Observer {
    fn get() -> Option<MaybeWaker> {
        OBSERVER.read().clone()
    }
}

pub fn log(s: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(s));
}
