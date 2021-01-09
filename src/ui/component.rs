use super::App;
use std::sync::{Arc,Mutex};
use std::cell::RefCell;
use std::rc::Rc;
use std::error::Error;

use crate::ui::AppState;

/// Trait for common components
pub trait Component {
    fn name(&self) -> &str {
        ""
    }
    fn handle(&self) -> &nwg::ControlHandle;

    fn run(&self, params: ComponentParams) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn children(&self) -> Option<&Vec<Box<dyn Component + 'static>>> {
        None
    }

    fn add_child(&mut self, child: Box<dyn Component + 'static>);
    fn clear_children(&mut self);
}

/// Parameters for calling a components lambda from the event handler
pub struct ComponentParams {
    pub callee: Rc<RefCell<App>>,
    pub state: Arc<Mutex<AppState>>,
    pub event: nwg::Event,
    pub event_data: nwg::EventData
}

impl ComponentParams {
    pub fn new(callee: Rc<RefCell<App>>, state: Arc<Mutex<AppState>>, event: nwg::Event, event_data: nwg::EventData)
        -> Self
    {
        ComponentParams {
            callee,
            state,
            event,
            event_data
        }
    }
}