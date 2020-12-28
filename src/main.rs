mod csv;
use csv::reader;

fn main() {
    let csv_contents = reader::from_file("test.csv", false);

    println!("{:?}", csv_contents.unwrap().unwrap())
}
