use std::sync::{Arc,Mutex};

use rcv::{App,AppState,NativeUiEx,Settings};

fn main() {
    nwg::init().expect("Failed to initialize window");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    match Settings::load(true) {
        Ok(s) => {
            // Store the app state in a ref-counted mutex in case we use threads later
            let app_state = Arc::new(Mutex::new(AppState::new(s)));
            // Build the App's ui
            let _ui = App::build_ui(App::new(), Arc::clone(&app_state))
                        .expect("Failed to create UI");
            // State the window message loop
            nwg::dispatch_thread_events();
        },
        Err(e) => {
            // Error loading the settings
            panic!("{}", e)
        }
    }
}