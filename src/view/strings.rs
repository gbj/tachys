use std::borrow::Cow;

use leptos_reactive::ReadSignal;

use super::{Mount, View};
use crate::dom::{Dom, Node};

impl<'a> View for &'a str {
    type State = (Self, Node);

    ////#[inline(always)]
    fn build(self) -> Self::State {
        let text = Dom::create_text_node(self);
        (self, text)
    }

    ////#[inline(always)]
    fn rebuild(self, state: &mut Self::State) {
        let (prev, node) = state;
        if &self != prev {
            node.set_data(self);
            state.0 = self;
        }
    }

    ////#[inline(always)]
    fn mount(state: &mut Self::State, parent: Mount) {
        parent.mount_node(state.1);
    }

    ////#[inline(always)]
    fn unmount(state: &mut Self::State) {
        state.1.remove();
    }
}

impl<'a> View for &'a String {
    type State = (Self, Node);

    ////#[inline(always)]
    fn build(self) -> Self::State {
        let text = Dom::create_text_node(self);
        (self, text)
    }

    ////#[inline(always)]
    fn rebuild(self, state: &mut Self::State) {
        let (prev, node) = state;
        if &self != prev {
            node.set_data(self);
            state.0 = self;
        }
    }

    ////#[inline(always)]
    fn mount(state: &mut Self::State, parent: Mount) {
        parent.mount_node(state.1);
    }

    ////#[inline(always)]
    fn unmount(state: &mut Self::State) {
        state.1.remove();
    }
}

impl View for String {
    type State = (Self, Node);

    ////#[inline(always)]
    fn build(self) -> Self::State {
        let text = Dom::create_text_node(&self);
        (self, text)
    }

    ////#[inline(always)]
    fn rebuild(self, state: &mut Self::State) {
        let (prev, node) = state;
        if &self != prev {
            node.set_data(&self);
            state.0 = self;
        }
    }

    ////#[inline(always)]
    fn mount(state: &mut Self::State, parent: Mount) {
        parent.mount_node(state.1);
    }

    ////#[inline(always)]
    fn unmount(state: &mut Self::State) {
        state.1.remove();
    }
}

impl View for Cow<'_, str> {
    type State = (Self, Node);

    ////#[inline(always)]
    fn build(self) -> Self::State {
        let text = Dom::create_text_node(&self);
        (self, text)
    }

    ////#[inline(always)]
    fn rebuild(self, state: &mut Self::State) {
        let (prev, node) = state;
        if &self != prev {
            node.set_data(&self);
            state.0 = self;
        }
    }

    ////#[inline(always)]
    fn mount(state: &mut Self::State, parent: Mount) {
        parent.mount_node(state.1);
    }

    ////#[inline(always)]
    fn unmount(state: &mut Self::State) {
        state.1.remove();
    }
}

macro_rules! viewable_primitive {
  ($($child_type:ty),* $(,)?) => {
    $(
      impl View for $child_type {
        type State = (Self, Node);

         ////#[inline(always)]
        fn build(self) -> Self::State {
            let string = self.to_string();
            let text = Dom::create_text_node(&string);
            (self, text)
        }

         ////#[inline(always)]
        fn rebuild(self, state: &mut Self::State) {
            let (prev, node) = state;
            if &self != prev {
                let text = self.to_string();
                node.set_data(&text);
                state.0 = self;
            }
        }

         ////#[inline(always)]
        fn mount(state: &mut Self::State, parent: Mount) {
            parent.mount_node(state.1);
        }

         ////#[inline(always)]
        fn unmount(state: &mut Self::State) {
            state.1.remove();
        }
      }
    )*
  };
}

viewable_primitive![
    usize,
    u8,
    u16,
    u32,
    u64,
    u128,
    isize,
    i8,
    i16,
    i32,
    i64,
    i128,
    f32,
    f64,
    char,
    bool,
    //Cow<'_, str>,
    std::net::IpAddr,
    std::net::SocketAddr,
    std::net::SocketAddrV4,
    std::net::SocketAddrV6,
    std::net::Ipv4Addr,
    std::net::Ipv6Addr,
    std::num::NonZeroI8,
    std::num::NonZeroU8,
    std::num::NonZeroI16,
    std::num::NonZeroU16,
    std::num::NonZeroI32,
    std::num::NonZeroU32,
    std::num::NonZeroI64,
    std::num::NonZeroU64,
    std::num::NonZeroI128,
    std::num::NonZeroU128,
    std::num::NonZeroIsize,
    std::num::NonZeroUsize,
    std::panic::Location<'_>,
];
