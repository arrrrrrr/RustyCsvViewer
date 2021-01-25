/// This file contains constants for application components

/// Generate the constants for a menu where:
/// NAME: Menu Title
/// CHILD: Menu Items
macro_rules! menu_label_const {
    (
        [ $a:literal | $b:literal $(,$c:literal)* ] @ $count:expr
    ) => {
            pub const NAME: &'static str = $a;
            pub const CHILD: [&'static str; $count] = [ $b $(,$c)* ];
    };
}

/// Structure that holds constants for details about the application
pub struct CAppAbout {}

impl CAppAbout {
    pub const NAME: &'static str = "Rusty CSV Viewer";
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
    pub const COPYRIGHT: &'static str = "Copyright © 2020-2021 arrrrr";
}

pub struct CDialogAboutApp {}

impl CDialogAboutApp {
    pub const WINDOW_SIZE: (i32, i32) = (400, 250);

}

/// Structure that holds constants for the Open File dialog
pub struct CDialogOpenFile {}

impl CDialogOpenFile {
    pub const TITLE: &'static str = "Open a CSV file";
    pub const FILTER: &'static str = "CSV(*.csv)|Text(*.txt)|All files(*.*)";
}

/// Structure that holds constants for the File Menu
pub struct CMenuFile {}
impl CMenuFile {
    menu_label_const![ ["&File" | "&Open file", "&Close file", "E&xit"]@3 ];
}

/// Structure that holds constants for the Edit Menu
pub struct CMenuEdit {}
impl CMenuEdit {
    menu_label_const![ ["&Edit" | "&Find", "&Preferences"] @2 ];
}

/// Structure that holds constants for the Help Menu
pub struct CMenuHelp {}
impl CMenuHelp {
    menu_label_const![ ["&Help" | "&About"] @1 ];
}