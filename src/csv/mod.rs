pub mod reader;

use std::fmt;
use std::cmp;

#[derive(Debug)]
pub struct CsvData {
    header: Vec<String>,
    data: Vec<String>,
    dims: (usize, usize),
}

impl CsvData {
    pub fn new() -> Self {
        CsvData {
            header: Vec::new(),
            data: Vec::new(),
            dims: (0, 0),
        }
    }

    pub fn has_headers(&self) -> bool {
        self.header.len() > 0
    }

    pub fn has_data(&self) -> bool {
        self.data.len() > 0
    }

    pub fn len(&self) -> usize {
        self.columns() * self.rows()
    }

    pub fn columns(&self) -> usize {
        self.dims.0
    }

    pub fn rows(&self) -> usize {
        self.dims.1
    }

    pub fn get_headers(&self) -> &Vec<String> {
        &self.header
    }

    pub fn get_data(&self) -> &Vec<String> {
        &self.data
    }

    pub fn set_dims(&mut self, cols: usize, rows: usize) {
        self.dims = (cols, rows)
    }

    pub fn set_header(&mut self, header: &mut Vec<String>) {
        if self.has_data() && header.len() == self.dims.0 {
            self.header.clear();
            self.header.append(header);
            return;
        }

        panic!("CsvData: column mismatch when attempting to update the header field")
    }

    pub fn set_data(&mut self, data: &mut Vec<String>, cols: usize) {
        if self.columns() > 0 && cols == self.columns() {
            self.data.append(data);
            self.set_dims(cols, self.data.len() / cols);
            return;
        }

        panic!("CsvData: column mismatch when attempting to update the data field")
    }
}

#[derive(Debug,PartialEq)]
pub enum CsvQuoteValidationError {
    InvalidQuoteError,
    InvalidEscapeError,
    UnterminatedQuoteError,
}

impl fmt::Display for CsvQuoteValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *&self {
            CsvQuoteValidationError::InvalidQuoteError =>
                write!(f, "Unbalanced quote error"),

            CsvQuoteValidationError::InvalidEscapeError =>
                write!(f, "Unquoted field with escaped quote error"),

            CsvQuoteValidationError::UnterminatedQuoteError =>
                write!(f, "Unterminated outer quote error"),
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum CsvValidationError {
    QuoteValidationError { subtype: CsvQuoteValidationError, row: i32, col: i32, value: String },
    RowFieldCountMismatchError { row: i32, expected: usize, found: usize },
}

impl fmt::Display for CsvValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *&self {
            CsvValidationError::QuoteValidationError {
                subtype, row, col, value } =>
                {
                    let max_value_len = cmp::min(64, value.len());
                    write!(f, "At row {}. {} in column: {}, value: {}",
                           row, subtype, col, &value[0..max_value_len])
                }

            CsvValidationError::RowFieldCountMismatchError {
                row, expected, found } =>
                write!(f, "At row {}. Field count mismatch. Expected: {}, Found: {}",
                       row, expected, found),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::path::Path;
    use std::io::Write;
    use std::fs;

    macro_rules! make_strvec {
        [ $($a:expr),+ ] => {
            vec![ $($a.to_owned()),+ ]
        }
    }

    #[test]
    fn test_csvdata_header_only() {
        let mut hdr = make_strvec![ "Name", "Type", "Value" ];
        let mut c = CsvData::new();
        c.set_header(&mut hdr);

        assert_eq!(c.columns(), 3);
        assert_eq!(c.rows(), 0);
        assert_eq!(c.len(), 0);
        assert_eq!(c.has_headers(), true);
        assert_eq!(c.has_data(), false);
    }

    #[test]
    fn test_csvdata_data_only() {
        let mut data = make_strvec![ "Name", "Type", "Value" ];
        let mut c = CsvData::new();
        c.set_data(&mut data, 3);

        assert_eq!(c.columns(), 3);
        assert_eq!(c.rows(), 1);
        assert_eq!(c.len(), 3);
        assert_eq!(c.has_headers(), false);
        assert_eq!(c.has_data(), true);
    }

    #[test]
    fn test_csvdata_header_and_data() {
        let mut hdr = make_strvec![ "Name", "Type", "Value" ];
        let mut data = make_strvec![ "a", "b", "c" ];
        let mut c = CsvData::new();

        c.set_header(&mut hdr);
        c.set_data(&mut data, 3);

        assert_eq!(c.columns(), 3);
        assert_eq!(c.rows(), 1);
        assert_eq!(c.len(), 3);
        assert_eq!(c.has_headers(), true);
        assert_eq!(c.has_data(), true);
    }
}