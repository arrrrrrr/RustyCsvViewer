use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::BoxedResult;
use crate::ui::AppState;

use super::MainWindow;

/// Trait for common components
pub trait Component {
    fn name(&self) -> &str {
        ""
    }
    fn handle(&self) -> &nwg::ControlHandle;

    fn run(&self, params: ComponentParams) -> BoxedResult<()> {
        Ok(())
    }

    fn children(&self) -> Option<&Vec<Box<dyn Component + 'static>>> {
        None
    }

    fn add_child(&mut self, child: Box<dyn Component + 'static>);
    fn clear_children(&mut self);
}

/**
    Parameters for executing a command on a component inside a
    native_windows_gui event handler

    * Arguments

     callee:     The target object of the call. The self argument
     state:      Mutable state for the command to modify
     event:      Event for native_windows_gui that the event handler passes
     event_data: EventData for native_windows_gui that provides additional
                 details for the Event
*/
pub struct ComponentParams {
    pub callee: Rc<RefCell<MainWindow>>,
    pub state: Arc<Mutex<AppState>>,
    pub event: nwg::Event,
    pub event_data: nwg::EventData
}

impl ComponentParams {
    pub fn new(callee: Rc<RefCell<MainWindow>>, state: Arc<Mutex<AppState>>, event: nwg::Event, event_data: nwg::EventData)
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