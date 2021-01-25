//! The settings module is used to load and store UI geometry values and well
//! as other state that should persist between sessions
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::BoxedResult;
use crate::utils::Point;

/// Default values for the AppSetting structure
struct CSettings {}

impl CSettings {
    pub const DEF_WINDOW_POS: Point<i32> = Point { x: 300, y: 300 };
    pub const DEF_WINDOW_SIZE: Point<u32> = Point { x: 400, y: 300 };
    pub const DEF_CFG_PATH: &'static str = "settings.json";
    pub const DEF_MAX_RECENT_FILES: usize = 10;
}

/// Structure to store persistent UI state between sessions
#[derive(Debug,Deserialize,Serialize,PartialEq,Default)]
pub struct Settings {
    /// Window position relative to (0,0) at top left
    pub window_pos: Point<i32>,
    /// Window dimensions
    pub window_size: Point<u32>,
    /// Recently opened file paths
    pub recent_files: Vec<String>,
    /// Maximum number of recent files to store
    pub max_recent_files: usize,
}

/// Implementation for AppSettings class
impl Settings {
    /// Construct AppSettings with default values
    fn new() -> Self {
        Settings {
            window_pos: CSettings::DEF_WINDOW_POS,
            window_size: CSettings::DEF_WINDOW_SIZE,
            recent_files: vec![],
            max_recent_files: CSettings::DEF_MAX_RECENT_FILES,
        }
    }

    /// Attempt to load the settings from settings.json or otherwise return default values
    pub fn load(validate: bool) -> BoxedResult<Settings> {
        let mut settings= Settings::new();

        match File::open(CSettings::DEF_CFG_PATH) {
            Ok(f) => {
                let br = BufReader::new(f);
                let s = serde_json::from_reader(br);

                match s {
                    Ok(s) => { settings = s; }
                    Err(e) => eprintln!("{:?}", e)
                }

                if validate {
                    Settings::verify_recent_files(&mut settings.recent_files);
                }
            },
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }

        Ok(settings)
    }

    /// Save the settings into the settings file
    /// which by default is the same directory as the executable
    pub fn save(&self) -> BoxedResult<()> {
        // TODO: This is intended to be called on exit.
        //       Panic would potentially leave things inconsistent, but is it ok to ignore errors?
        let mut _f = File::create(CSettings::DEF_CFG_PATH)?;
        let bw = BufWriter::new(_f);
        serde_json::to_writer_pretty(bw, &self)?;

        Ok(())
    }

    /// Verify that the files in the recent files list are still valid files
    fn verify_recent_files(files: &mut Vec<String>) {
        *files = files.iter_mut().filter(|x| Path::new(x).is_file()).map(|x| x.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    macro_rules! make_strvec {
        [ $($a:expr),+ ]
            =>
        {
            vec![ $($a.to_owned()),+ ]
        }
    }

    #[test]
    fn test_serialize_app_setting() {
        let settings = Settings {
            window_pos: Point { x: 400, y: 500 },
            window_size: Point { x: 1000, y: 500 },
            recent_files: make_strvec![
                "C:\\Temp\\data.csv",
                "C:\\Users\\user\\Documents\\grades.csv"
            ],
            max_recent_files: 10,
        };

        let r = serde_json::to_string(&settings).expect("serialization error");
        let expected_settings: Settings = serde_json::from_str(&r).expect("deserialization error");

        assert_eq!(&settings, &expected_settings);
    }

    #[test]
    fn test_deserialize_app_setting() {
        let s =
            r#"{
                "window_pos": {
                    "x": 400, "y": 500
                },
                "window_size": {
                    "x": 300, "y": 1000
                },
                "recent_files": [
                    "C:\\temp\\new_data.csv",
                    "X:\\bigdata.csv"
                ],
                "max_recent_files": 10
            }"#;

        let expected = Settings {
            window_pos: Point { x: 400, y: 500 },
            window_size: Point { x: 300, y: 1000 },
            recent_files: make_strvec![
                "C:\\temp\\new_data.csv",
                "X:\\bigdata.csv"
            ],
            max_recent_files: 10,
        };

        let r: Settings = serde_json::from_str(s)
            .expect("deserialization error during settings read");

        assert_eq!(r, expected);
    }

    #[test]
    fn test_load_settings_no_settings_file() {
        let expected = Settings {
            window_pos: CSettings::DEF_WINDOW_POS,
            window_size: CSettings::DEF_WINDOW_SIZE,
            recent_files: vec![],
            max_recent_files: 10,
        };

        match Settings::load(false) {
            Ok(r) => assert_eq!(r, expected),
            Err(e) => panic!("{:?}", e)
        }
    }

    fn setup_create_settings_file() {
        let s = Settings {
            window_pos: Point { x: 0, y: 2000 },
            window_size: Point { x: 1000, y: 1000 },
            recent_files: make_strvec![ "X:\\secrets.csv" ],
            max_recent_files: 10,
        };

        let f = File::create(Path::new(CSettings::DEF_CFG_PATH))
            .expect("failed to open file for write");
        let bw = BufWriter::new(f);

        serde_json::to_writer(bw, &s)
            .expect("serialization error during settings write");
    }

    fn teardown_remove_settings_file() {
        let p = Path::new(CSettings::DEF_CFG_PATH);
        std::fs::remove_file(p).expect("failed to delete settings file");
    }

    #[test]
    fn test_load_settings_with_settings_file() {
        setup_create_settings_file();

        let mut r: Settings = Settings::load(false).expect("load failed");

        r.window_pos.x = 1234;
        r.window_pos.y = 2200;
        r.window_size.x = 100;
        r.window_size.y = 150;
        r.recent_files.clear();
        r.recent_files.push(String::from("G:\\Path\\To\\Hidden\\Treasure.csv"));

        r.save().expect("saving failed");

        let f = File::open(CSettings::DEF_CFG_PATH).expect("open settings failed");
        let br = BufReader::new(f);

        let r2: Settings = serde_json::from_reader(br).expect("deserializing failed");
        let expected = Settings {
            window_pos: Point { x: 1234, y: 2200 },
            window_size: Point { x: 100, y: 150 },
            recent_files: make_strvec![ "G:\\Path\\To\\Hidden\\Treasure.csv" ],
            max_recent_files: 10,
        };

        assert_eq!(r2, expected);

        teardown_remove_settings_file();
    }
}