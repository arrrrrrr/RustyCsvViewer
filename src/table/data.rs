use std::cmp;
use std::error;
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub struct TableData {
    pub header: Vec<String>,
    pub data: Vec<String>,
    pub dims: (usize, usize),
}

impl TableData {
    pub fn new() -> Self {
        TableData {
            header: Vec::new(),
            data: Vec::new(),
            dims: (0, 0),
        }
    }

    fn set_dims(&mut self, cols: usize, rows: usize) {
        self.dims = (cols, rows)
    }

    pub fn set_header(&mut self, header: &mut Vec<String>) {
        if (self.columns() > 0 && header.len() == self.columns()) ||
            self.columns() == 0
        {
            self.header.clear();
            self.header.append(header);
            self.set_dims(self.header.len(), self.rows());

            return;
        }

        panic!("TableData: column mismatch when attempting to update the header field")
    }

    pub fn set_data(&mut self, data: &mut Vec<String>, cols: usize) {
        if (self.columns() > 0 && cols == self.columns()) || self.columns() == 0
        {
            self.data.append(data);
            self.set_dims(cols, self.data.len() / cols);
            return;
        }

        panic!("TableData: column mismatch when attempting to update the data field")
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

    pub fn header(&self) -> &Vec<String> {
        &self.header
    }

    pub fn data(&self) -> &Vec<String> {
        &self.data
    }
}

/// Csv validation error sub-types
/// InvalidEscapeError
#[derive(Debug,PartialEq)]
pub enum QuoteValidationError {
    InvalidEscapeError,
    InvalidQuoteError,
    UnterminatedQuoteError,
}

impl Display for QuoteValidationError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            QuoteValidationError::InvalidEscapeError =>
                write!(f, "Unquoted field with escaped quote error"),

            QuoteValidationError::InvalidQuoteError =>
                write!(f, "Unbalanced quote error"),

            QuoteValidationError::UnterminatedQuoteError =>
                write!(f, "Unterminated outer quote error"),
        }
    }
}

/// Primary csv validation error types
#[derive(Debug,PartialEq)]
pub enum TableDataValidationError {
    QuoteValidationError {
        subtype: QuoteValidationError,
        row: i32,
        col: i32,
        value: String
    },
    RowFieldCountMismatchError {
        row: i32,
        expected:
        usize,
        found: usize
    },
}


/// Display trait for displaying Validation error messages
impl Display for TableDataValidationError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            TableDataValidationError::QuoteValidationError {
                subtype, row, col, value } =>
                {
                    let max_value_len = cmp::min(64, value.len());
                    write!(f, "At row {}. {} in column: {}, value: {}",
                           row, subtype, col, &value[0..max_value_len])
                }

            TableDataValidationError::RowFieldCountMismatchError {
                row, expected, found } =>
                write!(f, "At row {}. Field count mismatch. Expected: {}, Found: {}",
                       row, expected, found),
        }
    }
}

impl error::Error for TableDataValidationError {}

#[cfg(test)]
mod tests {
    use crate::table::*;

    macro_rules! make_strvec {
        [ $($a:expr),+ ] => {
            vec![ $($a.to_owned()),+ ]
        }
    }

    #[test]
    fn test_table_data_header_only() {
        let mut hdr = make_strvec![ "Name", "Type", "Value" ];
        let mut c = TableData::new();
        c.set_header(&mut hdr);

        assert_eq!(c.columns(), 3);
        assert_eq!(c.rows(), 0);
        assert_eq!(c.len(), 0);
        assert_eq!(c.has_headers(), true);
        assert_eq!(c.has_data(), false);
    }

    #[test]
    fn test_table_data_data_only() {
        let mut data = make_strvec![ "Name", "Type", "Value" ];
        let mut c = TableData::new();
        c.set_data(&mut data, 3);

        assert_eq!(c.columns(), 3);
        assert_eq!(c.rows(), 1);
        assert_eq!(c.len(), 3);
        assert_eq!(c.has_headers(), false);
        assert_eq!(c.has_data(), true);
    }

    #[test]
    fn test_table_data_header_and_data() {
        let mut hdr = make_strvec![ "Name", "Type", "Value" ];
        let mut data = make_strvec![ "a", "b", "c" ];
        let mut c = TableData::new();

        c.set_header(&mut hdr);
        c.set_data(&mut data, 3);

        assert_eq!(c.columns(), 3);
        assert_eq!(c.rows(), 1);
        assert_eq!(c.len(), 3);
        assert_eq!(c.has_headers(), true);
        assert_eq!(c.has_data(), true);
    }
}