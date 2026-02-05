mod app;
mod page;
mod router;

pub use app::App;
pub use page::{Page, PageState, StatefulPage};
pub use router::Router;

pub use ratatui_recipe_macros::Pages;
