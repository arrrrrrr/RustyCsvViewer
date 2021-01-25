#[macro_use] extern crate nwg;

pub use ui::{AppState, MainWindow, MainWindowUi, Settings};

mod utils;
mod table;
mod resource;

mod ui;

pub trait NativeUiEx<UI,S> {
    fn build_ui(initial_state: Self, extra_state: S) -> Result<UI, nwg::NwgError>;
}

/// Type alias that represents a Result with:
///  Ok type of T and a boxed Err type of std::error::Error
pub type BoxedResult<T> = Result<T, Box<dyn std::error::Error>>;