use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::resource;
use crate::table;
use crate::table::{TableData};
use crate::ui::{OpenFileInfo, Component};
use crate::ui::AppState;
use crate::ui::menu::{MenuBuilder};

type CmdResult = Result<(), Box<dyn Error>>;
type NwgResult<T> = Result<T, nwg::NwgError>;

#[derive(Default)]
pub struct App {
    pub window: nwg::Window,
    pub layout: nwg::ListView,
    pub menu: Vec<Box<dyn Component + 'static>>,
    pub file_dialog: nwg::FileDialog,
    pub find_dialog: nwg::GridLayout,
    pub about_dialog: nwg::GridLayout,
}

impl App {
    pub fn new() -> Self {
        App {
            // TODO: might fail if UCS2 sequence cannot be converted to UTF8
            ..Default::default()
        }
    }

    /// On Event Handler Functions
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

    /** Commands
        Menu bar
            -- File | Help
        Pop-up menus
            -- &File -> &Open File, &Close File, Open Recent?, Exit
            -- &Help -> &About

    **/

    // Execute the open file command
    pub fn cmd_open_file(&self, state: &mut AppState) -> CmdResult {
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

    fn open_file_picker_dialog(&self, dialog: &nwg::FileDialog) -> Result<String,Box<dyn Error>> {
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

    // Read the file contents into a CsvData structure or display a message box on error
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

    // Execute the close file command
    pub fn cmd_close_file(&self, state: &mut AppState) -> CmdResult {
        let ofi = state.unload_data();
        eprintln!("cmd_close_file: Closing open file");
        Ok(())
    }

    pub fn cmd_exit(&self) -> CmdResult {
        eprintln!("cmd_exit: exiting");
        Ok(self.window.close())
    }

    pub fn cmd_find(&self, _event_data: &nwg::EventData) -> CmdResult {
        eprintln!("cmd_find: showing find dialog");
        Ok(())
    }

    pub fn cmd_preferences(&self, state: &mut AppState, _event_data: &nwg::EventData) -> CmdResult {
        eprintln!("cmd_preferences: showing preferences dialog");
        Ok(())
    }

    // Execute the about command
    pub fn cmd_about(&self) -> CmdResult {
        eprintln!("cmd_about: showing about dialog");
        Ok(())
    }

    /** Prepare the layout parameters for displaying the fields
        Considerations for laying out:
          window dims:
            need to calculate the paintable region. Do I resize the window to something more reasonable

          font size/DPI:
            is per monitor DPI a thing I need to think about? vertical line height in pixels.

          scrolling:
            rendered dimensions might be larger than the usable region
            granularity of scroll intervals

          fields:
            best fit (single line/multiline)?
            find the max rect needed for each column

          headers:
            if present draw them distinctly?
            should the header row always be visible when scrolling?

          column data type:
            would it be helpful to pretty up the value by attempting to infer their type.
            prompt the user to accept inferred types

    **/
    pub fn create_layout(&self) -> bool {
        false
    }

    pub fn create_main_window(data: Rc<RefCell<App>>, state: Arc<Mutex<AppState>>) -> NwgResult<()> {
        let settings = state.lock().unwrap();

        // Controls
        nwg::Window::builder()
            .flags(nwg::WindowFlags::MAIN_WINDOW |
                nwg::WindowFlags::VISIBLE)
            .size((settings.window_size().0 as i32, settings.window_size().1 as i32))
            .position(settings.window_pos())
            .title(resource::APP_TITLE)
            .build(&mut data.borrow_mut().window)
    }


    /// create a file picker dialog for opening csv and text files
    pub fn create_file_picker_dialog(data: Rc<RefCell<App>>) -> NwgResult<()> {
        nwg::FileDialog::builder()
            .title(resource::APP_OPEN_FILE_DLG)
            .action(nwg::FileDialogAction::Open)
            .filters(resource::APP_OPEN_FILE_DLG_FILTER)
            .build(&mut data.borrow_mut().file_dialog)
    }

    pub fn create_menus(data: Rc<RefCell<App>>) -> NwgResult<()> {
        use crate::ui::menu::MenuBuildType as BT;
        use crate::resource::*;

        let hwnd = data.borrow().window.handle.clone();

        // File menu template
        //
        // File
        //   Open File
        //   Close File
        //   ----
        //   Exit
        //
        let file_template = BT::Menu(
            LMENU_FILE::NAME.to_string(),
            false,
            vec![
                BT::MenuItem(LMENU_FILE::CHILD[0].to_string(), false,
                             Box::new(move |a,s,_e,_d| App::cmd_open_file(a, s))
                ),
                BT::MenuItem(LMENU_FILE::CHILD[1].to_string(), false,
                             Box::new(move |a,s,_e,_d| App::cmd_close_file(a, s))
                ),
                BT::MenuSeparator,
                BT::MenuItem(LMENU_FILE::CHILD[2].to_string(), false,
                             Box::new(move |a,_s,_e,_d| App::cmd_exit(a))
                ),
            ]
        );

        // Edit menu template
        //
        // Edit
        //   Find
        //   ----
        //   Preferences
        //
        let edit_template = BT::Menu(
            LMENU_EDIT::NAME.to_string(),
            false,
            vec![
                BT::MenuItem(LMENU_EDIT::CHILD[0].to_string(), false,
                             Box::new(move |a,_s,_e,d| App::cmd_find(a, d))
                ),
                BT::MenuSeparator,
                BT::MenuItem(LMENU_EDIT::CHILD[1].to_string(), false,
                             Box::new(move |a,s,_e,d| App::cmd_preferences(a, s, d))
                ),
            ]
        );

        // Help menu template
        //
        // Help
        //   About
        //
        let help_template = BT::Menu(
            LMENU_HELP::NAME.to_string(),
            false,
            vec![
                BT::MenuItem(LMENU_HELP::CHILD[0].to_string(), false,
                             Box::new(move |a,_s,_e,_d| App::cmd_about(a))
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

    pub fn find_menu_by_handle<'a>(root: &'a Vec<Box<dyn Component + 'static>>, handle: &nwg::ControlHandle) -> Option<&'a Box<dyn Component + 'static>> {
        // Iterate recursively through the menus to locate the handle
        for menu in root.iter() {
            match menu.children() {
                Some(children) => return App::find_menu_by_handle(children, handle),
                None if &menu.handle() == &handle => return Some(menu),
                _ => ()
            }
        }

        None
    }
}