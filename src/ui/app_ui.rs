use crate::ui::app::App;
use crate::ui::resource;

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

pub struct AppUi {
    inner: Rc<App>,
    default_handler: RefCell<Option<nwg::EventHandler>>,
}

impl nwg::NativeUi<AppUi> for App {
    fn build_ui(mut data: App) -> Result<AppUi, nwg::NwgError> {
        use nwg::Event as E;

        nwg::enable_visual_styles();

        // Controls
        nwg::Window::builder()
            .flags(nwg::WindowFlags::MAIN_WINDOW |
                   nwg::WindowFlags::VISIBLE)
            .size((data.state.window_dims.x, data.state.window_dims.y))
            .position((data.state.window_pos.x, data.state.window_pos.y))
            .title(resource::APP_TITLE)
            .build(&mut data.window)?;

        // Create the file picker dialog
        App::create_file_picker_dialog(&mut data.file_dialog);
        // Create the menubar and submenus
        App::create_menus(&mut data.menu, &data.window);

        let ui = AppUi {
            inner: Rc::new(data),
            default_handler: Default::default(),
        };

        let evt_ui = Rc::downgrade(&ui.inner);

        let handle_events = move |evt, _evt_data, handle| {
            if let Some(mut ui) = evt_ui.upgrade() {
                match evt {
                    E::OnWindowClose => {
                        if &handle == &ui.window {
                            App::on_window_close(&ui);
                        }
                    },
                    /// WM_COMMAND HANDLERS FOR MENU ITEMS GO HERE
                    /// TODO: a lookup table of window handle and lambda to execute on message
                    E::OnMenuItemSelected => {
                        use crate::ui::resource::*;

                        if let Some(h) = ui.find_submenu_handle(LMENU_FILE::IS, LMENU_FILE::HAS[0]) {
                            if &handle == h {
                                if let Some(f) = ui.cmd_open_file() {
                                    println!("opened {}", f);
                                }
                            }
                        }
                        else if &handle ==
                            ui.find_submenu_handle(LMENU_FILE::IS, LMENU_FILE::HAS[1]).unwrap()
                        {
                            // close a file!
                            ui.cmd_close_file();
                        }
                        else if &handle ==
                            ui.find_submenu_handle(LMENU_FILE::IS, LMENU_FILE::HAS[2]).unwrap()
                        {
                            // close a file!
                            ui.cmd_exit();
                        }
                        else if &handle ==
                            ui.find_submenu_handle(LMENU_EDIT::IS, LMENU_FILE::HAS[0]).unwrap()
                        {
                            // close a file!
                            ui.cmd_find();
                        }
                        else if &handle ==
                            ui.find_submenu_handle(LMENU_EDIT::IS, LMENU_FILE::HAS[1]).unwrap()
                        {
                            // close a file!
                            unimplemented!()
                        }
                        else if &handle ==
                            ui.find_submenu_handle(LMENU_HELP::IS, LMENU_FILE::HAS[0]).unwrap()
                        {
                            // close a file!
                            ui.cmd_about();
                        }
                    }
                    _ => {}
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
    }
}

impl Deref for AppUi {
    type Target = App;

    fn deref(&self) -> &App {
        &self.inner
    }
}