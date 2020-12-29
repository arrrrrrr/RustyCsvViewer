mod ui;

use nwg::NativeUi;
use ui::app::App;

fn main() {
    nwg::init().expect("Failed to initialize window");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _ui = App::build_ui(Default::default()).expect("Failed to create UI");
    nwg::dispatch_thread_events();
}
