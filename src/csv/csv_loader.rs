use std::io;
use std::io::Read;
use std::fs::File;
use std::vec::Vec;

#[derive(Debug)]
pub struct CsvData {
    header: Vec<String>,
    data: Vec<Vec<String>>,
}

impl CsvData {
    pub fn new() -> Self {
        CsvData {
            header: Vec::new(),
            data: Vec::new()
        }
    }
}

pub fn from_file(filename: &str, header: bool) -> io::Result<Option<CsvData>> {
    let mut f = File::open(filename)?;

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    Ok(parse_csv(&buffer, header))
}

fn parse_csv(buffer: &str, header: bool) -> Option<CsvData> {
    let mut csv_data = CsvData::new();
    let mut v: Vec<String> = Vec::new();

    let mut header_processed = false;
    let mut inside_quote = false;
    let mut current_field = String::new();
    let mut num_fields: usize = 0;
    let mut buffer_pos: usize = 0;
    let buffer_len: usize = buffer.len();

    for mut c in buffer.chars() {
        buffer_pos += 1;

        if c != ',' && c != '\n' && c != '\r' {
            if c == '"' {
                // track quoted strings
                inside_quote = !inside_quote;
            }
            current_field.push(c);
        }

        // handle the case where there is no terminating newline
        if buffer_pos == buffer_len {
            c = '\n';
        }

        // only process a field or row when not inside a set of outer quotes
        if !inside_quote {
            // process the field. field either terminates in a comma or newline
            if c == ',' || c == '\n' {
                if !validate_field(&current_field) {
                    println!("Invalid field. Failed quote validation. {}", current_field);
                    return None;
                }

                v.push(finalize_field(&current_field));
                current_field = String::new();
            }

            // process the row. row ends in a newline
            if c == '\n' {
                num_fields = if num_fields > 0 { num_fields } else { v.len() };

                if num_fields != v.len() {
                    let curr_row = if csv_data.header.len() > 0
                    { csv_data.data.len() + 1 } else { csv_data.data.len() };

                    println!("Field count mismatch on row {}. Expected: {}, Got: {}",
                             curr_row, num_fields, v.len());

                    return None;
                }

                if header && !header_processed {
                    csv_data.header = v;
                    header_processed = true;
                } else {
                    csv_data.data.push(v);
                }

                v = Vec::new();
            }
        }
    }

    // the parser might have not matched a set of quotes
    if inside_quote {
        return None;
    }

    Some(csv_data)
}

fn validate_field(field: &str) -> bool {
    let field_len = field.len();
    let has_outer_quotes = has_outer_quotes(&field);
    let mut found_escaped_quote = field_len;
    let mut field_pos = 0;

    for c in field.chars() {
        // look for valid escape sequences
        if field_pos > 0 && field_pos < field_len - 1 && c == '"' {
            if !has_outer_quotes ||
                (found_escaped_quote < field_len && found_escaped_quote != field_pos - 1)
            {
                return false;
            }

            if found_escaped_quote == field_len {
                found_escaped_quote = field_pos;
            }
            else {
                found_escaped_quote = field_len;
            }
        }

        field_pos += 1;
    }

    // check for the case there was an odd number of internal quotes
    if found_escaped_quote != field_len {
        return false;
    }

    true
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

    #[test]
    fn test_validate_field_none() {
        let s = String::from("abc");
        assert_eq!(validate_field(&s), true)
    }

    #[test]
    fn test_validate_field_outer_quotes_with_contents() {
        let s = String::from("\"abc\"");
        assert_eq!(validate_field(&s), true)
    }

    #[test]
    fn test_validate_field_outer_quotes_empty() {
        let s = String::from("\"\"");
        assert_eq!(validate_field(&s), true)
    }

    #[test]
    fn test_validate_field_invalid_escaped_quotes() {
        let s = String::from("abc\"\"de");
        assert_eq!(validate_field(&s), false)
    }

    #[test]
    fn test_validate_field_invalid_escaped_quotes2() {
        let s = String::from("\"abc\"\"de");
        assert_eq!(validate_field(&s), false)
    }

    #[test]
    fn test_validate_field_invalid_quotes_with_outer_single_quote() {
        let s = String::from("\"\"\"");
        assert_eq!(validate_field(&s), false)
    }

    #[test]
    fn test_validate_field_invalid_quotes_with_outer_with_many_single_quote() {
        let s = String::from("\"abc\"de\"f\"");
        assert_eq!(validate_field(&s), false)
    }

    #[test]
    fn test_validate_field_invalid_quotes_no_outer() {
        let s = String::from("abc\"def");
        assert_eq!(validate_field(&s), false)
    }

    #[test]
    fn test_validate_field_outer_quotes_with_one_valid_escape() {
        let s = String::from("\"a\"\"bc\"");
        assert_eq!(validate_field(&s), true)
    }

    #[test]
    fn test_validate_field_outer_quotes_with_many_valid_escapes() {
        let s = String::from("\"a\"\"bcd\"\"efg\"\"\"");
        assert_eq!(validate_field(&s), true)
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
            data: vec![]
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
            data: vec![]
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
            data: vec![]
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
                    vec![ String::from("value1"),
                          String::from("value2"),
                          String::from("this is a value")],
            ],
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
                vec![ String::from("value1"),
                      String::from("value2"),
                      String::from("this is a value")],
            ],
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
                vec![ String::from("value1"),
                      String::from("value2"),
                      String::from("this is a value")],
            ],
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
                vec![ String::from("value1"),
                      String::from("value2"),
                      String::from("this is a value")],
                vec![ String::from("value3"),
                      String::from("value4"),
                      String::from("another value")],
                vec![ String::from("value5"),
                      String::from("value6"),
                      String::from("yet another value")],
            ],
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
                vec![ String::from("value1"),
                      String::from("value2"),
                      String::from("this is a value")],
                vec![ String::from("value3"),
                      String::from("value4"),
                      String::from("another value")],
                vec![ String::from("value5"),
                      String::from("value6"),
                      String::from("yet another value")],
            ],
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
            header: vec![ String::from("Name"),
                          String::from("Type"),
                          String::from("Value")
            ],
            data: vec![
                vec![ String::from("value1"),
                      String::from("int"),
                      String::from("30")],
            ],
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
            header: vec![ String::from("Name"),
                          String::from("Type"),
                          String::from("Value")
            ],
            data: vec![
                vec![ String::from("value1"),
                      String::from("int"),
                      String::from("30")],
            ],
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
            header: vec![ String::from("Name"),
                          String::from("Type"),
                          String::from("Value")
            ],
            data: vec![
                vec![ String::from("value1"),
                      String::from("int"),
                      String::from("30")],
                vec![ String::from("value2"),
                      String::from("string"),
                      String::from("this is a value")
                ]
            ],
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
            header: vec![ String::from("Name"),
                          String::from("Type"),
                          String::from("Value")
            ],
            data: vec![
                vec![ String::from("value1"),
                      String::from("int"),
                      String::from("30")],
                vec![ String::from("value2"),
                      String::from("string"),
                      String::from("this is a value")
                ]
            ],
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
            header: vec![ String::from("Name"),
                          String::from("Type"),
                          String::from("Value")
            ],
            data: vec![
                vec![ String::from("value1"),
                      String::from("int"),
                      String::from("30")],
                vec![ String::from("value2"),
                      String::from("string"),
                      String::from("this is a value")
                ]
            ],
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
            header: vec![ String::from("Name"),
                          String::from("Type"),
                          String::from("Value")
            ],
            data: vec![
                vec![ String::from("value1"),
                      String::from("string"),
                      String::from("thisis a value")
                ]
            ],
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
            header: vec![ String::from("Name"),
                          String::from("Type"),
                          String::from("Value")
            ],
            data: vec![
                vec![ String::from("value1"),
                      String::from("string"),
                      String::from("this \"is a value")
                ]
            ],
        };

        let r = r.unwrap();

        assert_eq!(r.header, expected.header);
        assert_eq!(r.data, expected.data)
    }

    #[test]
    fn test_parse_csv_header_data_invalid_row_lengths() {
        let s = String::from("Name,Type,Value\nvalue1,string");
        let r = parse_csv(&s, true);

        assert!(r.is_none())
    }

    #[test]
    fn test_parse_csv_header_data_invalid_row_lengths2() {
        let s = String::from("Name,Type,Value\nvalue1,string\nvalue2,int,30");
        let r = parse_csv(&s, true);

        assert!(r.is_none())
    }

    #[test]
    fn test_parse_csv_header_data_invalid_row_lengths3() {
        let s = String::from("Name,Type\nvalue1,string,abc");
        let r = parse_csv(&s, true);

        assert!(r.is_none())
    }

    #[test]
    fn test_parse_csv_header_data_invalid_quotes() {
        let s = String::from("Name,Type,Value\nvalue1,string,a\"\"bc");
        let r = parse_csv(&s, true);

        assert!(r.is_none())
    }

    #[test]
    fn test_parse_csv_header_data_invalid_quotes2() {
        let s = String::from("Name,Type,Value\nvalue1,string,\"a\"bc\"");
        let r = parse_csv(&s, true);

        assert!(r.is_none())
    }

    #[test]
    fn test_parse_csv_header_data_invalid_quotes3() {
        let s = String::from("Name,Type,Value\n\"value1,string,abc");
        let r = parse_csv(&s, true);

        assert!(r.is_none())
    }
}