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

fn parse_csv(buffer: &str, header: bool) -> Option<CsvData> {
    let mut csv_data = CsvData::new();
    let mut header_processed = false;

    let mut v: Vec<String> = Vec::new();

    let mut inside_quote = false;
    let mut valid_csv = true;
    let mut current_field = String::new();

    for c in buffer.chars() {
        if c == '"' {
            // track quoted strings
            inside_quote = !inside_quote;
        }
        else if c == ',' && !inside_quote {
            // reached the end of a field
            v.push(current_field.clone());
            current_field.clear();
        }
        else if c == '\r' {
            // skip carriage returns
            continue;
        }
        else if c == '\n' && !inside_quote {
            // if a newline and not inside a quote the end of a row has been reached
            v.push(current_field.clone());
            current_field.clear();

            if header && !header_processed {
                csv_data.header = v.clone();
                header_processed = true;
            }
            else {
                csv_data.data.push(v.clone());
            }

            v.clear();
        }
        else {
            // append the character to the buffer
            current_field.push(c);
        }
    }

    // deal with the case there is not a newline at the end of the file
    if current_field.len() > 0 {
        v.push(current_field.clone());
        // handle the case where a file might just have a header row...
        if header && !header_processed {
            csv_data.header = v.clone();
        }
        else {
            csv_data.data.push(v.clone());
        }
    }

    // if still inside a quote the csv is invalid
    valid_csv = !inside_quote;

    let mut num_fields: usize = 0;
    let mut curr_row = 0;

    for v in &csv_data.data {
        curr_row += 1;

        if num_fields == 0 {
            num_fields = v.len();
        }
        else if num_fields > 0 && num_fields != v.len() {
            println!("Field count mismatch on row {}. Expected: {}, Got: {}",
                     curr_row, num_fields, v.len());
            valid_csv = false;
            break;
        }
    }

    if !valid_csv {
        return None;
    }

    Some(csv_data)
}

pub fn from_file(filename: &str, header: bool) -> io::Result<Option<CsvData>> {
    let mut f = File::open(filename)?;

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    Ok(parse_csv(&buffer, header))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv_header_only_no_newline() {
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
}