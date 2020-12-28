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
    let field_len = field.len();
    let has_outer_quotes = has_outer_quotes(&field);
    let mut found_escaped_quote = field_len;
    let mut field_pos = 0;

    for c in field.chars() {
        // look for valid escape sequences
        if field_pos > 0 && field_pos < field_len - 1 && c == '"' {
            if found_escaped_quote < field_len && found_escaped_quote != field_pos - 1
            {
                return Err(CsvQuoteValidationError::InvalidQuoteError);
            }

            if found_escaped_quote == field_len {
                found_escaped_quote = field_pos;
            }
            else {
                if !has_outer_quotes {
                    return Err(CsvQuoteValidationError::InvalidEscapeError);
                }
                found_escaped_quote = field_len;
            }
        }

        field_pos += 1;
    }

    // check for the case there was an odd number of internal quotes
    if found_escaped_quote != field_len {
        return Err(CsvQuoteValidationError::InvalidQuoteError);
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

    #[test]
    fn test_csvdata_cols_rows_len() {
        let data = vec![String::from("a"), String::from("b"), String::from("c")];
        let c = CsvData { header: vec![], data: data, dims: (3,1)};

        assert_eq!(c.columns(), 3);
        assert_eq!(c.rows(), 1);
        assert_eq!(c.len(), 3);
        assert_eq!(c.has_headers(), false);
        assert_eq!(c.has_data(), true);
    }

    // tests
    #[test]
    fn test_validate_field_none() {
        let s = String::from("abc");
        assert!(validate_field(&s).is_ok())
    }

    #[test]
    fn test_validate_field_outer_quotes_with_contents() {
        let s = String::from("\"abc\"");
        assert!(validate_field(&s).is_ok())
    }

    #[test]
    fn test_validate_field_outer_quotes_empty() {
        let s = String::from("\"\"");
        assert!(validate_field(&s).is_ok())
    }

    #[test]
    fn test_validate_field_invalid_escaped_quotes() {
        let s = String::from("abc\"\"de");
        let e = CsvQuoteValidationError::InvalidEscapeError;
        assert_eq!(validate_field(&s).err().unwrap(), e);
    }

    #[test]
    fn test_validate_field_invalid_escaped_quotes2() {
        let s = String::from("\"abc\"\"de");
        let e = CsvQuoteValidationError::InvalidEscapeError;
        assert_eq!(validate_field(&s).err().unwrap(), e);
    }

    #[test]
    fn test_validate_field_invalid_quotes_with_outer_single_quote() {
        let s = String::from("\"\"\"");
        let e = CsvQuoteValidationError::InvalidQuoteError;
        assert_eq!(validate_field(&s).err().unwrap(), e);
    }

    #[test]
    fn test_validate_field_invalid_quotes_with_outer_with_many_single_quote() {
        let s = String::from("\"abc\"de\"f\"");
        let e = CsvQuoteValidationError::InvalidQuoteError;
        assert_eq!(validate_field(&s).err().unwrap(), e);
    }

    #[test]
    fn test_validate_field_invalid_quotes_with_outer_with_inner_single_quote() {
        let s = String::from("\"a\"bc\"");
        let e = CsvQuoteValidationError::InvalidQuoteError;
        assert_eq!(validate_field(&s).err().unwrap(), e);
    }

    #[test]
    fn test_validate_field_invalid_quotes_no_outer() {
        let s = String::from("abc\"def");
        let e = CsvQuoteValidationError::InvalidQuoteError;
        assert_eq!(validate_field(&s).err().unwrap(), e);
    }

    #[test]
    fn test_validate_field_outer_quotes_with_one_valid_escape() {
        let s = String::from("\"a\"\"bc\"");
        assert!(validate_field(&s).is_ok())
    }

    #[test]
    fn test_validate_field_outer_quotes_with_many_valid_escapes() {
        let s = String::from("\"a\"\"bcd\"\"efg\"\"\"");
        assert!(validate_field(&s).is_ok())
    }

    #[test]
    fn test_has_outer_quotes_quoted() {
        let s = String::from("\"abc\"");
        assert_eq!(has_outer_quotes(&s), true)
    }

    #[test]
    fn test_has_outer_quotes_only_quotes() {
        let s = String::from("\"\"");
        assert_eq!(has_outer_quotes(&s), true)
    }

    #[test]
    fn test_has_outer_quotes_none() {
        let s = String::from("a\"\"bc");
        assert_eq!(has_outer_quotes(&s), false)
    }

    #[test]
    fn test_finalize_field_outer_quotes() {
        let s = String::from("\"this is a value\"");
        assert_eq!(finalize_field(&s), String::from("this is a value"))
    }

    #[test]
    fn test_finalize_field_escaped_quotes() {
        let s = String::from("\"this is a \"\"value\"\" that is quoted\"");
        assert_eq!(finalize_field(&s), String::from("this is a \"value\" that is quoted"))
    }

    #[test]
    fn test_finalize_field_escaped_quotes2() {
        let s = String::from("\"this is a \"\"\"\"value\"\" that\"\" is quoted\"");
        assert_eq!(finalize_field(&s), String::from("this is a \"\"value\" that\" is quoted"))
    }

    #[test]
    fn test_finalize_field_no_quotes() {
        let s = String::from("this is a string without quotes");
        assert_eq!(finalize_field(&s), String::from("this is a string without quotes"))
    }

    #[test]
    fn test_finalize_field_only_quotes() {
        let s = String::from("\"\"");
        assert_eq!(finalize_field(&s), String::new())
    }

    #[test]
    fn test_parse_csv_header_only_no_lf() {
        let s = String::from("Name,Type,Value");
        let r = parse_csv(&s, true);

        let expected = CsvData {
            header: vec![ String::from("Name"),
                          String::from("Type"),
                          String::from("Value")
            ],
            data: vec![],
            dims: (3,0),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_only_lf() {
        let s = String::from("Name,Type,Value\n");
        let r = parse_csv(&s, true);

        let expected = CsvData {
            header: vec![ String::from("Name"),
                          String::from("Type"),
                          String::from("Value")
            ],
            data: vec![],
            dims: (3,0),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_only_crlf() {
        let s = String::from("Name,Type,Value\r\n");
        let r = parse_csv(&s, true);

        let expected = CsvData {
            header: vec![ String::from("Name"),
                          String::from("Type"),
                          String::from("Value")
            ],
            data: vec![],
            dims: (3,0),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_no_header_no_lf() {
        let s = String::from("value1,value2,this is a value");
        let r = parse_csv(&s, false);

        let expected = CsvData {
            header: vec![],
            data: vec![
                String::from("value1"),
                String::from("value2"),
                String::from("this is a value"),
            ],
            dims: (3,1),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_no_header_lf() {
        let s = String::from("value1,value2,this is a value\n");
        let r = parse_csv(&s, false);

        let expected = CsvData {
            header: vec![],
            data: vec![
                String::from("value1"),
                String::from("value2"),
                String::from("this is a value"),
            ],
            dims: (3,1),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_no_header_crlf() {
        let s = String::from("value1,value2,this is a value\r\n");
        let r = parse_csv(&s, false);

        let expected = CsvData {
            header: vec![],
            data: vec![
                String::from("value1"),
                String::from("value2"),
                String::from("this is a value"),
            ],
            dims: (3,1),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_no_header_multiple_rows_trailing_lf() {
        let s = String::from(
            "value1,value2,this is a value\nvalue3,value4,another value\nvalue5,value6,yet another value\n");
        let r = parse_csv(&s, false);

        let expected = CsvData {
            header: vec![],
            data: vec![
                String::from("value1"),
                String::from("value2"),
                String::from("this is a value"),

                String::from("value3"),
                String::from("value4"),
                String::from("another value"),

                String::from("value5"),
                String::from("value6"),
                String::from("yet another value"),
            ],
            dims: (3,3),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_no_header_multiple_rows_no_trailing_lf() {
        let s = String::from(
            "value1,value2,this is a value\nvalue3,value4,another value\nvalue5,value6,yet another value");
        let r = parse_csv(&s, false);

        let expected = CsvData {
            header: vec![],
            data: vec![
                String::from("value1"),
                String::from("value2"),
                String::from("this is a value"),

                String::from("value3"),
                String::from("value4"),
                String::from("another value"),

                String::from("value5"),
                String::from("value6"),
                String::from("yet another value"),
            ],
            dims: (3,3),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_data() {
        let s = String::from("Name,Type,Value\nvalue1,int,30\n");
        let r = parse_csv(&s, true);

        let expected = CsvData {
            header: vec![
                String::from("Name"),
                String::from("Type"),
                String::from("Value")
            ],
            data: vec![
                String::from("value1"),
                String::from("int"),
                String::from("30"),
            ],
            dims: (3, 1),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_data_no_trailing_lf() {
        let s = String::from("Name,Type,Value\nvalue1,int,30");
        let r = parse_csv(&s, true);

        let expected = CsvData {
            header: vec![
                String::from("Name"),
                String::from("Type"),
                String::from("Value")
            ],
            data: vec![
                String::from("value1"),
                String::from("int"),
                String::from("30"),
            ],
            dims: (3, 1),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_data_multiple_rows_no_trailing_lf() {
        let s = String::from("Name,Type,Value\nvalue1,int,30\nvalue2,string,this is a value");
        let r = parse_csv(&s, true);

        let expected = CsvData {
            header: vec![
                String::from("Name"),
                String::from("Type"),
                String::from("Value")
            ],
            data: vec![
                String::from("value1"),
                String::from("int"),
                String::from("30"),

                String::from("value2"),
                String::from("string"),
                String::from("this is a value"),
            ],
            dims: (3,2),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_data_multiple_rows_trailing_lf() {
        let s = String::from("Name,Type,Value\nvalue1,int,30\nvalue2,string,this is a value\n");
        let r = parse_csv(&s, true);

        let expected = CsvData {
            header: vec![
                String::from("Name"),
                String::from("Type"),
                String::from("Value")
            ],
            data: vec![
                String::from("value1"),
                String::from("int"),
                String::from("30"),

                String::from("value2"),
                String::from("string"),
                String::from("this is a value"),
            ],
            dims: (3,2),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_data_multiple_rows_quoted_string_trailing_lf() {
        let s = String::from("Name,Type,Value\nvalue1,int,30\nvalue2,string,\"this is a value\"\n");
        let r = parse_csv(&s, true);

        let expected = CsvData {
            header: vec![
                String::from("Name"),
                String::from("Type"),
                String::from("Value")
            ],
            data: vec![
                String::from("value1"),
                String::from("int"),
                String::from("30"),

                String::from("value2"),
                String::from("string"),
                String::from("this is a value"),
            ],
            dims: (3,2),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_data_quoted_string_has_newline() {
        let s = String::from("Name,Type,Value\nvalue1,string,\"this\nis a value\"");
        let r = parse_csv(&s, true);

        let expected = CsvData {
            header: vec![
                String::from("Name"),
                String::from("Type"),
                String::from("Value")
            ],
            data: vec![
                String::from("value1"),
                String::from("string"),
                String::from("thisis a value"),
            ],
            dims: (3, 1),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_data_escaped_quoted_string() {
        let s = String::from("Name,Type,Value\nvalue1,string,\"this \"\"is a value\"");
        let r = parse_csv(&s, true);

        let expected = CsvData {
            header: vec![
                String::from("Name"),
                String::from("Type"),
                String::from("Value")
            ],
            data: vec![
                String::from("value1"),
                String::from("string"),
                String::from("this \"is a value"),
            ],
            dims: (3,1),
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_data_invalid_row_lengths() {
        let s = String::from("Name,Type,Value\nvalue1,string");
        let r = parse_csv(&s, true);
        let e = CsvValidationError::RowFieldCountMismatchError { row: 2, expected: 3, found: 2};

        assert_eq!(r.err().unwrap(), e);
    }

    #[test]
    fn test_parse_csv_header_data_invalid_row_lengths2() {
        let s = String::from("Name,Type,Value\nvalue1,string\nvalue2,int,30");
        let r = parse_csv(&s, true);
        let e = CsvValidationError::RowFieldCountMismatchError { row: 2, expected: 3, found: 2};

        assert_eq!(r.err().unwrap(), e);
    }

    #[test]
    fn test_parse_csv_header_data_invalid_row_lengths3() {
        let s = String::from("Name,Type\nvalue1,string,abc");
        let r = parse_csv(&s, true);
        let e = CsvValidationError::RowFieldCountMismatchError { row: 2, expected: 2, found: 3};

        assert_eq!(r.err().unwrap(), e);
    }

    #[test]
    fn test_parse_csv_header_data_invalid_quotes() {
        let s = String::from("Name,Type,Value\nvalue1,string,a\"\"bc");
        let r = parse_csv(&s, true);

        let e = CsvValidationError::QuoteValidationError {
            subtype: CsvQuoteValidationError::InvalidEscapeError,
            row: 2, col: 3, value: String::from("a\"\"bc") };

        assert_eq!(r.err().unwrap(), e);
    }

    #[test]
    fn test_parse_csv_header_data_invalid_quotes2() {
        let s = String::from("Name,Type,Value\nvalue1,string,\"a\"bc\"");
        let r = parse_csv(&s, true);

        let e = CsvValidationError::QuoteValidationError {
            subtype: CsvQuoteValidationError::UnterminatedQuoteError,
            row: 2, col: 3, value: String::from("\"a\"bc\"") };

        assert_eq!(r.err().unwrap(), e);
    }

    #[test]
    fn test_parse_csv_header_data_invalid_quotes3() {
        let s = String::from("Name,Type,Value\n\"value1,string,abc");
        let r = parse_csv(&s, true);

        let e = CsvValidationError::QuoteValidationError {
            subtype: CsvQuoteValidationError::UnterminatedQuoteError,
            row: 2, col: 1, value: String::from("\"value1,string,abc") };

        assert_eq!(r.err().unwrap(), e);
    }

    #[test]
    fn test_parse_csv_header_data_invalid_quotes3_msg() {
        let s = String::from("Name,Type,Value\n\"value1,string,abc");
        let r = parse_csv(&s, true);

        let m = String::from(
            "At row 2. Unterminated outer quote error \
            in column: 1, value: \"value1,string,abc"
        );

        assert_eq!(r.err().map(|e| format!("{}",e)).unwrap(), m);
    }

    #[test]
    fn test_from_file_valid_data() {
        let s = String::from(
            "Name,Value,Type\n\
            value1,10,int\n\
            value2,20,int\n\
            value3,40.5,float\n\
            \"val\n\
            ue4\",\"a value, is it not?\",string\n\
            value5,\"this is a \"\"quoted\"\" word\",string"
        );

        let header_expected = vec![
            String::from("Name"),
            String::from("Value"),
            String::from("Type")
        ];

        let data_expected = vec![
            String::from("value1"),
            String::from("10"),
            String::from("int"),

            String::from("value2"),
            String::from("20"),
            String::from("int"),

            String::from("value3"),
            String::from("40.5"),
            String::from("float"),

            String::from("value4"),
            String::from("a value, is it not?"),
            String::from("string"),

            String::from("value5"),
            String::from("this is a \"quoted\" word"),
            String::from("string"),
        ];

        let dims_expected = (3, 5);

        let f = String::from("csv_data_valid.csv");

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
        let s = String::from(
            "Name,Value,Type\n\
            value1,10,int\n\
            value2,20,int\n\
            value3,40.5,float\n\
            \"val\n\
            ue4,\"a value, is it not?\",string\n\
            value5,\"this is a \"\"quoted\"\" word\",string"
        );

        let f = String::from("csv_data_invalid.csv");
        setup_from_file(&f, &s).expect("setup_from_file failed");

        let r = from_file(&f, true).expect("file read error");

        let e = CsvValidationError::QuoteValidationError {
            subtype: CsvQuoteValidationError::InvalidQuoteError,
            row: 5,
            col: 1,
            value: String::from("\"value4,\"a value")
        };

        assert_eq!(r.err().unwrap(), e);

        teardown_from_file(&f).expect("teardown failed");
    }
}
