use nwg::{ControlHandle, Event, EventData, NwgError};
use std::cell::RefCell;
use std::rc::Rc;

use crate::MainWindow;
use crate::resource::{CAppAbout, CDialogAboutApp};
use crate::utils::{Point, Rect};

/// About dialog box
///
/// Structure
///
/// Popup Window
/// {
///     Grid Layout
///     {
///         Rich Text Editbox
///     }
/// }
///

pub struct DialogAbout {
    parent_window: Rc<RefCell<nwg::Window>>,
    window: nwg::Window,
    layout: nwg::GridLayout,
    textbox: nwg::RichTextBox,
}

impl nwg::PartialUi for DialogAbout {
    fn build_partial<W: Into<ControlHandle>>(data: &mut Self, parent: Option<W>) -> Result<(), NwgError> {
        let parent = parent.unwrap().into();

        Ok(())
    }

    fn process_event(&self, _evt: Event, _evt_data: &EventData, _handle: ControlHandle) {
        unimplemented!()
    }

    fn handles<'a>(&'a self) -> Vec<&'a ControlHandle> {
        unimplemented!()
    }
}

impl DialogAbout {
    pub fn new(parent: Rc<RefCell<nwg::Window>>) -> Self {
        DialogAbout {
            parent_window: parent,
            window: nwg::Window::default(),
            layout: nwg::GridLayout::default(),
            textbox: nwg::RichTextBox::default()
        }
    }

    fn init(data: &mut Self, parent: &ControlHandle) -> nwg::NwgError {
        unimplemented!();

        // use nwg::WindowFlags as WF;
        // // Create the popup window
        // nwg::Window::builder()
        //     .position(data.window.position())
        //     .size(CDialogAboutApp::WINDOW_SIZE)
        //     .parent(Some(parent))
        //     .title(&format!("About {}", CAppAbout::NAME))
        //     .topmost(true)
        //     .flags(WF::WINDOW | WF::POPUP)
        //     .ex_flags()
        //     .build(&mut data.window)?;

    }

    fn align_rect_rel_to_parent(data: &mut Self, parent: &nwg::Window, child: &nwg::Window) -> Rect<u32> {
        unimplemented!()
    }
}