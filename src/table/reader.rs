use std::fs::File;
use std::io::{self, Read};
use std::vec::Vec;

use crate::table::data::{QuoteValidationError, TableData, TableDataValidationError};

type TableResult<T> = Result<T, TableDataValidationError>;

pub fn from_csv_file(filename: &str, header: bool) -> io::Result<TableResult<TableData>> {
    let mut f = File::open(filename)?;

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    Ok(parse_values(&buffer, ',', header))
}

pub fn from_tsv_file(filename: &str, header: bool) -> io::Result<TableResult<TableData>> {
    let mut f = File::open(filename)?;

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    Ok(parse_values(&buffer, '\t', header))
}

fn parse_values(buffer: &str, delimiter: char, header: bool) -> TableResult<TableData> {
    let mut csv_data = TableData::new();
    let mut v: Vec<String> = Vec::new();

    let mut inside_quote = false;
    let mut current_field = String::new();
    let mut num_fields: usize = 0;
    let mut prev_num_fields: usize = 0;
    let mut row_count= 0;
    let mut prev_char = '\0';

    for c in buffer.chars().filter(|x| x != &'\r')
                        .chain(std::iter::repeat('\n').take(1)) {
        if c == prev_char && c == '\n' {
            continue;
        }
        if (c != '\n' && c != delimiter) || (inside_quote && c == delimiter) {
            current_field.push(c);
        }

        // change state if the character is a quote
        inside_quote = if c == '"' { !inside_quote } else { inside_quote };
        // only process a field or row when not inside a set of outer quotes
        if !inside_quote {
            // process the field. field either terminates in a comma or newline
            if (c == '\n' && current_field.len() > 0) || c == delimiter {
                if let Err(e) = validate_field(&current_field) {
                    return Err(TableDataValidationError::QuoteValidationError {
                        subtype: e, row: row_count+1, col: (v.len()+1) as i32, value: current_field
                    });
                }

                v.push(finalize_field(&current_field));
                current_field.clear();
                num_fields += 1;
            }

            // process the row. row ends in a newline
            if c == '\n' && v.len() > 0 {
                if prev_num_fields > 0 && num_fields != prev_num_fields {
                    return Err(TableDataValidationError::RowFieldCountMismatchError {
                        row: row_count+1, expected: prev_num_fields, found: num_fields
                    });
                }

                prev_num_fields = num_fields;
                num_fields = 0;

                if header && !csv_data.has_headers() {
                    csv_data.set_header(&mut v);
                } else {
                    csv_data.set_data(&mut v, prev_num_fields);
                    row_count += 1;
                }
            }
        }

        prev_char = c;
    }

    // the parser might have not matched a set of quotes
    if inside_quote {
        return Err(TableDataValidationError::QuoteValidationError {
            subtype: QuoteValidationError::UnterminatedQuoteError,
            row: row_count+1, col: (v.len()+1) as i32, value: current_field
        });
    }

    Ok(csv_data)
}

fn validate_field(field: &str) -> Result<bool, QuoteValidationError> {
    let has_outer_quotes = has_outer_quotes(&field);
    // extract the quote indices skipping the outer quotes
    let indices= field.chars().enumerate()
                                .filter(|(i,v)|
                                    { *v == '"' && (*i > 0 && *i < field.len()-1) })
                                .map(|(i,_)| i).collect::<Vec<_>>();
    // number of quotes must be even
    if indices.len() % 2 > 0 {
        return Err(QuoteValidationError::InvalidQuoteError);
    }

    for v in indices.chunks(2) {
        if v[1] - v[0] > 1 {
            return Err(QuoteValidationError::InvalidQuoteError);
        }
        else if !has_outer_quotes {
            return Err(QuoteValidationError::InvalidEscapeError);
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
    use std::fs;
    use std::io;
    use std::io::Write;
    use std::path::Path;

    use super::*;

    macro_rules! make_strvec {
    [ $($a:expr),+ ]
        =>
    {
        vec![ $($a.to_owned()),+ ]
    }
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
        let e = QuoteValidationError::InvalidEscapeError;
        assert_eq!(validate_field(&s).err().unwrap(), e);
    }

    #[test]
    fn test_validate_field_invalid_escaped_quotes2() {
        let s = "\"abc\"\"de";
        let e = QuoteValidationError::InvalidEscapeError;
        assert_eq!(validate_field(&s).err().unwrap(), e);
    }

    #[test]
    fn test_validate_field_invalid_quotes_with_outer_single_quote() {
        let s = "\"\"\"";
        let e = QuoteValidationError::InvalidQuoteError;
        assert_eq!(validate_field(&s).err().unwrap(), e);
    }

    #[test]
    fn test_validate_field_invalid_quotes_with_outer_with_many_single_quote() {
        let s = "\"abc\"de\"f\"";
        let e = QuoteValidationError::InvalidQuoteError;
        assert_eq!(validate_field(&s).err().unwrap(), e);
    }

    #[test]
    fn test_validate_field_invalid_quotes_with_outer_with_inner_single_quote() {
        let s = "\"a\"bc\"";
        let e = QuoteValidationError::InvalidQuoteError;
        assert_eq!(validate_field(&s).err().unwrap(), e);
    }

    #[test]
    fn test_validate_field_invalid_quotes_no_outer() {
        let s = "abc\"def";
        let e = QuoteValidationError::InvalidQuoteError;
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
        let r = parse_values(&s, ',', true);

        let expected = TableData {
            header: make_strvec![ "Name", "Type", "Value" ],
            data: vec![],
            dims: (3,0),
        };

        let r = r.unwrap();

        assert_eq!(*r.header(), expected.header);
        assert_eq!(*r.data(), expected.data)
    }

    #[test]
    fn test_parse_csv_header_only_lf() {
        let s = "Name,Type,Value\n";
        let r = parse_values(&s, ',', true);

        let expected = TableData {
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
        let r = parse_values(&s, ',', true);

        let expected = TableData {
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
        let r = parse_values(&s, ',', false);

        let expected = TableData {
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
        let r = parse_values(&s, ',', false);

        let expected = TableData {
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
        let r = parse_values(&s, ',', false);

        let expected = TableData {
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
        let r = parse_values(&s, ',', false);

        let expected = TableData {
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
        let r = parse_values(&s, ',', false);

        let expected = TableData {
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
        let r = parse_values(&s, ',', true);

        let expected = TableData {
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
        let r = parse_values(&s, ',', true);

        let expected = TableData {
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
        let r = parse_values(&s, ',', true);

        let expected = TableData {
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
        let r = parse_values(&s, ',', true);

        let expected = TableData {
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
        let r = parse_values(&s, ',', true);

        let expected = TableData {
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
        let r = parse_values(&s, ',', true);

        let expected = TableData {
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
        let r = parse_values(&s, ',', true);

        let expected = TableData {
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
        let r = parse_values(&s, ',', true);
        let e = TableDataValidationError::RowFieldCountMismatchError { row: 1, expected: 3, found: 2};

        assert_eq!(r.err().unwrap(), e);
    }

    #[test]
    fn test_parse_csv_header_data_invalid_row_lengths2() {
        let s = "Name,Type,Value\nvalue1,string\nvalue2,int,30";
        let r = parse_values(&s, ',', true);
        let e = TableDataValidationError::RowFieldCountMismatchError { row: 1, expected: 3, found: 2};

        assert_eq!(r.err().unwrap(), e);
    }

    #[test]
    fn test_parse_csv_header_data_invalid_row_lengths3() {
        let s = "Name,Type\nvalue1,string,abc";
        let r = parse_values(&s, ',', true);
        let e = TableDataValidationError::RowFieldCountMismatchError { row: 1, expected: 2, found: 3};

        assert_eq!(r.err().unwrap(), e);
    }

    #[test]
    fn test_parse_csv_header_data_invalid_quotes() {
        let s = "Name,Type,Value\nvalue1,string,a\"\"bc";
        let r = parse_values(&s, ',', true);

        let e = TableDataValidationError::QuoteValidationError {
            subtype: QuoteValidationError::InvalidEscapeError,
            row: 1, col: 3, value: String::from("a\"\"bc") };

        assert_eq!(r.err().unwrap(), e);
    }

    #[test]
    fn test_parse_csv_header_data_invalid_quotes2() {
        let s = "Name,Type,Value\nvalue1,string,\"a\"bc\"";
        let r = parse_values(&s, ',', true);

        let e = TableDataValidationError::QuoteValidationError {
            subtype: QuoteValidationError::UnterminatedQuoteError,
            row: 1, col: 3, value: String::from("\"a\"bc\"") };

        assert_eq!(r.err().unwrap(), e);
    }

    #[test]
    fn test_parse_csv_header_data_invalid_quotes3() {
        let s = "Name,Type,Value\n\"value1,string,abc";
        let r = parse_values(&s, ',', true);

        let e = TableDataValidationError::QuoteValidationError {
            subtype: QuoteValidationError::UnterminatedQuoteError,
            row: 1, col: 1, value: String::from("\"value1,string,abc") };

        assert_eq!(r.err().unwrap(), e);
    }

    #[test]
    fn test_parse_csv_header_data_invalid_quotes3_msg() {
        let s = "Name,Type,Value\n\"value1,string,abc";
        let r = parse_values(&s, ',', true);

        let m =
            "At row 1. Unterminated outer quote error \
            in column: 1, value: \"value1,string,abc";

        assert_eq!(r.err().map(|e| format!("{}",e)).unwrap(), m);
    }

    // helpers for testing from_file(...)
    fn setup_from_file(target: &str, data: &str) -> io::Result<()> {
        let mut f = File::create(target)?;
        f.write_all(data.as_bytes())?;
        Ok(())
    }

    fn teardown_from_file(target: &str) -> io::Result<()> {
        fs::remove_file(Path::new(target))?;
        Ok(())
    }

    #[test]
    fn test_from_csv_file_valid_data() {
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

        let r = from_csv_file(&f, true).expect("file read error")
            .expect("parse error");

        assert_eq!(r.header(), &header_expected);
        assert_eq!(r.data(), &data_expected);
        assert_eq!(r.columns(), dims_expected.0);
        assert_eq!(r.rows(), dims_expected.1);

        teardown_from_file(&f).expect("teardown_from_file failed");
    }

    #[test]
    fn test_from_csv_file_invalid_data() {
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

        let r = from_csv_file(&f, true).expect("file read error");
        let e = TableDataValidationError::QuoteValidationError {
            subtype: QuoteValidationError::InvalidQuoteError,
            row: 4,
            col: 1,
            value: "\"value4,\"a value".to_owned()
        };

        assert_eq!(r.err().unwrap(), e);

        teardown_from_file(&f).expect("teardown failed");
    }
}