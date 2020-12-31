use std::path::{PathBuf};
use std::collections::HashMap;

use crate::ui::resource;
use crate::settings::AppSettings;
use crate::csv::*;
use crate::ui::menu::{CMenu};

#[derive(Default)]
pub struct App {
    pub window: nwg::Window,
    pub layout: nwg::GridLayout,
    pub file_dialog: nwg::FileDialog,
    pub layout_params: Option<LayoutParams>,
    pub state: AppSettings,
    pub menu: HashMap<String, CMenu>,
}

pub struct LayoutParams {
    data: CsvData,
    col_widths: Vec<usize>,
    row_height: usize,
}

impl App {
    pub fn new(state: AppSettings) -> Self {
        App {
            state,
            // TODO: might fail if UCS2 sequence cannot be converted to UTF8
            ..Default::default()
        }
    }

    /// On Event Handler Functions
    pub fn on_window_close(&self) {
        // TODO: this isn't ideal but its ok for now
        if let Err(e) =  self.state.save() {
            eprintln!("{}", e);
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
    pub fn cmd_open_file(&self) -> Option<String> {
        let selected = self.open_file_picker_dialog(&self.file_dialog);

        if selected.is_none() {
            return None;
        }

        let selected = selected.unwrap();
        eprintln!("Selected file: {}", selected);

        return Some(selected);

        // TODO: Implement further
        // if let Some(data) = self.read_file(&selected) {
        //     let l = self.prepare_layout(data);
        //     println!("{:?}", l.data)
        // }
    }

    // Execute the about command
    pub fn cmd_about(&self) {
        eprintln!("cmd_about: showing about dialog");
    }

    // Execute the close file command
    pub fn cmd_close_file(&self) {
        eprintln!("cmd_close_file: Closing open file");
    }

    pub fn cmd_exit(&self) {
        eprintln!("cmd_exit: exiting");
        self.on_window_close();
    }

    pub fn cmd_find(&self) {
        eprintln!("cmd_find: showing find dialog");
    }

    fn open_file_picker_dialog(&self, dialog: &nwg::FileDialog) -> Option<String> {
        let mut selected: Option<String> = None;

        // Run the file picker dialog and select a file
        if dialog.run(Some(&self.window)) {
            selected = match dialog.get_selected_item() {
                Ok(filepath) => Some(filepath),
                Err(e) => {
                    let msg_ = format!("{}", e);
                    nwg::error_message("Open File", &msg_);
                    None
                }
            }
        }

        selected
    }

    // Read the file contents into a CsvData structure or display a message box on error
    fn read_file(&self, filename: &str) -> Option<CsvData> {
        let msg= move |content| nwg::fatal_message("Open File", format!("{}", content).as_str());

        // Map error types into formatted strings to simplify display logic
        let data = reader::from_file(filename, false)
            .map_err(|e| e.to_string()).unwrap()
            .map_or_else(|e| { msg(e); None }, |v| Some(v));

        data
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
    fn prepare_layout(&self, data: CsvData) -> LayoutParams {
        //LayoutParams { data, col_widths: vec![], row_height: 1 }
        unimplemented!()
    }

    pub fn find_submenu_handle(&self, parent: &str, child: &str) ->
        Option<&nwg::ControlHandle>
    {
        if let Some(v) = self.menu.get(parent) {
            if let Some(v_) = v.get_submenu(child) {
                return Some(&v_.handle());
            }
            else {
                // return the top level menu handle
                return Some(&v.get_menu().handle());
            }
        }

        None
    }

    /// create a file picker dialog for opening csv and text files
    pub fn create_file_picker_dialog(dialog: &mut nwg::FileDialog) -> Result<(),nwg::NwgError> {
        nwg::FileDialog::builder()
            .title(resource::APP_OPEN_FILE_DLG)
            .action(nwg::FileDialogAction::Open)
            .filters(resource::APP_OPEN_FILE_DLG_FILTER)
            .build(dialog)?;

        Ok(())
    }

    pub fn create_menus<C: Into<nwg::ControlHandle> + Copy>(
        menu: &mut HashMap<String, CMenu>,
        parent: C)
    {
        use crate::ui::menu::{BulkMenuBuilder, TMenu, IMenu};
        use crate::ui::resource::*;

        let mut bmb = BulkMenuBuilder::new();

        let mut submenu_vec =
            (0..LMENU_FILE::HAS.len()).into_iter()
                .map(|_|IMenu::from(nwg::MenuItem::default()))
                .collect::<Vec<IMenu>>();

        let mut file_cnt = CMenu::new(
            IMenu::Menu(nwg::Menu::default()),
            &mut Vec::from(resource::LMENU_FILE::HAS),
            submenu_vec
        );

        bmb.add_menu(TMenu::Menu(LMENU_FILE::IS.to_owned(), false))
            .add_submenu_item(TMenu::MenuItem(LMENU_FILE::HAS[0].to_owned(), false, false))
            .add_submenu_item(TMenu::MenuItem(LMENU_FILE::HAS[1].to_owned(), true, false))
            .add_submenu_item(TMenu::MenuItem(LMENU_FILE::HAS[2].to_owned(), false, false))
            .build(&mut file_cnt, parent)
            .map_err(|e| eprintln!("{:?}",e)).unwrap();

        bmb = BulkMenuBuilder::new();
        submenu_vec =
            (0..LMENU_EDIT::HAS.len()).into_iter()
                .map(|_| IMenu::from(nwg::MenuItem::default())).collect();

        let mut edit_cnt = CMenu::new(
            IMenu::from(nwg::Menu::default()),
            &mut Vec::from(LMENU_EDIT::HAS),
            submenu_vec
        );

        bmb.add_menu(TMenu::Menu(LMENU_EDIT::IS.to_owned(), false))
            .add_submenu_item(TMenu::MenuItem(LMENU_EDIT::HAS[0].to_owned(), false, false))
            .add_submenu_item(TMenu::MenuItem(LMENU_EDIT::HAS[1].to_owned(), true, false))
            .build(&mut edit_cnt, parent)
            .map_err(|e| eprintln!("{:?}",e)).unwrap();

        bmb = BulkMenuBuilder::new();
        submenu_vec =
            (0..LMENU_HELP::HAS.len()).into_iter()
                .map(|_| IMenu::from(nwg::MenuItem::default())).collect();

        let mut help_cnt = CMenu::new(
            IMenu::from(nwg::Menu::default()),
            &mut Vec::from(LMENU_HELP::HAS),
            submenu_vec
        );

        bmb.add_menu(TMenu::Menu(LMENU_HELP::IS.to_owned(), false))
            .add_submenu_item(TMenu::MenuItem(LMENU_HELP::HAS[0].to_owned(), false, false))
            .build(&mut help_cnt, parent)
            .map_err(|e| eprintln!("{:?}",e)).unwrap();

        menu.insert(resource::LMENU_FILE::IS.to_string(), file_cnt);
        menu.insert(resource::LMENU_EDIT::IS.to_string(), edit_cnt);
        menu.insert(resource::LMENU_HELP::IS.to_string(), help_cnt);
    }


}

