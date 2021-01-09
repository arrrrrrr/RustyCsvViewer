use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::{NativeUiEx};
use crate::ui::{App,AppState,ComponentParams};

pub struct AppUi {
    inner: Rc<RefCell<App>>,
    state: Arc<Mutex<AppState>>,
    default_handler: RefCell<Option<nwg::EventHandler>>,
    control_handlers: RefCell<Vec<nwg::EventHandler>>,
}

impl NativeUiEx<AppUi, Arc<Mutex<AppState>>> for App {
    fn build_ui(mut data: App, state: Arc<Mutex<AppState>>) -> Result<AppUi, nwg::NwgError> {
        use nwg::Event as E;

        let ui = AppUi {
            inner: Rc::new(RefCell::new(data)),
            state: Arc::clone(&state),
            default_handler: Default::default(),
            control_handlers: Default::default(),
        };

        // Create the main window
        App::create_main_window(Rc::clone(&ui.inner), Arc::clone(&state))?;
        // Create the file picker dialog
        App::create_file_picker_dialog(Rc::clone(&ui.inner))?;
        // Create the menubar and submenus
        App::create_menus(Rc::clone(&ui.inner))?;

        let evt_ui = Rc::downgrade(&Rc::clone(&ui.inner));
        let evt_state = Arc::downgrade(&Arc::clone(&ui.state));

        let handle_events = move |evt, evt_data, handle| {
            if let Some(ui) = evt_ui.upgrade() {
                if let Some(state) = evt_state.upgrade() {
                    match evt {
                        E::OnWindowClose => {
                            if &handle == &ui.borrow().window.handle {
                                App::exit(&ui.borrow(), &mut state.lock().unwrap());
                            }
                        },
                        E::OnMenuItemSelected => {
                            // Search the menu tree and return the menu item that matches the handle
                            if let Some(menu) = App::find_menu_by_handle(&ui.borrow().menu, &handle) {
                                // Build the parameters for the command to be executed
                                let params =
                                    ComponentParams::new(Rc::clone(&ui), Arc::clone(&state),
                                                         evt, evt_data);
                                // Execute the command
                                menu.run(params)
                                    .map_err(|e| nwg::error_message(menu.name(), &format!("{:?}", e)));
                            }
                        }
                        _ => {}
                    }
                }
            }
        };

        *ui.default_handler.borrow_mut() =
            Some(nwg::full_bind_event_handler(&ui.window.handle, handle_events));

        Ok(ui)
    }
}

impl Drop for AppUi {
    fn drop(&mut self) {
        // To make sure that everything is freed without issues, the default handler must be unbound.
        let handler = self.default_handler.borrow();
        if handler.is_some() {
            nwg::unbind_event_handler(handler.as_ref().unwrap());
        }

        // Unbind any event handlers for common controls
        for h in self.control_handlers.borrow().iter() {
            nwg::unbind_event_handler(h);
        }
    }
}

impl Deref for AppUi {
    type Target = App;

    fn deref(&self) -> &App {
        unsafe {
            &self.inner.as_ptr().as_ref().unwrap()
        }
    }
}