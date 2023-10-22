//! This crate exists to serve a very specific purpose.
//!
//! ## The Problem
//!
//! `wasm-bindgen` provides the essential glue code to interact with the
//! browser and Web APIs, including the DOM. All interactions with the browser
//! ultimately involve some use of the `JsValue` type, which represents indices
//! into a stack held on the JS side. This type is [explicitly marked as `!Send`]
//! (https://github.com/rustwasm/wasm-bindgen/blob/2e9ff5dfa3f11415f0efe9b946ee2734500e9ee3/src/lib.rs#L93).
//! It’s also impossible to construct outside the browser; it simply panics, which
//! makes sense.
//!
//! When building a frontend web framework using Rust, this makes it hard to share
//! types between the server rendering HTML and the client code compiled to WASM and running
//! in the browser.
//!
//! For example, the Leptos web framework uses a reactive system, in which some reactive
//! side effects run in the server, and some only run in the browser. These are
//! shared in the same data structure, which means that structure is `!Send`, because
//! those browser-only effects need to hold some `JsValue`s (for example, an HTML element.)
//! However, some web frameworks in turn (like Axum) do not allow `!Send` handlers, so
//! we need special workarounds to spawn Tokio tasks pinned to a specific thread.
//!
//! In practice, there’s no actual problem here: the `!Send` types can’t even be constructed
//! on the server, which will panic in any case, because there’s no browser present.
//!
//! ## The Solution
//!
//! This crate introduces a single type, [`BrowserOnly`], with the following characteristics:
//! 1. [`BrowserOnly::new`] can only be called in the single-threaded browser environment,
//!    defined as “in a `wasm32` architecture without atomics.” It panics in any other setting.
//! 2. `BrowserOnly` unsafely implements `Send` and `Sync`.
//!
//! Because `BrowserOnly` can only be created in a single-threaded environment, it can never
//! be unsafely sent between threads or accessed from multiple threads, allowing us to wrap
//! the `JsValue` type (or other structures that might contain it) and store it in an otherwise-
//! `Send` data structure without making that structure `!Send`.
//!
//! I can’t see how this would be unsound, but please let me know if it’s not!

use std::ops::{Deref, DerefMut};

/// Creates a `Send + Sync` wrapper for any value, which can only be created in a
/// single-threaded browser environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BrowserOnly<T>(T);

impl<T> BrowserOnly<T> {
    /// Wraps a value to make it `Send + Sync`.
    ///
    /// ## Panics
    /// Panics if called outside the browser, as defined by
    /// ```rust
    /// #[cfg(not(all(
    ///     target_arch = "wasm32",
    ///     not(target_feature = "atomics")
    /// )))]
    /// ```
    pub fn new(value: T) -> Self {
        #[cfg(all(target_arch = "wasm32", not(target_feature = "atomics")))]
        {
            Self(value)
        }
        #[cfg(not(all(
            target_arch = "wasm32",
            not(target_feature = "atomics")
        )))]
        {
            _ = value;
            panic!(
                "BrowserOnly can only be constructed in the browser's \
                 single-threaded environment."
            )
        }
    }

    /// Consumes the wrapper, returning the inner type.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for BrowserOnly<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for BrowserOnly<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// SAFETY: BrowserOnlyWaker can only be constructed in the browser
// the browser is always single-threaded
unsafe impl<T> Send for BrowserOnly<T> {}
unsafe impl<T> Sync for BrowserOnly<T> {}
