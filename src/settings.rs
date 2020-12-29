use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use serde::{Deserialize,Serialize};

#[derive(Debug,Deserialize,Serialize,PartialEq)]
pub struct Coord<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug,Deserialize,Serialize,PartialEq)]
pub struct AppSettings {
    window_pos: Coord<u32>,
    window_dims: Coord<u32>,
    recent_files: Vec<String>,
}

pub struct CAppSettings {}

impl CAppSettings {
    pub const DEF_WINDOW_POS: Coord<u32> = Coord { x: 300, y: 300 };
    pub const DEF_WINDOW_DIMS: Coord<u32> = Coord { x: 400, y: 300 };
    pub const DEF_CFG_PATH: &'static str = "settings.json";
}

impl AppSettings {
    fn new() -> Self {
        AppSettings {
            window_pos: CAppSettings::DEF_WINDOW_POS,
            window_dims: CAppSettings::DEF_WINDOW_DIMS,
            recent_files: vec![],
        }
    }

    pub fn load() -> Result<AppSettings,Box<dyn Error>> {
        // If the config file has not yet been written, it will be saved during cleanup
        if !Path::new(CAppSettings::DEF_CFG_PATH).exists() {
            return Ok(AppSettings::new());
        }

        let f = File::open(CAppSettings::DEF_CFG_PATH)?;
        let br = BufReader::new(f);

        // deserialize the json into the AppSettings struct
        let settings = serde_json::from_reader(br)?;
        Ok(settings)
    }

    pub fn save(&self) -> Result<(),Box<dyn Error>> {
        let f = File::create(CAppSettings::DEF_CFG_PATH)?;
        let bw = BufWriter::new(f);

        // serialize the AppSettings struct into json
        serde_json::to_writer_pretty(bw, &self)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            ]
        };

        let r = serde_json::to_string(&settings).unwrap();
        let expected_settings: AppSettings = serde_json::from_str(&r).unwrap();
        assert_eq!(&settings, &expected_settings);
        println!("{}", r);
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
                ]
            }"#;

        let expected = AppSettings {
            window_pos: Coord { x: 400, y: 500 },
            window_dims: Coord { x: 300, y: 1000 },
            recent_files: make_strvec![
                "C:\\temp\\new_data.csv",
                "X:\\bigdata.csv"
            ]
        };

        let r: AppSettings = serde_json::from_str(s).unwrap();
        assert_eq!(r, expected);
    }

    #[test]
    fn test_load_settings_no_settings_file() {
        let r = AppSettings::load().unwrap();
        let expected = AppSettings {
            window_pos: CAppSettings::DEF_WINDOW_POS,
            window_dims: CAppSettings::DEF_WINDOW_DIMS,
            recent_files: vec![],
        };

        assert_eq!(r, expected);
    }

    fn setup_create_settings_file() {
        let f = File::create(Path::new(CAppSettings::DEF_CFG_PATH)).unwrap();
        let bw = BufWriter::new(f);

        let r = AppSettings {
            window_pos: Coord { x: 0, y: 2000 },
            window_dims: Coord { x: 1000, y: 1000 },
            recent_files: make_strvec![ "X:\\secrets.csv" ],
        };

        serde_json::to_writer(bw, &r).unwrap();
    }

    fn teardown_remove_settings_file() {
        let p = Path::new(CAppSettings::DEF_CFG_PATH);
        let f = std::fs::remove_file(p).unwrap();
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

        let mut f = File::open(CAppSettings::DEF_CFG_PATH).expect("open settings failed");
        let mut br = BufReader::new(f);

        let r2: AppSettings = serde_json::from_reader(br).expect("deserializing failed");
        let expected = AppSettings {
            window_pos: Coord { x: 1234, y: 2200 },
            window_dims: Coord { x: 100, y: 150 },
            recent_files: make_strvec![ "G:\\Path\\To\\Hidden\\Treasure.csv" ]
        };

        assert_eq!(r2, expected);

        teardown_remove_settings_file();
    }
}