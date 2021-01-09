macro_rules! menu_label_const {
    (
        [ $a:literal | $b:literal $(,$c:literal)* ] @ $count:expr
    ) => {
            pub const NAME: &'static str = $a;
            pub const CHILD: [&'static str; $count] = [ $b $(,$c)* ];
    };
}

/// This is the resource file where any static app values are stored
pub const APP_TITLE: &str = "Rusty CSV Viewer";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_ABOUT: &str = "Copyright 2020";

pub const APP_OPEN_FILE_DLG: &str = "Open a CSV file";
pub const APP_OPEN_FILE_DLG_FILTER: &str = "CSV(*.csv)|Text(*.txt)|All Files(*.*)";

pub struct LMENU_FILE {}
impl LMENU_FILE {
    menu_label_const![ ["&File" | "&Open File", "&Close File", "E&xit"]@3 ];
}

pub struct LMENU_EDIT {}
impl LMENU_EDIT {
    menu_label_const![ ["&Edit" | "&Find", "&Preferences"] @2 ];
}

pub struct LMENU_HELP {}
impl LMENU_HELP {
    menu_label_const![ ["&Help" | "&About"] @1 ];
}