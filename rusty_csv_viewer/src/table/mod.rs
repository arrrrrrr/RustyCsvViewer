// Bring these into the table namespace
pub use data::*;
pub use reader::{from_csv_file, from_tsv_file};

mod reader;
mod data;