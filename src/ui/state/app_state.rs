use super::settings::{Settings};
use crate::table::TableData;
use crate::utils::Coord;
use crate::BoxedResult;

/// Stores the name of the file and its tabulated contents
pub struct OpenFileInfo {
    pub name: String,
    pub data: TableData,
}

/// Stores the Applications state
/// This is intended to separate the state from the application
pub struct AppState {
    settings: Settings,
    ofi: Option<OpenFileInfo>,
}

impl AppState {
    /// Return a new state object
    pub fn new(settings: Settings) -> Self {
        AppState {
            settings,
            ofi: None
        }
    }

    /// Get the stored window position
    pub fn window_pos(&self) -> (i32, i32) {
        (self.settings.window_pos.x, self.settings.window_pos.y)
    }

    /// Get the stored window size
    pub fn window_size(&self) -> (u32, u32) {
        (self.settings.window_size.x, self.settings.window_size.y)
    }

    /// Get the stored recent files list
    pub fn recent_files(&self) -> Vec<String> {
        Vec::clone(&self.settings.recent_files)
    }

    /// Get the stored maximum number of recent files to keep
    pub fn max_recent_files(&self) -> usize {
        self.settings.max_recent_files
    }

    /// Retrieve the file data
    pub fn file_data(&self) -> Option<&OpenFileInfo> {
        if let Some(ofi) = &self.ofi {
            return Some(ofi);
        }

        None
    }

    /// Set the stored window position
    pub fn set_window_pos(&mut self, pos: (i32, i32)) {
        match pos {
            (x,y) if x >= 0 && y >= 0 => {
                self.settings.window_pos = Coord { x, y }
            },
            _ => ()
        }
    }

    /// Set the stored window size
    pub fn set_window_size(&mut self, size: (u32,u32)) {
        self.settings.window_size = Coord { x: size.0, y: size.1 }
    }

    /// Set the stored number of maximum recent files
    pub fn set_max_recent_files(&mut self, limit: usize) {
        self.settings.max_recent_files = limit;
        self.settings.recent_files.truncate(limit);
    }

    /// Add a file to the recent files list restricted by max_recent_files
    pub fn add_recent_file(&mut self, filename: &str) {
        // If the file exists in the list
        if self.settings.recent_files.iter().any(|e| e == filename) {
            self.settings.recent_files.retain(|x| x != filename);
        }

        if self.settings.recent_files.len() < self.settings.max_recent_files {
            self.settings.recent_files.push(filename.to_string());
        }
    }

    /// Test whether there is file data loaded
    pub fn is_data_loaded(&self) -> bool {
        self.ofi.is_some()
    }

    /// Load file data
    /// Returns any previous file data as to not invalidate potential references
    pub fn load_data(&mut self, ofi: OpenFileInfo) -> Option<OpenFileInfo> {
        let mut ofi_ = Some(ofi);
        std::mem::swap(&mut self.ofi, &mut ofi_);
        ofi_
    }

    /// Unload file data
    /// Returns any previous file data as to not invalidate potential references
    pub fn unload_data(&mut self) -> Option<OpenFileInfo> {
        let mut ofi_ = None;
        std::mem::swap(&mut self.ofi, &mut ofi_);
        ofi_
    }

    /// Write the settings to the settings file
    pub fn write_settings(&self) -> BoxedResult<()> {
        self.settings.save()
    }
}