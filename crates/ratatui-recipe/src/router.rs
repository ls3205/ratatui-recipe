use tokio::sync::mpsc;

pub enum RouterAction<ID> {
    PUSH(ID),
    REPLACE(ID),
    BACK,
    CLEAR,
    RESTART,
    EXIT,
    REDRAW,
}

#[derive(Clone)]
pub struct Router<ID> {
    bus: mpsc::UnboundedSender<RouterAction<ID>>,
}

impl<ID> Router<ID> {
    pub fn new(bus: mpsc::UnboundedSender<RouterAction<ID>>) -> Self {
        Router { bus }
    }

    pub fn push(&self, id: ID) {
        let _ = self.bus.send(RouterAction::PUSH(id));
    }

    pub fn replace(&self, id: ID) {
        let _ = self.bus.send(RouterAction::REPLACE(id));
    }

    pub fn back(&self) {
        let _ = self.bus.send(RouterAction::BACK);
    }

    pub fn clear(&self) {
        let _ = self.bus.send(RouterAction::CLEAR);
    }

    pub fn restart(&self) {
        let _ = self.bus.send(RouterAction::RESTART);
    }

    pub fn exit(&self) {
        let _ = self.bus.send(RouterAction::EXIT);
    }

    pub fn redraw(&self) {
        let _ = self.bus.send(RouterAction::REDRAW);
    }
}
