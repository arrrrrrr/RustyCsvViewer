mod ui;
mod csv;
mod utils;
mod settings;
// mod settings;

use nwg::NativeUi;
use ui::app::App;

fn main() {
    nwg::init().expect("Failed to initialize window");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let app = App::new();
    let _ui = App::build_ui(app).expect("Failed to create UI");
    nwg::dispatch_thread_events();
}
