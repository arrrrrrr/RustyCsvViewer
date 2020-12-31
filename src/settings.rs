//! The settings module is used to load and store UI geometry values and well
//! as other state that should persist between sessions
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::{fmt,env};
use std::path::{PathBuf};
use serde::{Deserialize,Serialize};

use crate::utils::geometry::Coord;

macro_rules! make_error {
    { $t:ident, $e:expr } => {
        Err(AppSettingsError::$t(format!("Info: {:?}", $e).to_owned()))
    }
}

/// Generic JSON and IO error types that can occur when interacting with the settings store
#[derive(Debug,PartialEq)]
pub enum AppSettingsError {
    IOError(String),
    SerializationError(String),
    DeserializationError(String),
}

/// Implement custom error messages
impl fmt::Display for AppSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result<> {
        match self {
            AppSettingsError::IOError(s) => {
                write!(f, "Unable to open settings or IO error. {}", s)
            },
            AppSettingsError::SerializationError(s) => {
                write!(f, "JSON serialization error writing settings. {}", s)
            },
            AppSettingsError::DeserializationError(s) => {
                write!(f, "JSON deserialization error reading settings. {}", s)
            }
        }
    }
}

/// Default values for the AppSetting structure
pub struct CAppSettings {}

impl CAppSettings {
    pub const DEF_WINDOW_POS: Coord<i32> = Coord { x: 300, y: 300 };
    pub const DEF_WINDOW_DIMS: Coord<i32> = Coord { x: 400, y: 300 };
    pub const DEF_CFG_PATH: &'static str = "settings.json";

    pub fn DEF_OPEN_FOLDER() -> String {
        let mut p = PathBuf::from(env::var_os("USERPROFILE").unwrap());
        p.push("Documents");
        p.to_str().unwrap().to_owned()
    }
}

/// Structure to store persistent UI state between sessions
#[derive(Debug,Deserialize,Serialize,PartialEq,Default)]
pub struct AppSettings {
    /// Window position relative to (0,0) at top left
    pub window_pos: Coord<i32>,
    /// window dimensions
    pub window_dims: Coord<i32>,
    /// list of recently open file paths
    pub recent_files: Vec<String>,
    pub last_seen_folder: String,
}

/// Implementation for AppSettings class
impl AppSettings {
    /// Construct AppSettings with default values
    fn new() -> Self {
        AppSettings {
            window_pos: CAppSettings::DEF_WINDOW_POS,
            window_dims: CAppSettings::DEF_WINDOW_DIMS,
            recent_files: vec![],
            last_seen_folder: CAppSettings::DEF_OPEN_FOLDER(),
        }
    }

    /// Attempt to load the settings from settings.json or otherwise return default values
    pub fn load() -> Result<AppSettings,AppSettingsError> {
        let mut app_settings = AppSettings::new();

        if let Ok(f) = File::open(CAppSettings::DEF_CFG_PATH) {
            let br = BufReader::new(f);
            let settings = serde_json::from_reader(br);

            if let Err(e) = settings {
                return make_error!(DeserializationError, e.to_string());
            }

            app_settings = settings.unwrap();
        }
        else {
            eprintln!("Warning: AppSettings::load() error. File::open failed");
        }

        Ok(app_settings)
    }

    /// Save the settings into the settings file
    /// which by default is the same directory as the executable
    pub fn save(&self) -> Result<(),AppSettingsError> {
        // TODO: This is intended to be called on exit.
        //       Panic would potentially leave things inconsistent, but is it ok to ignore errors?
        match File::create(CAppSettings::DEF_CFG_PATH) {
            Ok(mut _f) => {
                let bw = BufWriter::new(_f);
                let r = serde_json::to_writer_pretty(bw, &self);

                if let Err(e) = r {
                    return make_error!(SerializationError, e.to_string());
                }

                Ok(())
            },
            Err(e) => {
                return make_error!(IOError, e.to_string());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    macro_rules! make_strvec {
        [ $($a:expr),+ ] => {
            vec![ $($a.to_owned()),+ ]
        }
    }

    #[test]
    fn test_serialize_app_setting() {
        let settings = AppSettings {
            window_pos: Coord { x: 400, y: 500 },
            window_dims: Coord { x: 1000, y: 500 },
            recent_files: make_strvec![
                "C:\\Temp\\data.csv",
                "C:\\Users\\user\\Documents\\grades.csv"
            ],
            last_seen_folder: "C:\\Users\\joe\\Documents".to_owned()
        };

        let r = serde_json::to_string(&settings).expect("serialization error");
        let expected_settings: AppSettings = serde_json::from_str(&r).expect("deserialization error");

        assert_eq!(&settings, &expected_settings);
    }

    #[test]
    fn test_deserialize_app_setting() {
        let s =
            r#"{
                "window_pos": {
                    "x": 400, "y": 500
                },
                "window_dims": {
                    "x": 300, "y": 1000
                },
                "recent_files": [
                    "C:\\temp\\new_data.csv",
                    "X:\\bigdata.csv"
                ],
                "last_seen_folder": "C:\\Users\\joe\\Documents"
            }"#;

        let expected = AppSettings {
            window_pos: Coord { x: 400, y: 500 },
            window_dims: Coord { x: 300, y: 1000 },
            recent_files: make_strvec![
                "C:\\temp\\new_data.csv",
                "X:\\bigdata.csv"
            ],
            last_seen_folder: "C:\\Users\\joe\\Documents".to_owned()
        };

        let r: AppSettings = serde_json::from_str(s)
            .expect("deserialization error during settings read");

        assert_eq!(r, expected);
    }

    #[test]
    fn test_load_settings_no_settings_file() {
        let expected = AppSettings {
            window_pos: CAppSettings::DEF_WINDOW_POS,
            window_dims: CAppSettings::DEF_WINDOW_DIMS,
            recent_files: vec![],
            last_seen_folder: CAppSettings::DEF_OPEN_FOLDER.clone()
        };

        match AppSettings::load() {
            Ok(r) => assert_eq!(r, expected),
            Err(e) => panic!("{:?}", e)
        }
    }

    fn setup_create_settings_file() {
        let s = AppSettings {
            window_pos: Coord { x: 0, y: 2000 },
            window_dims: Coord { x: 1000, y: 1000 },
            recent_files: make_strvec![ "X:\\secrets.csv" ],
            last_seen_folder: CAppSettings::DEF_OPEN_FOLDER()
        };

        let f = File::create(Path::new(CAppSettings::DEF_CFG_PATH))
            .expect("failed to open file for write");
        let bw = BufWriter::new(f);

        serde_json::to_writer(bw, &s)
            .expect("serialization error during settings write");
    }

    fn teardown_remove_settings_file() {
        let p = Path::new(CAppSettings::DEF_CFG_PATH);
        std::fs::remove_file(p).expect("failed to delete settings file");
    }

    #[test]
    fn test_load_settings_with_settings_file() {
        setup_create_settings_file();

        let mut r: AppSettings = AppSettings::load().expect("load failed");

        r.window_pos.x = 1234;
        r.window_pos.y = 2200;
        r.window_dims.x = 100;
        r.window_dims.y = 150;
        r.recent_files.clear();
        r.recent_files.push(String::from("G:\\Path\\To\\Hidden\\Treasure.csv"));

        r.save().expect("saving failed");

        let f = File::open(CAppSettings::DEF_CFG_PATH).expect("open settings failed");
        let br = BufReader::new(f);

        let r2: AppSettings = serde_json::from_reader(br).expect("deserializing failed");
        let expected = AppSettings {
            window_pos: Coord { x: 1234, y: 2200 },
            window_dims: Coord { x: 100, y: 150 },
            recent_files: make_strvec![ "G:\\Path\\To\\Hidden\\Treasure.csv" ],
            last_seen_folder: CAppSettings::DEF_OPEN_FOLDER()
        };

        assert_eq!(r2, expected);

        teardown_remove_settings_file();
    }
}