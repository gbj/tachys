pub mod prelude {
    pub use tachy_maccy::{component, view};
    pub use tachy_reaccy::prelude::*;
    pub use tachydom::prelude::*;
}

pub extern crate self as tachys;

pub mod children; // TODO fix children
pub mod component;
mod for_loop;
mod show;
pub use for_loop::*;
pub use show::*;
pub use tachy_maccy::*;
pub use tachy_reaccy;
pub use tachydom;
#[doc(hidden)]
pub use typed_builder;
#[doc(hidden)]
pub use typed_builder_macro;
