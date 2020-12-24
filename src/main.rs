mod csv;
use csv::csv_loader;

fn main() {
    let csv_contents = csv_loader::from_file("test.csv", false);

    println!("{:?}", csv_contents.unwrap().unwrap())
}
