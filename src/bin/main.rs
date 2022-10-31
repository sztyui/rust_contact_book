
// Project 2: Contact manager
//
// User stories:
// * L1: I want to view my saved contacts.
// * L2: I want to add new contacts.
// * L2: I want to search for contacts.
// * L3: I want to edit and remove existing contacts.
//
// Tips:
// * Make a backup of the existing `p2_data.csv` file.
// * CSV files contain records and fields:
//   Each line is a "record" which contain "fields" of information.
//   The fields are separated by commas. If a field is not provided,
//   then there is no data for that particular field. There will
//   always be a comma for the field even if no data is present.
// * The `id` and `name` fields are required, the `email` field is optional.
// * Check the documentation on the `std::fs::File` struct for reading
//   and writing files.
// * Use the `split` function from the standard library to extract
//   specific fields.
// * Try the `structopt` crate if you want to make a non-interactive
//   command line application.
// * Create your program starting at level 1. Once finished, advance
//   to the next level.
// * Make your program robust: there are 7 errors & multiple blank lines
//   present in the data.

use display::menu::{MenuOption, menu};
use display::files::{
    open_csv_with_header
};
use display::functions::{
    show_data, 
    add_data, 
    search_data, 
    edit_data
};
extern crate lapp;

fn main() {
    let args = lapp::parse_args("
Simple contact handling application.
    -f, --file (string)  neccesary input CSV file for loading the data");
    let csv_path = args.get_string("file");
    let mut contact_object = open_csv_with_header(csv_path.clone()).unwrap();

    loop {
        match menu() {
            Ok(MenuOption::Show) => show_data(&contact_object),
            Ok(MenuOption::Add) => add_data(&mut contact_object.content),
            Ok(MenuOption::Search) => search_data(&contact_object),
            Ok(MenuOption::Edit) => edit_data(&mut contact_object),
            Ok(MenuOption::Quit) => break,
            Err(e) => panic!("{}", e),
        }
    }   
}
