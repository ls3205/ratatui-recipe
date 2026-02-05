use std::{collections::VecDeque, io::Result};

use ratatui::crossterm::event::{self, Event};
use tokio::sync::mpsc;

use crate::{
    page::PageState,
    router::{Router, RouterAction},
};

pub struct App<S = ()> {
    event_bus: mpsc::UnboundedReceiver<Event>,
    state: S,
}

impl App<()> {
    pub fn new() -> Self {
        let (bus_tx, bus_rx) = mpsc::unbounded_channel();

        tokio::task::spawn_blocking(move || {
            loop {
                if let Ok(event) = event::read()
                    && bus_tx.send(event).is_err()
                {
                    break;
                }
            }
        });

        Self {
            event_bus: bus_rx,
            state: (),
        }
    }
}

impl<S> App<S> {
    pub fn stateful(state: S) -> Self {
        let (bus_tx, bus_rx) = mpsc::unbounded_channel();

        tokio::task::spawn_blocking(move || {
            loop {
                if let Ok(event) = event::read()
                    && bus_tx.send(event).is_err()
                {
                    break;
                }
            }
        });

        Self {
            event_bus: bus_rx,
            state,
        }
    }

    pub async fn run<P>(&mut self) -> Result<()>
    where
        P: PageState<S>,
    {
        let mut terminal = ratatui::init();
        let mut pages = VecDeque::from([P::default()]);

        let (bus_tx, mut bus_rx) = mpsc::unbounded_channel();
        let router = Router::new(bus_tx);

        pages
            .back_mut()
            .unwrap()
            .on_enter(router.clone(), &mut self.state)
            .await;

        let mut draw = true;

        loop {
            let page = pages.back_mut().expect("uhoh");

            if draw {
                terminal
                    .draw(|f| page.draw(f, &self.state))
                    .inspect_err(|_| {
                        ratatui::restore();
                    })?;

                draw = false;
            }

            tokio::select! {
                _ = page.task(router.clone(), &mut self.state) => {},
                Some(event) = self.event_bus.recv() => {
                    if let Event::Resize(_, _) = event {
                        draw = true;
                    }

                    page.on_event(event, router.clone(), &mut self.state).await;
                },
                Some(action) = bus_rx.recv() => {
                    match action {
                        RouterAction::PUSH(id) => {
                            page.on_pause(router.clone(), &mut self.state).await;

                            let mut page = P::new(id);
                            page.on_enter(router.clone(), &mut self.state).await;

                            pages.push_back(page);

                            draw = true;
                        }
                        RouterAction::REPLACE(id) => {
                            let mut old_page = pages.pop_back().unwrap();
                            old_page.on_exit(router.clone(), &mut self.state).await;

                            let mut new_page = P::new(id);
                            new_page.on_enter(router.clone(), &mut self.state).await;
                            pages.push_back(new_page);

                            draw = true;
                        }
                        RouterAction::BACK => {
                            if pages.len() > 1 {
                                let mut old_page = pages.pop_back().unwrap();
                                old_page.on_exit(router.clone(), &mut self.state).await;

                                let current_page = pages.back_mut().unwrap();
                                current_page.on_resume(router.clone(), &mut self.state).await;

                                draw = true;
                            }
                        }
                        RouterAction::CLEAR => {
                            let current_page = pages.pop_back().unwrap();

                            while let Some(mut old_page) = pages.pop_back() {
                                old_page.on_exit(router.clone(), &mut self.state).await;
                            }

                            pages.push_back(current_page);
                        }
                        RouterAction::RESTART => {
                            while let Some(mut old_page) = pages.pop_back() {
                                old_page.on_exit(router.clone(), &mut self.state).await;
                            }

                            let mut new_page = P::default();
                            new_page.on_enter(router.clone(), &mut self.state).await;
                            pages.push_back(new_page);

                            draw = true;
                        }
                        RouterAction::REDRAW => {
                            draw = true;
                        }
                        RouterAction::EXIT => {
                            while let Some(mut old_page) = pages.pop_back() {
                                old_page.on_exit(router.clone(), &mut self.state).await;
                            }

                            break;
                        }
                    }
                }
            }
        }

        ratatui::restore();
        Ok(())
    }
}

impl<S> Default for App<S>
where
    S: Default,
{
    fn default() -> Self {
        Self::stateful(S::default())
    }
}
