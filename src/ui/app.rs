use std::path::{PathBuf};

use crate::settings::AppSettings;
use crate::utils;
use crate::csv::*;

#[derive(Default)]
pub struct App {
    pub window: nwg::Window,
    pub layout: nwg::GridLayout,
    pub file_dialog: nwg::FileDialog,
    pub default_folder: String,
    pub layout_params: Option<LayoutParams>,
    pub menu: nwg::Menu,
    pub state: AppSettings,
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
            default_folder: utils::get_cwd_as_str(),
            ..Default::default()
        }
    }

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
    pub fn cmd_open_file(&mut self) {
        // default folder might have changed? Not sure if this remembers the path
        if let Err(e) = self.file_dialog.set_default_folder(&self.default_folder) {
            nwg::error_message("Open File", &format!("{}",e));
        }

        let mut selected_file = String::new();

        // Run the file picker dialog and select a file
        if self.file_dialog.run(Some(&self.window)) {
            match self.file_dialog.get_selected_item() {
                Ok(filepath) => {
                    // update the default folder
                    if let Some(p) = PathBuf::from(&filepath).parent() {
                        self.default_folder = p.to_string_lossy().to_string();
                    }

                    selected_file.push_str(&filepath);
                },
                Err(e) => {
                    let msg_ = format!("{}", e);
                    nwg::error_message("Open File", &msg_);
                    return;
                }
            }
        }

        eprintln!("Selected file: {}", selected_file);

        if let Some(data) = self.read_file(&selected_file) {
            let l = self.prepare_layout(data);
            println!("{:?}", l.data)
        }
    }

    // Execute the close file command
    pub fn cmd_close_file(&mut self) {
        unimplemented!()
    }

    pub fn cmd_exit() {
        unimplemented!()
    }

    // Execute the about command
    pub fn cmd_about(&self) {
        unimplemented!()
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

}

