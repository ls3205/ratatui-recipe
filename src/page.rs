use std::future;

use ratatui::{Frame, crossterm::event::Event};

use crate::router::Router;

pub trait PageState<S = ()>: Default {
    type ID: Copy;
    fn new(id: Self::ID) -> Self;
    fn draw(&mut self, frame: &mut Frame, state: &S);
    async fn on_event(&mut self, event: Event, router: Router<Self::ID>, state: &mut S);
    async fn on_enter(&mut self, router: Router<Self::ID>, state: &mut S);
    async fn on_exit(&mut self, router: Router<Self::ID>, state: &mut S);
    async fn on_pause(&mut self, router: Router<Self::ID>, state: &mut S);
    async fn on_resume(&mut self, router: Router<Self::ID>, state: &mut S);
    async fn task(&mut self, router: Router<Self::ID>, state: &mut S);
}

pub trait Page<ID>: Default {
    fn draw(&mut self, frame: &mut Frame);
    async fn on_event(&mut self, event: Event, router: Router<ID>) {}
    async fn on_enter(&mut self, router: Router<ID>) {}
    async fn on_exit(&mut self, router: Router<ID>) {}
    async fn on_pause(&mut self, router: Router<ID>) {}
    async fn on_resume(&mut self, router: Router<ID>) {}
    async fn task(&mut self, router: Router<ID>) {
        future::pending().await
    }
}

pub trait StatefulPage<ID, State>: Default {
    fn draw(&mut self, frame: &mut Frame, state: &State);
    async fn on_event(&mut self, event: Event, router: Router<ID>, state: &mut State) {}
    async fn on_enter(&mut self, router: Router<ID>, state: &mut State) {}
    async fn on_exit(&mut self, router: Router<ID>, state: &mut State) {}
    async fn on_pause(&mut self, router: Router<ID>, state: &mut State) {}
    async fn on_resume(&mut self, router: Router<ID>, state: &mut State) {}
    async fn task(&mut self, router: Router<ID>, state: &mut State) {
        future::pending().await
    }
}

impl<ID, S, P> StatefulPage<ID, S> for P
where
    P: Page<ID>,
{
    fn draw(&mut self, frame: &mut Frame, _state: &S) {
        self.draw(frame);
    }

    async fn on_event(&mut self, event: Event, router: Router<ID>, _state: &mut S) {
        self.on_event(event, router).await;
    }

    async fn on_enter(&mut self, router: Router<ID>, _state: &mut S) {
        self.on_enter(router).await;
    }

    async fn on_exit(&mut self, router: Router<ID>, _state: &mut S) {
        self.on_exit(router).await;
    }

    async fn on_pause(&mut self, router: Router<ID>, _state: &mut S) {
        self.on_pause(router).await;
    }

    async fn on_resume(&mut self, router: Router<ID>, _state: &mut S) {
        self.on_resume(router).await;
    }

    async fn task(&mut self, router: Router<ID>, _state: &mut S) {
        self.task(router).await;
    }
}
