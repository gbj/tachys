//use browser_only_send::BrowserOnly;
use futures::channel::mpsc::{channel, Receiver, Sender};
use std::{fmt::Debug, hash::Hash};

#[derive(Debug)]
pub struct NotificationSender(Sender<()>);

impl NotificationSender {
    pub fn channel() -> (NotificationSender, Receiver<()>) {
        // buffer of 0 means we can only send N messages at once
        // where N is the number of NotificationSenders
        // we don't implement Clone on that type, so it can only hold 1 message
        // this means we can't double-notify in a single tick
        let (tx, rx) = channel::<()>(0);
        (NotificationSender(tx), rx)
    }

    pub fn notify(&mut self) {
        // if this fails, it's because there's already a message
        // in the buffer. but we're just sending () to wake it up;
        // we really don't care if multiple signals try to notify it synchronously
        // and it fails to send, as long as it's sent the one time
        _ = self.0.try_send(());
    }
}

impl Hash for NotificationSender {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash_receiver(state);
    }
}

impl PartialEq for NotificationSender {
    fn eq(&self, other: &Self) -> bool {
        self.0.same_receiver(&other.0)
    }
}
