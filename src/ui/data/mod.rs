/// This file contains data structures necessary to communicate between UI Components
/// These include Windows, Menus and Controls

pub enum CallbackContext {
    OpenFile(),

}

pub enum CallbackResult {
    OpenFile(String)
}