use crate::hydration::Cursor;

pub mod strings;
pub mod tuples;

pub trait View {
    type State;

    fn to_html(&self, buf: &mut String, position: Position);

    fn to_template(buf: &mut String, position: Position) -> Position;

    fn hydrate<const IS_HYDRATING: bool>(self, cursor: &mut Cursor, position: Position)
        -> Position;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Position {
    Root,
    FirstChild,
    NextChild,
    OnlyChild,
    LastChild,
}
