use std::io;
use std::io::Read;
use std::fs::File;
use std::vec::Vec;
use std::fmt;
use std::cmp::min;

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
        self.header.append(header)
    }

    pub fn set_data(&mut self, data: &mut Vec<String>) {
        self.data.append(data)
    }
}

type CsvResult<T> = Result<T,CsvValidationError>;

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
                    let max_value_len = min(64, value.len());
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

pub fn from_file(filename: &str, header: bool) -> io::Result<CsvResult<CsvData>> {
    let mut f = File::open(filename)?;

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    Ok(parse_csv(&buffer, header))
}

fn parse_csv(buffer: &str, header: bool) -> CsvResult<CsvData> {
    let mut csv_data = CsvData::new();
    let mut row_data: Vec<Vec<String>> = Vec::new();
    let mut v: Vec<String> = Vec::new();

    let mut header_processed = false;
    let mut inside_quote = false;
    let mut current_field = String::new();
    let mut num_fields: usize = 0;
    let mut buffer_pos: usize = 0;
    let mut row_id= 0;
    let buffer_len: usize = buffer.len();

    for mut c in buffer.chars() {
        buffer_pos += 1;

        if c != '\n' && c != '\r' {
            if inside_quote || c != ',' {
                current_field.push(c);
            }
        }

        // change state if the character is a quote
        inside_quote = if c == '"' { !inside_quote } else { inside_quote };

        // handle the case where there is no terminating newline
        if buffer_pos == buffer_len {
            c = '\n';
        }

        // only process a field or row when not inside a set of outer quotes
        if !inside_quote {
            // process the field. field either terminates in a comma or newline
            if c == ',' || c == '\n' {
                match validate_field(&current_field) {
                    Err(e) => {
                        return Err(CsvValidationError::QuoteValidationError {
                            subtype: e,
                            row: row_id + 1,
                            col: (v.len() + 1) as i32,
                            value: current_field
                        });
                    },
                    Ok(_) => {
                        if row_id == 0 {
                            num_fields += 1;
                        }
                        v.push(finalize_field(&current_field));
                        current_field = String::new();
                    }
                };
            }

            // process the row. row ends in a newline
            if c == '\n' {
                if num_fields != v.len() {
                    return Err(CsvValidationError::RowFieldCountMismatchError {
                        row: row_id + 1,
                        expected: num_fields,
                        found: v.len()
                    });
                }

                if header && !header_processed {
                    csv_data.set_header(&mut v);
                    header_processed = true;
                } else {
                    row_data.push(v);
                }

                v = Vec::new();
                row_id += 1;
            }
        }
    }

    // the parser might have not matched a set of quotes
    if inside_quote {
        return Err(CsvValidationError::QuoteValidationError {
            subtype: CsvQuoteValidationError::UnterminatedQuoteError,
            row: row_id + 1,
            col: (v.len() as i32) + 1,
            value: current_field
        });
    }

    // set the dimensions
    csv_data.set_dims(num_fields, row_data.len());
    // update the data field
    csv_data.set_data(&mut row_data.into_iter().flatten().collect::<Vec<String>>());

    Ok(csv_data)
}

fn validate_field(field: &str) -> Result<bool, CsvQuoteValidationError> {
    let has_outer_quotes = has_outer_quotes(&field);
    // extract the quote indices skipping the outer quotes
    let indices: Vec<usize> = field.chars().enumerate()
                                .filter(|(i,v)|
                                    { *v == '"' && (*i > 0 && *i < field.len()-1) })
                                .map(|(i,_)| i).collect();
    // number of quotes must be even
    if indices.len() % 2 > 0 {
        return Err(CsvQuoteValidationError::InvalidQuoteError);
    }
    // iterate over the indices as tuples of successive values - (0,1), (1,2), ...
    let iter = indices.iter().step_by(2).zip(indices.iter().skip(1).step_by(2));

    for (i,j) in iter {
        if j - i > 1 {
            return Err(CsvQuoteValidationError::InvalidQuoteError);
        }
        else if !has_outer_quotes {
            return Err(CsvQuoteValidationError::InvalidEscapeError);
        }
    }

    Ok(true)
}

fn finalize_field(field: &str) -> String {
    let mut finalized = String::from(field);

    // remove leading and trailing quotes
    if has_outer_quotes(&finalized) {
        finalized = finalized[1..finalized.len()-1].to_owned();
    }

    finalized.replace("\"\"", "\"")
}

fn has_outer_quotes(field: &str) -> bool {
    field.starts_with("\"") && field.ends_with("\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::io;
    use std::path::Path;

    // helpers for testing from_file(...)
    fn setup_from_file(target: &str, data: &str) -> io::Result<()> {
        let mut f = File::create(target)?;
        f.write_all(data.as_bytes())?;
        Ok(())
    }

    fn teardown_from_file(target: &str) -> io::Result<()> {
        std::fs::remove_file( Path::new(target))?;
        Ok(())
    }

    macro_rules! make_strvec {
        [ $($a:expr),+ ] => {
            vec![ $($a.to_owned()),+ ]
        }
    }

    #[test]
    fn test_csvdata_cols_rows_len() {
        let data = make_strvec![ "a", "b", "c" ];
        let c = CsvData { header: vec![], data, dims: (3,1)};

        assert_eq!(c.columns(), 3);
        assert_eq!(c.rows(), 1);
        assert_eq!(c.len(), 3);
        assert_eq!(c.has_headers(), false);
        assert_eq!(c.has_data(), true);
    }

    // tests
    #[test]
    fn test_validate_field_none() {
        let s = "abc";
        assert!(validate_field(&s).is_ok())
    }

    #[test]
    fn test_validate_field_outer_quotes_with_contents() {
        let s = "\"abc\"";
        assert!(validate_field(&s).is_ok())
    }

    #[test]
    fn test_validate_field_outer_quotes_empty() {
        let s = "\"\"";
        assert!(validate_field(&s).is_ok())
    }

    #[test]
    fn test_validate_field_invalid_escaped_quotes() {
        let s = "abc\"\"de";
        let e = CsvQuoteValidationError::InvalidEscapeError;
        assert_eq!(validate_field(&s).err().unwrap(), e);
    }

    #[test]
    fn test_validate_field_invalid_escaped_quotes2() {
        let s = "\"abc\"\"de";
        let e = CsvQuoteValidationError::InvalidEscapeError;
        assert_eq!(validate_field(&s).err().unwrap(), e);
    }

    #[test]
    fn test_validate_field_invalid_quotes_with_outer_single_quote() {
        let s = "\"\"\"";
        let e = CsvQuoteValidationError::InvalidQuoteError;
        assert_eq!(validate_field(&s).err().unwrap(), e);
    }

    #[test]
    fn test_validate_field_invalid_quotes_with_outer_with_many_single_quote() {
        let s = "\"abc\"de\"f\"";
        let e = CsvQuoteValidationError::InvalidQuoteError;
        assert_eq!(validate_field(&s).err().unwrap(), e);
    }

    #[test]
    fn test_validate_field_invalid_quotes_with_outer_with_inner_single_quote() {
        let s = "\"a\"bc\"";
        let e = CsvQuoteValidationError::InvalidQuoteError;
        assert_eq!(validate_field(&s).err().unwrap(), e);
    }

    #[test]
    fn test_validate_field_invalid_quotes_no_outer() {
        let s = "abc\"def";
        let e = CsvQuoteValidationError::InvalidQuoteError;
        assert_eq!(validate_field(&s).err().unwrap(), e);
    }

    #[test]
    fn test_validate_field_outer_quotes_with_one_valid_escape() {
        let s = "\"a\"\"bc\"";
        assert!(validate_field(&s).is_ok())
    }

    #[test]
    fn test_validate_field_outer_quotes_with_many_valid_escapes() {
        let s = "\"a\"\"bcd\"\"efg\"\"\"";
        assert!(validate_field(&s).is_ok())
    }

    #[test]
    fn test_has_outer_quotes_quoted() {
        let s = "\"abc\"";
        assert_eq!(has_outer_quotes(&s), true)
    }

    #[test]
    fn test_has_outer_quotes_only_quotes() {
        let s = "\"\"";
        assert_eq!(has_outer_quotes(&s), true)
    }

    #[test]
    fn test_has_outer_quotes_none() {
        let s = "a\"\"bc";
        assert_eq!(has_outer_quotes(&s), false)
    }

    #[test]
    fn test_finalize_field_outer_quotes() {
        let s = "\"this is a value\"";
        assert_eq!(finalize_field(&s), "this is a value")
    }

    #[test]
    fn test_finalize_field_escaped_quotes() {
        let s = "\"this is a \"\"value\"\" that is quoted\"";
        assert_eq!(finalize_field(&s), "this is a \"value\" that is quoted")
    }

    #[test]
    fn test_finalize_field_escaped_quotes2() {
        let s = "\"this is a \"\"\"\"value\"\" that\"\" is quoted\"";
        assert_eq!(finalize_field(&s), "this is a \"\"value\" that\" is quoted")
    }

    #[test]
    fn test_finalize_field_no_quotes() {
        let s = "this is a string without quotes";
        assert_eq!(finalize_field(&s), "this is a string without quotes")
    }

    #[test]
    fn test_finalize_field_only_quotes() {
        let s = "\"\"";
        assert_eq!(finalize_field(&s), "")
    }

    #[test]
    fn test_parse_csv_header_only_no_lf() {
        let s = "Name,Type,Value";
        let r = parse_csv(&s, true);

        let expected = CsvData {
            header: make_strvec![ "Name", "Type", "Value" ],
            data: vec![],
            dims: (3,0),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_only_lf() {
        let s = "Name,Type,Value\n";
        let r = parse_csv(&s, true);

        let expected = CsvData {
            header: make_strvec![ "Name", "Type", "Value" ],
            data: vec![],
            dims: (3,0),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_only_crlf() {
        let s = "Name,Type,Value\r\n";
        let r = parse_csv(&s, true);

        let expected = CsvData {
            header: make_strvec![ "Name", "Type", "Value" ],
            data: vec![],
            dims: (3,0),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_no_header_no_lf() {
        let s = "value1,value2,this is a value";
        let r = parse_csv(&s, false);

        let expected = CsvData {
            header: vec![],
            data: make_strvec![ "value1", "value2", "this is a value" ],
            dims: (3,1),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_no_header_lf() {
        let s = "value1,value2,this is a value\n";
        let r = parse_csv(&s, false);

        let expected = CsvData {
            header: vec![],
            data: make_strvec![ "value1", "value2", "this is a value" ],
            dims: (3,1),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_no_header_crlf() {
        let s = "value1,value2,this is a value\r\n";
        let r = parse_csv(&s, false);

        let expected = CsvData {
            header: vec![],
            data: make_strvec![ "value1", "value2", "this is a value" ],
            dims: (3,1),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_no_header_multiple_rows_trailing_lf() {
        let s =
            "value1,value2,this is a value\nvalue3,value4,another value\nvalue5,value6,yet another value\n";
        let r = parse_csv(&s, false);

        let expected = CsvData {
            header: vec![],
            data: make_strvec![ "value1", "value2", "this is a value",
                                "value3", "value4", "another value",
                                "value5", "value6", "yet another value" ],
            dims: (3,3),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_no_header_multiple_rows_no_trailing_lf() {
        let s =
            "value1,value2,this is a value\nvalue3,value4,another value\nvalue5,value6,yet another value";
        let r = parse_csv(&s, false);

        let expected = CsvData {
            header: vec![],
            data: make_strvec![ "value1", "value2", "this is a value",
                                "value3", "value4", "another value",
                                "value5", "value6", "yet another value" ],
            dims: (3,3),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_data() {
        let s = "Name,Type,Value\nvalue1,int,30\n";
        let r = parse_csv(&s, true);

        let expected = CsvData {
            header: make_strvec![ "Name", "Type", "Value" ],
            data: make_strvec![ "value1", "int", "30" ],
            dims: (3, 1),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_data_no_trailing_lf() {
        let s = "Name,Type,Value\nvalue1,int,30";
        let r = parse_csv(&s, true);

        let expected = CsvData {
            header: make_strvec![ "Name", "Type", "Value" ],
            data: make_strvec![ "value1", "int", "30" ],
            dims: (3, 1),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_data_multiple_rows_no_trailing_lf() {
        let s = "Name,Type,Value\nvalue1,int,30\nvalue2,string,this is a value";
        let r = parse_csv(&s, true);

        let expected = CsvData {
            header: make_strvec![ "Name", "Type", "Value" ],
            data: make_strvec![ "value1", "int", "30",
                                "value2", "string", "this is a value" ],
            dims: (3,2),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_data_multiple_rows_trailing_lf() {
        let s = "Name,Type,Value\nvalue1,int,30\nvalue2,string,this is a value\n";
        let r = parse_csv(&s, true);

        let expected = CsvData {
            header: make_strvec![ "Name", "Type", "Value" ],
            data: make_strvec![ "value1", "int", "30",
                                "value2", "string", "this is a value" ],
            dims: (3,2),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_data_multiple_rows_quoted_string_trailing_lf() {
        let s = "Name,Type,Value\nvalue1,int,30\nvalue2,string,\"this is a value\"\n";
        let r = parse_csv(&s, true);

        let expected = CsvData {
            header: make_strvec![ "Name", "Type", "Value" ],
            data: make_strvec![ "value1", "int", "30",
                                "value2", "string", "this is a value" ],
            dims: (3,2),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_data_quoted_string_has_newline() {
        let s = "Name,Type,Value\nvalue1,string,\"this\nis a value\"";
        let r = parse_csv(&s, true);

        let expected = CsvData {
            header: make_strvec![ "Name", "Type", "Value" ],
            data: make_strvec![ "value1", "string", "thisis a value" ],
            dims: (3, 1),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_data_escaped_quoted_string() {
        let s = "Name,Type,Value\nvalue1,string,\"this \"\"is a value\"";
        let r = parse_csv(&s, true);

        let expected = CsvData {
            header: make_strvec![ "Name", "Type", "Value" ],
            data: make_strvec![ "value1", "string", "this \"is a value" ],
            dims: (3,1),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_data_invalid_row_lengths() {
        let s = "Name,Type,Value\nvalue1,string";
        let r = parse_csv(&s, true);
        let e = CsvValidationError::RowFieldCountMismatchError { row: 2, expected: 3, found: 2};

        assert_eq!(r.err().unwrap(), e);
    }

    #[test]
    fn test_parse_csv_header_data_invalid_row_lengths2() {
        let s = "Name,Type,Value\nvalue1,string\nvalue2,int,30";
        let r = parse_csv(&s, true);
        let e = CsvValidationError::RowFieldCountMismatchError { row: 2, expected: 3, found: 2};

        assert_eq!(r.err().unwrap(), e);
    }

    #[test]
    fn test_parse_csv_header_data_invalid_row_lengths3() {
        let s = "Name,Type\nvalue1,string,abc";
        let r = parse_csv(&s, true);
        let e = CsvValidationError::RowFieldCountMismatchError { row: 2, expected: 2, found: 3};

        assert_eq!(r.err().unwrap(), e);
    }

    #[test]
    fn test_parse_csv_header_data_invalid_quotes() {
        let s = "Name,Type,Value\nvalue1,string,a\"\"bc";
        let r = parse_csv(&s, true);

        let e = CsvValidationError::QuoteValidationError {
            subtype: CsvQuoteValidationError::InvalidEscapeError,
            row: 2, col: 3, value: String::from("a\"\"bc") };

        assert_eq!(r.err().unwrap(), e);
    }

    #[test]
    fn test_parse_csv_header_data_invalid_quotes2() {
        let s = "Name,Type,Value\nvalue1,string,\"a\"bc\"";
        let r = parse_csv(&s, true);

        let e = CsvValidationError::QuoteValidationError {
            subtype: CsvQuoteValidationError::UnterminatedQuoteError,
            row: 2, col: 3, value: String::from("\"a\"bc\"") };

        assert_eq!(r.err().unwrap(), e);
    }

    #[test]
    fn test_parse_csv_header_data_invalid_quotes3() {
        let s = "Name,Type,Value\n\"value1,string,abc";
        let r = parse_csv(&s, true);

        let e = CsvValidationError::QuoteValidationError {
            subtype: CsvQuoteValidationError::UnterminatedQuoteError,
            row: 2, col: 1, value: String::from("\"value1,string,abc") };

        assert_eq!(r.err().unwrap(), e);
    }

    #[test]
    fn test_parse_csv_header_data_invalid_quotes3_msg() {
        let s = "Name,Type,Value\n\"value1,string,abc";
        let r = parse_csv(&s, true);

        let m =
            "At row 2. Unterminated outer quote error \
            in column: 1, value: \"value1,string,abc";

        assert_eq!(r.err().map(|e| format!("{}",e)).unwrap(), m);
    }

    #[test]
    fn test_from_file_valid_data() {
        let s =
            "Name,Value,Type\n\
            value1,10,int\n\
            value2,20,int\n\
            value3,40.5,float\n\
            \"val\n\
            ue4\",\"a value, is it not?\",string\n\
            value5,\"this is a \"\"quoted\"\" word\",string";

        let header_expected = make_strvec![ "Name", "Value", "Type" ];

        let data_expected = make_strvec![
                "value1", "10", "int",
                "value2", "20", "int",
                "value3", "40.5", "float",
                "value4", "a value, is it not?", "string",
                "value5", "this is a \"quoted\" word", "string" ];

        let dims_expected = (3, 5);

        let f = "csv_data_valid.csv";
        setup_from_file(&f, &s).expect("setup_from_file failed");

        let r = from_file(&f, true).expect("file read error")
            .expect("parse error");

        assert_eq!(r.header, header_expected);
        assert_eq!(r.data, data_expected);
        assert_eq!(r.dims, dims_expected);

        teardown_from_file(&f).expect("teardown_from_file failed");
    }

    #[test]
    fn test_from_file_invalid_data() {
        let s =
            "Name,Value,Type\n\
            value1,10,int\n\
            value2,20,int\n\
            value3,40.5,float\n\
            \"val\n\
            ue4,\"a value, is it not?\",string\n\
            value5,\"this is a \"\"quoted\"\" word\",string";

        let f = "csv_data_invalid.csv";
        setup_from_file(&f, &s).expect("setup_from_file failed");

        let r = from_file(&f, true).expect("file read error");
        let e = CsvValidationError::QuoteValidationError {
            subtype: CsvQuoteValidationError::InvalidQuoteError,
            row: 5,
            col: 1,
            value: "\"value4,\"a value".to_owned()
        };

        assert_eq!(r.err().unwrap(), e);

        teardown_from_file(&f).expect("teardown failed");
    }
}
