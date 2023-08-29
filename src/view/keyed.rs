use super::{
    html::{attribute::Attribute, Html},
    View,
};
use crate::dom::{Dom, Node};
use drain_filter_polyfill::VecExt as VecDrainFilterExt;
use indexmap::IndexSet;
use rustc_hash::FxHasher;
use std::hash::{BuildHasherDefault, Hash};

type FxIndexSet<T> = IndexSet<T, BuildHasherDefault<FxHasher>>;

pub struct Keyed<K, A, C>(pub K, pub Html<A, C>)
where
    K: Eq + Hash + 'static,
    A: Attribute,
    C: View;

impl<'a, K, A, C> View for Vec<Keyed<K, A, C>>
where
    K: Eq + Hash + 'static,
    A: Attribute,
    C: View,
{
    type State = (
        Option<Node>,
        IndexSet<K, BuildHasherDefault<FxHasher>>,
        Vec<Option<<Html<A, C> as View>::State>>,
    );

    fn build(self) -> Self::State {
        let capacity = self.len();
        let mut hashed_items = FxIndexSet::with_capacity_and_hasher(capacity, Default::default());
        let mut rendered_items = Vec::new();
        for Keyed(key, item) in self {
            hashed_items.insert(key);
            rendered_items.push(Some(item.build()));
        }
        (None, hashed_items, rendered_items)
    }

    fn rebuild(self, state: &mut Self::State) {
        let (parent, prev_hashes, ref mut children) = state;
        let capacity = self.len();
        let mut new_hashed_items =
            FxIndexSet::with_capacity_and_hasher(capacity, Default::default());

        let mut items = Vec::new();
        for Keyed(key, item) in self {
            new_hashed_items.insert(key);
            items.push(Some(item));
        }

        let cmds = diff(&prev_hashes, &new_hashed_items);

        apply_diff(parent.unwrap(), cmds, children, items);

        *prev_hashes = new_hashed_items;
    }

    fn mount(state: &mut Self::State, parent: Node) {
        let (empty_parent, _, items) = state;
        *empty_parent = Some(parent);
        for item in items.iter_mut().flatten() {
            Html::<A, C>::mount(item, parent);
        }
    }

    fn unmount(state: &mut Self::State) {
        let (_, _, items) = state;
        for item in items.iter_mut().flatten() {
            Html::<A, C>::unmount(item);
        }
    }
}

trait VecExt<T> {
    fn get_next_closest_mounted_sibling(&self, start_at: usize) -> Option<&Option<T>>;
}

impl<T> VecExt<T> for Vec<Option<T>> {
    fn get_next_closest_mounted_sibling(&self, start_at: usize) -> Option<&Option<T>> {
        self[start_at..].iter().find(|s| s.is_some())
    }
}

/// Calculates the operations needed to get from `from` to `to`.
fn diff<K: Eq + Hash>(from: &FxIndexSet<K>, to: &FxIndexSet<K>) -> Diff {
    if from.is_empty() && to.is_empty() {
        return Diff::default();
    } else if to.is_empty() {
        return Diff {
            clear: true,
            ..Default::default()
        };
    } else if from.is_empty() {
        return Diff {
            added: to
                .iter()
                .enumerate()
                .map(|(at, _)| DiffOpAdd {
                    at,
                    mode: DiffOpAddMode::Append,
                })
                .collect(),
            ..Default::default()
        };
    }

    let mut removed = vec![];
    let mut moved = vec![];
    let mut added = vec![];
    let max_len = std::cmp::max(from.len(), to.len());

    for index in 0..max_len {
        let from_item = from.get_index(index);
        let to_item = to.get_index(index);

        // if they're the same, do nothing
        if from_item != to_item {
            // if it's only in old, not new, remove it
            if from_item.is_some() && !to.contains(from_item.unwrap()) {
                let op = DiffOpRemove { at: index };
                removed.push(op);
            }
            // if it's only in new, not old, add it
            if to_item.is_some() && !from.contains(to_item.unwrap()) {
                let op = DiffOpAdd {
                    at: index,
                    mode: DiffOpAddMode::Normal,
                };
                added.push(op);
            }
            // if it's in both old and new, it can either
            // 1) be moved (and need to move in the DOM)
            // 2) be moved (but not need to move in the DOM)
            //    * this would happen if, for example, 2 items
            //      have been added before it, and it has moved by 2
            if let Some(from_item) = from_item {
                if let Some(to_item) = to.get_full(from_item) {
                    let moves_forward_by = (to_item.0 as i32) - (index as i32);
                    let move_in_dom =
                        moves_forward_by != (added.len() as i32) - (removed.len() as i32);

                    let op = DiffOpMove {
                        from: index,
                        len: 1,
                        to: to_item.0,
                        move_in_dom,
                    };
                    moved.push(op);
                }
            }
        }
    }

    moved = group_adjacent_moves(moved);

    Diff {
        removed,
        items_to_move: moved.iter().map(|m| m.len).sum(),
        moved,
        added,
        clear: false,
    }
}

/// Group adjacent items that are being moved as a group.
/// For example from `[2, 3, 5, 6]` to `[1, 2, 3, 4, 5, 6]` should result
/// in a move for `2,3` and `5,6` rather than 4 individual moves.
fn group_adjacent_moves(moved: Vec<DiffOpMove>) -> Vec<DiffOpMove> {
    let mut prev: Option<DiffOpMove> = None;
    let mut new_moved = Vec::with_capacity(moved.len());
    for m in moved {
        match prev {
            Some(mut p) => {
                if (m.from == p.from + p.len) && (m.to == p.to + p.len) {
                    p.len += 1;
                    prev = Some(p);
                } else {
                    new_moved.push(prev.take().unwrap());
                    prev = Some(m);
                }
            }
            None => prev = Some(m),
        }
    }
    if let Some(prev) = prev {
        new_moved.push(prev)
    }
    new_moved
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Diff {
    removed: Vec<DiffOpRemove>,
    moved: Vec<DiffOpMove>,
    items_to_move: usize,
    added: Vec<DiffOpAdd>,
    clear: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DiffOpMove {
    /// The index this range is starting relative to `from`.
    from: usize,
    /// The number of elements included in this range.
    len: usize,
    /// The starting index this range will be moved to relative to `to`.
    to: usize,
    /// Marks this move to be applied to the DOM, or just to the underlying
    /// storage
    move_in_dom: bool,
}

impl Default for DiffOpMove {
    fn default() -> Self {
        Self {
            from: 0,
            to: 0,
            len: 1,
            move_in_dom: true,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct DiffOpAdd {
    at: usize,
    mode: DiffOpAddMode,
}

#[derive(Debug, PartialEq, Eq)]
struct DiffOpRemove {
    at: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DiffOpAddMode {
    Normal,
    Append,
}

impl Default for DiffOpAddMode {
    fn default() -> Self {
        Self::Normal
    }
}

fn apply_diff<A, C>(
    parent: Node,
    diff: Diff,
    children: &mut Vec<Option<<Html<A, C> as View>::State>>,
    mut items: Vec<Option<Html<A, C>>>,
) where
    A: Attribute,
    C: View,
{
    // The order of cmds needs to be:
    // 1. Clear
    // 2. Removals
    // 3. Move out
    // 4. Resize
    // 5. Move in
    // 6. Additions
    // 7. Removes holes
    if diff.clear {
        parent.set_text("");
        children.clear();

        if diff.added.is_empty() {
            return;
        }
    }

    for DiffOpRemove { at } in &diff.removed {
        let mut item_to_remove = children[*at].take().unwrap();

        Html::<A, C>::unmount(&mut item_to_remove);
    }

    let (move_cmds, add_cmds) = unpack_moves(&diff);

    let mut moved_children = move_cmds
        .iter()
        .map(|move_| {
            let mut each_item = children[move_.from].take().unwrap();
            Some(each_item)
        })
        .collect::<Vec<_>>();

    children.resize_with(children.len() + diff.added.len(), || None);

    for (i, DiffOpMove { to, .. }) in move_cmds
        .iter()
        .enumerate()
        .filter(|(_, move_)| !move_.move_in_dom)
    {
        let mut child = moved_children[i].take();
        let item = items[i].take();
        if let (Some(child), Some(item)) = (&mut child, item) {
            item.rebuild(child);
        }
        children[*to] = child;
    }

    for (i, DiffOpMove { to, .. }) in move_cmds
        .into_iter()
        .enumerate()
        .filter(|(_, move_)| move_.move_in_dom)
    {
        let mut each_item = moved_children[i].take().unwrap();

        if let Some(Some((node, _, _))) = children.get_next_closest_mounted_sibling(to) {
            Dom::insert_before(parent, each_item.0, *node);
        } else {
            Html::<A, C>::mount(&mut each_item, parent);
        }

        children[to] = Some(each_item);
    }

    for DiffOpAdd { at, mode } in add_cmds {
        let item = items[at].take().unwrap();
        let mut item = item.build();

        match mode {
            DiffOpAddMode::Normal => {
                if let Some(Some((node, _, _))) = moved_children.get(at + 1) {
                    Dom::insert_before(parent, *node, item.0);
                } else {
                    Html::<A, C>::mount(&mut item, parent);
                }
            }
            DiffOpAddMode::Append => {
                Html::<A, C>::mount(&mut item, parent);
            }
        }

        children[at] = Some(item);
    }

    for (item, state) in items.into_iter().zip(children.iter_mut()) {
        if let (Some(item), Some(state)) = (item, state.as_mut()) {
            item.rebuild(state);
        }
    }

    #[allow(unstable_name_collisions)]
    children.drain_filter(|c| c.is_none());
}

fn unpack_moves(diff: &Diff) -> (Vec<DiffOpMove>, Vec<DiffOpAdd>) {
    let mut moves = Vec::with_capacity(diff.items_to_move);
    let mut adds = Vec::with_capacity(diff.added.len());

    let mut removes_iter = diff.removed.iter();
    let mut adds_iter = diff.added.iter();
    let mut moves_iter = diff.moved.iter();

    let mut removes_next = removes_iter.next();
    let mut adds_next = adds_iter.next();
    let mut moves_next = moves_iter.next().copied();

    for i in 0..diff.items_to_move + diff.added.len() + diff.removed.len() {
        if let Some(DiffOpRemove { at, .. }) = removes_next {
            if i == *at {
                removes_next = removes_iter.next();

                continue;
            }
        }

        match (adds_next, &mut moves_next) {
            (Some(add), Some(move_)) => {
                if add.at == i {
                    adds.push(*add);

                    adds_next = adds_iter.next();
                } else {
                    let mut single_move = *move_;
                    single_move.len = 1;

                    moves.push(single_move);

                    move_.len -= 1;
                    move_.from += 1;
                    move_.to += 1;

                    if move_.len == 0 {
                        moves_next = moves_iter.next().copied();
                    }
                }
            }
            (Some(add), None) => {
                adds.push(*add);

                adds_next = adds_iter.next();
            }
            (None, Some(move_)) => {
                let mut single_move = *move_;
                single_move.len = 1;

                moves.push(single_move);

                move_.len -= 1;
                move_.from += 1;
                move_.to += 1;

                if move_.len == 0 {
                    moves_next = moves_iter.next().copied();
                }
            }
            (None, None) => break,
        }
    }

    (moves, adds)
}
