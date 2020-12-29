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

        // Controls
        nwg::Window::builder()
            .flags(nwg::WindowFlags::MAIN_WINDOW |
                   nwg::WindowFlags::VISIBLE)
            .size((400,200))
            .position((300,300))
            .title(resource::APP_TITLE)
            .build(&mut data.window)?;

        // Dialogs
        nwg::FileDialog::builder()
            .title(resource::APP_OPEN_FILE_DLG)
            .action(nwg::FileDialogAction::Open)
            .default_folder(&data.default_folder)
            .filters(resource::APP_OPEN_FILE_DLG_FILTER)
            .build(&mut data.file_dialog)?;

        // nwg::MenuItem::builder()
        //     .parent(&data.window)
        //     .build()?;

        let ui = AppUi {
            inner: Rc::new(data),
            default_handler: Default::default(),
        };

        let evt_ui = Rc::downgrade(&ui.inner);

        let handle_events = move |evt, _evt_data, handle| {
            if let Some(ui) = evt_ui.upgrade() {
                match evt {
                    E::OnWindowClose => {
                        if &handle == &ui.window {
                            &ui.on_window_close();
                        }
                    },
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