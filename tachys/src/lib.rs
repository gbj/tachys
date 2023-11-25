pub mod prelude {
    pub use tachy_maccy::{component, view};
    pub use tachy_reaccy::prelude::*;
    pub use tachydom::prelude::*;
}

// pub mod children; // TODO fix children
pub mod component;

pub use tachy_maccy::*;
pub use tachy_reaccy;
pub use tachydom;
#[doc(hidden)]
pub use typed_builder;
#[doc(hidden)]
pub use typed_builder_macro;
