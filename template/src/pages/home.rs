use ratatui::{
    Frame,
    crossterm::event::{Event, KeyCode},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};
use ratatui_recipe::{Router, StatefulPage};

use crate::{GlobalState, pages::pageID};

#[derive(Default)]
pub struct HomeScreen;

impl StatefulPage<pageID, GlobalState> for HomeScreen {
    fn draw(&mut self, frame: &mut Frame, _state: &GlobalState) {
        let area = frame.area();
        let widget = Paragraph::new(vec![Line::from("Hello, world!")])
            .block(Block::default().borders(Borders::ALL).title("Home"));
        frame.render_widget(widget, area);
    }

    async fn on_event(&mut self, event: Event, router: Router<pageID>, state: &mut GlobalState) {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => router.exit(),
                _ => {}
            }

            router.redraw()
        }
    }
}
