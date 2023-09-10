use crate::hydration::Cursor;

pub mod dynamic;
pub mod strings;
pub mod tuples;

pub trait View {
    /// The “view state” for this type, which can be retained between updates.
    ///
    /// For example, for a text node, `State` might be the actual DOM text node
    /// and the previous string, to allow for diffing between updates.
    type State;

    /// Renders a view to HTML.
    fn to_html(&self, buf: &mut String, position: &mut Position);

    /// Makes a set of DOM nodes rendered from HTML interactive. If `FROM_SERVER` is
    /// `true`, this HTML was rendered on the server. If `FROM_SERVER` is `false`, the
    /// HTML was in a client-side `<template>` element.
    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &mut Cursor,
        position: &mut Position,
    ) -> Self::State;

    /// Updates the view with new data.
    fn rebuild(self, state: &mut Self::State);
}

pub trait ToTemplate {
    /// Renders a view type to a template. This does not take actual view data,
    /// but can be used for constructing part of an HTML `<template>` that corresponds
    /// to a view of a particular type.
    fn to_template(buf: &mut String, position: &mut Position);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Position {
    Root,
    FirstChild,
    NextChild,
    OnlyChild,
    LastChild,
}
