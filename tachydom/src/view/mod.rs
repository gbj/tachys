use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use crate::hydration::Cursor;
use web_sys::{HtmlElement, Node};

pub mod any_view;
pub mod dynamic;
pub mod iterators;
#[cfg(feature = "nightly")]
pub mod static_types;
pub mod strings;
pub mod template;
pub mod tuples;

/// Allows data to be rendered to the UI, as either HTML or DOM nodes.
pub trait Render {
    /// The “view state” for this type, which can be retained between updates.
    ///
    /// For example, for a text node, `State` might be the actual DOM text node
    /// and the previous string, to allow for diffing between updates.
    type State: Mountable;

    /// Renders a view to HTML.
    fn to_html(&mut self, buf: &mut String, position: &PositionState);

    /// Makes a set of DOM nodes rendered from HTML interactive. If `FROM_SERVER` is
    /// `true`, this HTML was rendered on the server. If `FROM_SERVER` is `false`, the
    /// HTML was in a client-side `<template>` element.
    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State;

    fn hydrate_from<const FROM_SERVER: bool>(self, el: &HtmlElement) -> Self::State
    where
        Self: Sized,
    {
        let cursor = Cursor::new(el.clone());
        let position = PositionState::default();
        self.hydrate::<FROM_SERVER>(&cursor, &position)
    }

    /// Creates the view for the first time, without hydrating from existing HTML.
    fn build(self) -> Self::State;

    /// Updates the view with new data.
    fn rebuild(self, state: &mut Self::State);
}

/// Allows a type to be mounted to the DOM.
pub trait Mountable {
    /// Detaches the view from the DOM.
    fn unmount(&mut self);

    /// Returns a node that can be mounted anywhere in the DOM.
    fn as_mountable(&self) -> Option<Node>;
}

impl<T> Mountable for Option<T>
where
    T: Mountable,
{
    fn unmount(&mut self) {
        if let Some(ref mut mounted) = self {
            mounted.unmount()
        }
    }

    fn as_mountable(&self) -> Option<Node> {
        self.as_ref().and_then(Mountable::as_mountable)
    }
}

/// Allows data to be added to a static template.
pub trait ToTemplate {
    /// Renders a view type to a template. This does not take actual view data,
    /// but can be used for constructing part of an HTML `<template>` that corresponds
    /// to a view of a particular type.
    fn to_template(buf: &mut String, position: &mut Position);
}

#[derive(Debug, Default, Clone)]
pub struct PositionState(Rc<Cell<Position>>);

impl PositionState {
    pub fn new(position: Position) -> Self {
        Self(Rc::new(Cell::new(position)))
    }

    pub fn set(&self, position: Position) {
        self.0.set(position);
    }

    pub fn get(&self) -> Position {
        self.0.get()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum Position {
    #[default]
    FirstChild,
    NextChild,
    OnlyChild,
    LastChild,
}
