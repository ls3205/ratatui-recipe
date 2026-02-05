use std::{collections::VecDeque, io::Result};

use ratatui::crossterm::event::{self, Event};
use tokio::sync::mpsc;

use crate::{page::PageState, router::Router};

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
                },
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
