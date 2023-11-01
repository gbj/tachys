use crate::{hydration::Cursor, renderer::Renderer};
use std::{cell::Cell, rc::Rc};

pub mod any_view;
pub mod either;
pub mod iterators;
pub mod keyed;
#[cfg(feature = "nightly")]
pub mod static_types;
pub mod strings;
pub mod template;
pub mod tuples;

/// The `Render` trait allows rendering something as part of the user interface.
///
/// It is generic over the renderer itself, as long as that implements the [`Renderer`]
/// trait.
pub trait Render<R: Renderer> {
    /// The “view state” for this type, which can be retained between updates.
    ///
    /// For example, for a text node, `State` might be the actual DOM text node
    /// and the previous string, to allow for diffing between updates.
    type State: Mountable<R>;

    /// Creates the view for the first time, without hydrating from existing HTML.
    fn build(self) -> Self::State;

    /// Updates the view with new data.
    fn rebuild(self, state: &mut Self::State);
}

/// The `RenderHtml` trait allows rendering something to HTML, and transforming
/// that HTML into an interactive interface.
///
/// This process is traditionally called “server rendering” and “hydration.” As a
/// metaphor, this means that the structure of the view is created on the server, then
/// “dehydrated” to HTML, sent across the network, and “rehydrated” with interactivity
/// in the browser.
///
/// However, the same process can be done entirely in the browser: for example, a view
/// can be transformed into some HTML that is used to create a `<template>` node, which
/// can be cloned many times and “hydrated,” which is more efficient than creating the
/// whole view piece by piece.
pub trait RenderHtml<R: Renderer>
where
    Self: Render<R>,
    R::Node: Clone,
    R::Element: Clone,
{
    /// Renders a view to HTML.
    fn to_html(self, buf: &mut String, position: &PositionState);

    /// Makes a set of DOM nodes rendered from HTML interactive.
    ///
    /// If `FROM_SERVER` is `true`, this HTML was rendered using [`RenderHtml::to_html`]
    /// (e.g., during server-side rendering ).
    ///
    /// If `FROM_SERVER` is `false`, the HTML was rendered using [`ToTemplate::to_template`]
    /// (e.g., into a `<template>` element).
    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor<R>,
        position: &PositionState,
    ) -> Self::State;

    /// Hydrates using [`RenderHtml::hydrate`], beginning at the given element.
    fn hydrate_from<const FROM_SERVER: bool>(
        self,
        el: &R::Element,
    ) -> Self::State
    where
        Self: Sized,
    {
        let cursor = Cursor::new(el.clone());
        let position = PositionState::default();
        self.hydrate::<FROM_SERVER>(&cursor, &position)
    }
}

/// Allows a type to be mounted to the DOM.
pub trait Mountable<R: Renderer> {
    /// Detaches the view from the DOM.
    fn unmount(&mut self);

    /// Mounts a node to the interface.
    fn mount(&mut self, parent: &R::Element, marker: Option<&R::Node>);

    /// Inserts another `Mountable` type before this one. Returns `false` if
    /// this does not actually exist in the UI (for example, `()`).
    fn insert_before_this(
        &self,
        parent: &R::Element,
        child: &mut dyn Mountable<R>,
    ) -> bool;

    /// Inserts another `Mountable` type before this one, or before the marker
    /// if this one doesn't exist in the UI (for example, `()`).
    fn insert_before_this_or_marker(
        &self,
        parent: &R::Element,
        child: &mut dyn Mountable<R>,
        marker: Option<&R::Node>,
    ) {
        if !self.insert_before_this(parent, child) {
            child.mount(parent, marker);
        }
    }
}

/// Indicates where a node should be mounted to its parent.
pub enum MountKind<R>
where
    R: Renderer,
{
    /// Node should be mounted before this marker node.
    Before(R::Node),
    /// Node should be appended to the parent’s children.
    Append,
}

impl<T, R> Mountable<R> for Option<T>
where
    T: Mountable<R>,
    R: Renderer,
{
    fn unmount(&mut self) {
        if let Some(ref mut mounted) = self {
            mounted.unmount()
        }
    }

    fn mount(&mut self, parent: &R::Element, marker: Option<&R::Node>) {
        if let Some(ref mut inner) = self {
            inner.mount(parent, marker);
        }
    }

    fn insert_before_this(
        &self,
        parent: &<R as Renderer>::Element,
        child: &mut dyn Mountable<R>,
    ) -> bool {
        self.as_ref()
            .map(|inner| inner.insert_before_this(parent, child))
            .unwrap_or(false)
    }
}

/// Allows data to be added to a static template.
pub trait ToTemplate {
    const TEMPLATE: &'static str = "";
    const CLASS: &'static str = "";
    const STYLE: &'static str = "";
    const LEN: usize = Self::TEMPLATE.as_bytes().len();

    /// Renders a view type to a template. This does not take actual view data,
    /// but can be used for constructing part of an HTML `<template>` that corresponds
    /// to a view of a particular type.
    fn to_template(
        buf: &mut String,
        class: &mut String,
        style: &mut String,
        position: &mut Position,
    );
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
