#[macro_use] extern crate nwg;

mod utils;
mod table;
mod resource;

mod ui;
pub use ui::{App,AppState,AppUi,Settings};

pub trait NativeUiEx<UI,S> {
    fn build_ui(initial_state: Self, extra_state: S) -> Result<UI, nwg::NwgError>;
}

pub type BoxedResult<T> = Result<T, Box<dyn std::error::Error>>;