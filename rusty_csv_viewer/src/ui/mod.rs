pub use app::MainWindow;
pub use app_ui::MainWindowUi;
pub use component::{Component, ComponentParams};
pub use state::app_state::{AppState, OpenFileInfo};
pub use state::settings::Settings;

mod app;
mod app_ui;
mod menu;
mod layout;
mod component;
mod state;
mod dialog;