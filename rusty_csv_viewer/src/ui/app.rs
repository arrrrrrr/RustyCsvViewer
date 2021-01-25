use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::{BoxedResult, resource};
use crate::table;
use crate::table::TableData;
use crate::ui::{Component, OpenFileInfo};
use crate::ui::AppState;
use crate::ui::menu::MenuBuilder;

type NwgResult<T> = Result<T, nwg::NwgError>;

/// The main application structure for the UI
#[derive(Default)]
pub struct MainWindow {
    /// Main window
    pub window: nwg::Window,
    /// Layout for displaying data
    pub layout: nwg::ListView,
    /// Menu container that contains menu trees
    pub menu: Vec<Box<dyn Component + 'static>>,
    /// Status bar
    pub status_bar: nwg::StatusBar,
    /// Open file dialog
    pub file_dialog: nwg::FileDialog,
    /// Find dialog
    pub find_dialog: nwg::GridLayout,
    /// About application dialog
    pub about_dialog: nwg::GridLayout,
}

impl MainWindow {
    /// Create a new instance of an application with default field values
    pub fn new() -> Self {
        MainWindow {
            // TODO: might fail if UCS2 sequence cannot be converted to UTF8
            ..Default::default()
        }
    }

    /// Perform pre-destruction tasks for the main window and disable events
    pub fn exit(&self, state: &mut AppState) {
        // Store the window position and size
        state.set_window_pos(self.window.position());
        state.set_window_size(self.window.size());

        // TODO: handle this error properly
        // Write the settings file before exiting
        if let Err(e) = state.write_settings() {
            eprintln!("{:?}", e);
        }

        // Terminate message loop and unblock the main thread
        nwg::stop_thread_dispatch();
    }

    /// The following methods are for executing menu commands
    /// such as opening a file, closing a file and displaying about information
    ///
    /// The signature of these functions are only constrained to the event handling closure
    /// They are typically called through a capturing closure

    /// Open a file to read data from
    pub fn cmd_open_file(&self, state: &mut AppState) -> BoxedResult<()> {
        let selected = self.open_file_picker_dialog(&self.file_dialog);

        match selected {
            Ok(s) => {
                eprintln!("Selected file: {}", s);

                if let Some(ofi) = self.read_file(&s) {
                    state.load_data(ofi);
                    // TODO: Layout the data
                }

                Ok(())
            },
            Err(e) => Err(e)
        }
    }

    /// Close an open file and remove the layout
    pub fn cmd_close_file(&self, state: &mut AppState) -> BoxedResult<()> {
        let _ofi = state.unload_data();
        eprintln!("cmd_close_file: Closing open file");
        Ok(())
    }

    /// Exit the application
    /// Uses Window::close() to send an exit message to the main window
    pub fn cmd_exit(&self) -> BoxedResult<()> {
        eprintln!("cmd_exit: exiting");
        Ok(self.window.close())
    }

    /// Find a pattern in the open data and store the results
    /// for the UI to display and navigate
    pub fn cmd_find(&self, _event_data: &nwg::EventData) -> BoxedResult<()> {
        eprintln!("cmd_find: showing find dialog");
        Ok(())
    }

    /// Modify the application preferences
    /// The preferences are stored in a ui::state::settings::Settings
    pub fn cmd_preferences(&self, _state: &mut AppState, _event_data: &nwg::EventData) -> BoxedResult<()> {
        eprintln!("cmd_preferences: showing preferences dialog");
        Ok(())
    }

    /// Display the about application dialog window
    pub fn cmd_about(&self) -> BoxedResult<()> {
        eprintln!("cmd_about: showing about dialog");
        Ok(())
    }

    /// Run the open file dialog for the user to select a file to open
    ///
    /// Returns a valid file name or error
    fn open_file_picker_dialog(&self, dialog: &nwg::FileDialog) -> BoxedResult<String> {
        // Run the file picker dialog and select a file
        if dialog.run(Some(&self.window)) {
            match dialog.get_selected_item() {
                Ok(s) => return Ok(s),
                Err(e) => return Err(Box::new(e))
            }
        }

        // Dialog was cancelled
        Ok(String::new())
    }

    /// Read a selected file into a TableData structure
    ///
    /// If the extension ends with:
    ///     .csv:      comma separated value format
    ///     .tsv|.txt: tab separated value format
    ///
    /// Returns an OpenFileInfo structure with the data and filename if successful
    fn read_file(&self, filename: &str) -> Option<OpenFileInfo> {
        let msg= move |content| nwg::fatal_message("Open File", format!("{}", content).as_str());
        let mut data: Option<TableData> = None;

        // Map error types into formatted strings to simplify display logic
        if filename.ends_with("csv") {
            data = table::from_csv_file(filename, false)
                .map_err(|e| e.to_string()).unwrap()
                .map_or_else(|e| { msg(e); None }, |v| Some(v));

        }
        else if filename.ends_with("tsv") || filename.ends_with("txt") {
            data = table::from_tsv_file(filename, false)
                .map_err(|e| e.to_string()).unwrap()
                .map_or_else(|e| { msg(e); None }, |v| Some(v));
        }

        if let Some(d) = data {
            return Some(OpenFileInfo { name: filename.to_string(), data: d });
        }

        None
    }

    /// Search the menu tree and locate a menu container by its handle
    ///
    /// root:  Vector of Menu Components
    /// handle: Menu handle captured from an event handler
    ///
    pub fn find_menu_by_handle<'a>(root: &'a Vec<Box<dyn Component + 'static>>, handle: &nwg::ControlHandle) ->
    Option<&'a Box<dyn Component + 'static>>
    {
        // Iterate recursively through the menus to locate the handle
        for menu in root.iter() {
            if &menu.handle() == &handle {
                return Some(menu);
            }

            // Each child may be a tree itself so recursively call this function on each child
            if let Some(children) = menu.children() {
                if let Some(res) = MainWindow::find_menu_by_handle(children, &handle) {
                    return Some(res);
                }
            }
        }

        // No match
        None
    }

    /// Create the main window for the application
    pub fn create_main_window(data: Rc<RefCell<MainWindow>>, state: Arc<Mutex<AppState>>) -> NwgResult<()> {
        let settings = state.lock().unwrap();

        // Controls
        nwg::Window::builder()
            .flags(nwg::WindowFlags::MAIN_WINDOW | nwg::WindowFlags::VISIBLE)
            .size((settings.window_size().0 as i32, settings.window_size().1 as i32))
            .position(settings.window_pos())
            .title(resource::CAppAbout::NAME)
            .build(&mut data.borrow_mut().window)
    }

    /// Create a status bar for the window
    /// The parent is the main window
    pub fn create_status_bar(data: Rc<RefCell<MainWindow>>) -> NwgResult<()> {
        let parent = data.borrow().window.handle.clone();

        nwg::StatusBar::builder()
            .text("")
            .parent(&parent)
            .build(&mut data.borrow_mut().status_bar)
    }

    /// Create the dialog for opening files
    pub fn create_file_picker_dialog(data: Rc<RefCell<MainWindow>>) -> NwgResult<()> {
        nwg::FileDialog::builder()
            .title(resource::CDialogOpenFile::TITLE)
            .action(nwg::FileDialogAction::Open)
            .filters(resource::CDialogOpenFile::FILTER)
            .build(&mut data.borrow_mut().file_dialog)
    }

    /// Create the menubar and submenu hierarchy
    ///
    /// Creates a menu bar with the following menus:
    ///
    ///     File            | Edit              | Help
    ///       Open File     |   Find            |   About
    ///       Close File    |   ----            |
    ///       ----------    |   Preferences     |
    ///       Exit          |                   |
    ///
    pub fn create_menus(data: Rc<RefCell<MainWindow>>) -> NwgResult<()> {
        use crate::ui::menu::MenuBuildType as BT;
        use crate::resource::*;

        let hwnd = data.borrow().window.handle.clone();

        // File menu template
        //
        // File { Open File, Close File, -- Separator --, Exit }
        let file_template = BT::Menu(
            CMenuFile::NAME.to_string(),
            false,
            vec![
                BT::MenuItem(CMenuFile::CHILD[0].to_string(), false,
                             Box::new(move |a,s,_e,_d|
                                 MainWindow::cmd_open_file(a, s))
                ),
                BT::MenuItem(CMenuFile::CHILD[1].to_string(), false,
                             Box::new(move |a,s,_e,_d|
                                 MainWindow::cmd_close_file(a, s))
                ),
                BT::MenuSeparator,
                BT::MenuItem(CMenuFile::CHILD[2].to_string(), false,
                             Box::new(move |a,_s,_e,_d| MainWindow::cmd_exit(a))
                ),
            ]
        );

        // Edit menu template
        //
        // Edit { Find, -- Separator --, Preferences }
        let edit_template = BT::Menu(
            CMenuEdit::NAME.to_string(),
            false,
            vec![
                BT::MenuItem(CMenuEdit::CHILD[0].to_string(), false,
                             Box::new(move |a,_s,_e,d|
                                 MainWindow::cmd_find(a, d))
                ),
                BT::MenuSeparator,
                BT::MenuItem(CMenuEdit::CHILD[1].to_string(), false,
                             Box::new(move |a,s,_e,d|
                                 MainWindow::cmd_preferences(a, s, d))
                ),
            ]
        );

        // Help menu template
        //
        // Help { About }
        let help_template = BT::Menu(
            CMenuHelp::NAME.to_string(),
            false,
            vec![
                BT::MenuItem(CMenuHelp::CHILD[0].to_string(), false,
                             Box::new(move |a,_s,_e,_d| MainWindow::cmd_about(a))
                ),
            ]
        );

        let mut v: Vec<Box<dyn Component>> = Vec::new();
        // Now build the menus from the templates
        v.push(MenuBuilder::builder(file_template).build(&hwnd)?);
        v.push(MenuBuilder::builder(edit_template).build(&hwnd)?);
        v.push(MenuBuilder::builder(help_template).build(&hwnd)?);
        // Store the menus in App
        data.borrow_mut().menu = v;

        Ok(())
    }
}