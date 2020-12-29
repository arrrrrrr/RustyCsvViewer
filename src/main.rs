mod ui;
mod csv;
mod utils;
mod settings;

use nwg::NativeUi;
use ui::app::App;
use crate::settings::AppSettings;

fn main() {
    nwg::init().expect("Failed to initialize window");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let app_state = AppSettings::load();

    match app_state {
        Ok(s) => {
            let app = App::new(s);
            let _ui = App::build_ui(app).expect("Failed to create UI");
            nwg::dispatch_thread_events();
        },
        Err(e) => {
            panic!("{}", e)
        }
    }
}
