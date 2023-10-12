use std::sync::mpsc;

// A broadcast channel has multiple senders and receivers, and allows every sender to broadcast the same message to all the receivers subscribed to the channel.
//
//            Message Passing
// ┌─────────────────────────────────────┐
// │ ┌─────────────────────────────────┐ │
// │ │                                 │ │
// │ │        Broadcast Channel        │ │
// │ │                                 │ │
// │ └────▲─┬───────▲─┬─────────▲─┬────┘ │
// │      │ │       │ │         │ │      │
// │  ┌───┴─▼─┐  ┌──┴─▼──┐  ┌───┴─▼─┐    │
// │  │   A   │  │   B   │  │   C   │    │
// │  ├───────┤  ├───────┤  ├───────┤    │
// └──┘process└──┘process└──┘process└────┘
// Every process A, B & C can `broadcast()` a message to every other process subscribed to the broadcast channel.

type Subscriber<T> = mpsc::Sender<T>;

pub struct Broadcast<T> {
    subscribers: Vec<Subscriber<T>>,
}

impl<T> Broadcast<T> {
    pub fn new() -> Self {
        Broadcast {
            subscribers: vec![],
        }
    }

    pub fn subscribe(&mut self) -> mpsc::Receiver<T> {
        let (sender, receiver) = mpsc::channel();
        self.subscribers.push(sender);
        receiver
    }

    pub fn broadcast(&mut self, message: T)
    where
        T: Clone,
    {
        self.subscribers
            .iter_mut()
            .for_each(|subscriber| subscriber.send(message.clone()).unwrap())
    }
}
