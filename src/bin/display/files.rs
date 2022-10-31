use std::error::Error;
use std::fs::File;
use std::io::{BufReader};

use crate::contact::{Contact, ContactObject};

pub fn open_csv_with_header(csv_path: String) -> Result<ContactObject, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .comment(Some(b';'))
        .has_headers(true)
        .flexible(true)
        .trim(csv::Trim::All)
        .from_path(csv_path.clone())?;
    
    // Headers
    let mut headers: Vec<String> = Vec::new();
    for header in rdr.headers()? {
        headers.push(header.to_owned())
    }

    // CSV file content
    let mut content: Vec<Contact> = Vec::new();
    for r in rdr.deserialize() {
        let record: Contact = r?;
        content.push(record);
    }
    Ok(ContactObject { csv_file: csv_path, headers: headers, content: content })
}

pub fn open_csv_header(csv_path: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let rdr = csv::Reader::from_path(csv_path);
    let mut result:Vec<String> = Vec::new();
    match rdr {
        Ok(mut read) => {
            let headers = read.headers()?;
            for header in headers {
                result.push(header.to_owned());
            }
        },
        Err(_) => println!("unable to open csv header"),
    }
    Ok(result)
}

pub fn open_csv(csv: &String) -> Result<Vec<Contact>, Box<dyn Error>> {
    let f = File::open(csv)?;
    let reader = BufReader::new(f);

    let mut rdr = csv::ReaderBuilder::new()
        .comment(Some(b';'))
        .has_headers(true)
        .flexible(true)
        .trim(csv::Trim::All)
        .from_reader(reader);

    let mut contacts: Vec<Contact> = Vec::new();
    for result in rdr.deserialize() {
        let record: Contact = result?;
        contacts.push(record);
    }
    
    Ok(contacts)
}