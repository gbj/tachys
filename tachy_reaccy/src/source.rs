use crate::{Observer, OBSERVER};
use rustc_hash::FxHashSet;
use std::{
    collections::hash_set::{IntoIter, Iter},
    fmt::Debug,
    hash::Hash,
    mem,
    sync::Arc,
};

pub trait ReactiveNode {
    /// Sets the state of this source.
    fn set_state(&self, state: ReactiveNodeState);

    /// Notifies the source's dependencies that it has changed.
    fn mark_dirty(&self);

    /// Notifies the source's dependencies that it may have changed.
    fn mark_check(&self);

    /// Marks that all subscribers need to be checked.
    fn mark_subscribers_check(&self);

    /// Regenerates the value for this node, if needed, and returns whether
    /// it has actually changed or not.
    fn update_if_necessary(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReactiveNodeState {
    Clean,
    Check,
    Dirty,
}

/// Describes the behavior of any source of reactivity (like a signal, trigger, or memo.)
pub trait Source: ReactiveNode {
    /// Converts this type to its type-erased equivalent.
    fn to_any_source(&self) -> AnySource;

    /// Adds a subscriber to this source's list of dependencies.
    fn add_subscriber(&self, subscriber: AnySubscriber);

    /// Removes a subscriber from this source's list of dependencies.
    fn remove_subscriber(&self, subscriber: &AnySubscriber);

    /// Remove all subscribers from this source's list of dependencies.
    fn clear_subscribers(&self);
}

pub trait Track: Source {
    fn track(&self) {
        if let Some(subscriber) = Observer::get() {
            subscriber.add_source(self.to_any_source());
            self.add_subscriber(subscriber);
        }
    }
}

impl<T: Source> Track for T {}

#[derive(Clone)]
pub struct AnySource(pub usize, pub Arc<dyn Source + Send + Sync>);

impl Debug for AnySource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AnySource").field(&self.0).finish()
    }
}

impl Hash for AnySource {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl PartialEq for AnySource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for AnySource {}

impl Source for AnySource {
    fn to_any_source(&self) -> AnySource {
        self.clone()
    }

    fn add_subscriber(&self, subscriber: AnySubscriber) {
        self.1.add_subscriber(subscriber)
    }

    fn remove_subscriber(&self, subscriber: &AnySubscriber) {
        self.1.remove_subscriber(subscriber)
    }

    fn clear_subscribers(&self) {
        self.1.clear_subscribers();
    }
}

impl ReactiveNode for AnySource {
    fn set_state(&self, state: ReactiveNodeState) {
        self.1.set_state(state)
    }

    fn mark_dirty(&self) {
        self.1.mark_dirty()
    }

    fn mark_subscribers_check(&self) {
        self.1.mark_subscribers_check()
    }

    fn update_if_necessary(&self) -> bool {
        self.1.update_if_necessary()
    }

    fn mark_check(&self) {
        self.1.mark_check()
    }
}

/// Any type that can track reactive values (like an effect or a memo).
pub trait Subscriber: ReactiveNode {
    /// Converts this type to its type-erased equivalent.
    fn to_any_subscriber(&self) -> AnySubscriber;

    /// Adds a subscriber to this subscriber's list of dependencies.
    fn add_source(&self, source: AnySource);

    /// Clears the set of sources for this subscriber.
    fn clear_sources(&self);
}

/// A type-erased subscriber.
#[derive(Clone)]
pub struct AnySubscriber(pub usize, pub Arc<dyn Subscriber + Send + Sync>);

impl Subscriber for AnySubscriber {
    fn to_any_subscriber(&self) -> AnySubscriber {
        self.clone()
    }

    fn add_source(&self, source: AnySource) {
        self.1.add_source(source);
    }

    fn clear_sources(&self) {
        self.1.clear_sources();
    }
}

impl ReactiveNode for AnySubscriber {
    fn set_state(&self, state: ReactiveNodeState) {
        self.1.set_state(state)
    }

    fn mark_dirty(&self) {
        self.1.mark_dirty()
    }

    fn mark_subscribers_check(&self) {
        self.1.mark_subscribers_check()
    }

    fn update_if_necessary(&self) -> bool {
        self.1.update_if_necessary()
    }

    fn mark_check(&self) {
        self.1.mark_check()
    }
}

impl AnySubscriber {
    pub fn with_observer<T>(&self, fun: impl FnOnce() -> T) -> T {
        let prev = {
            OBSERVER.with(|o| {
                mem::replace(&mut *o.borrow_mut(), Some(self.clone()))
            })
        };
        let val = fun();
        OBSERVER.with(|o| {
            *o.borrow_mut() = prev;
        });
        val
    }
}

impl Debug for AnySubscriber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AnySubscriber").field(&self.0).finish()
    }
}

impl Hash for AnySubscriber {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl PartialEq for AnySubscriber {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for AnySubscriber {}

#[derive(Default, Clone)]
pub struct SourceSet(FxHashSet<AnySource>);

impl SourceSet {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn insert(&mut self, source: AnySource) {
        self.0.insert(source);
    }

    pub fn remove(&mut self, source: &AnySource) {
        self.0.remove(source);
    }

    pub fn take(&mut self) -> FxHashSet<AnySource> {
        mem::take(&mut self.0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn clear_sources(&mut self, subscriber: &AnySubscriber) {
        for source in self.take() {
            source.remove_subscriber(subscriber);
        }
    }
}

impl IntoIterator for SourceSet {
    type Item = AnySource;
    type IntoIter = IntoIter<AnySource>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a SourceSet {
    type Item = &'a AnySource;
    type IntoIter = Iter<'a, AnySource>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(Default, Clone)]
pub struct SubscriberSet(FxHashSet<AnySubscriber>);

impl SubscriberSet {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn subscribe(&mut self, subscriber: AnySubscriber) {
        self.0.insert(subscriber);
    }

    pub fn unsubscribe(&mut self, subscriber: &AnySubscriber) {
        self.0.remove(subscriber);
    }

    pub fn take(&mut self) -> FxHashSet<AnySubscriber> {
        mem::take(&mut self.0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl IntoIterator for SubscriberSet {
    type Item = AnySubscriber;
    type IntoIter = IntoIter<AnySubscriber>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a SubscriberSet {
    type Item = &'a AnySubscriber;
    type IntoIter = Iter<'a, AnySubscriber>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
