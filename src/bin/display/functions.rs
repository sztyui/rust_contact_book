extern crate term_table;
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
};
use term_table::{Table, TableStyle};

use crate::{contact::{Contact, ContactObject}, menu::{get_contact, get_input}, menu::get_search};

use std::fmt;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum SearchMode {
    None,
    Id,
    Name,
    Email
}

impl fmt::Display for SearchMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Search {
    pub search_mode: SearchMode,
    pub keyword: String,
}

pub struct Tables {
    pub header_line: Vec<String>
}

impl Tables {

    pub fn display(&self, contacts: &Vec<Contact>) {
        let mut table = Table::new();
        table.max_column_width = 40;
    
        table.style = TableStyle::extended();

        table.add_row(
            Row::new(
                vec![
                    TableCell::new_with_alignment(&self.header_line[0], 1, Alignment::Center),
                    TableCell::new_with_alignment(&self.header_line[1], 2, Alignment::Right),
                    TableCell::new_with_alignment(&self.header_line[2], 2, Alignment::Right),
                ]
            )
        );
    
        for c in contacts {
            table.add_row(
                Row::new(
                    vec![
                        TableCell::new_with_alignment(c.get_id(), 1, Alignment::Center),
                        TableCell::new_with_alignment(c.get_name(), 2, Alignment::Right),
                        TableCell::new_with_alignment(c.get_email(), 2, Alignment::Right),
                    ]
                )
            )
        }
    
        print!("{}[2J", 27 as char);
        println!("{}", table.render());
    }
}

pub fn show_data(co: &ContactObject) {
    let table = Tables{header_line: co.headers.to_vec()};
    table.display(&co.content);
}

pub fn add_data(contacts: &mut Vec<Contact>){
    let new_id = contacts[contacts.len() - 1].get_id() + 1.0;
    let mut c = get_contact().unwrap();
    c.set_id(Some(new_id));
    contacts.push(c);
}

pub fn search_data(co: &ContactObject){
    let result = get_search(vec![SearchMode::Id, SearchMode::Name, SearchMode::Email]);
    match result {
        Ok(s) =>{
            match s.search_mode {
                SearchMode::Id => {
                    let f = s.keyword.parse::<f64>().unwrap();
                    let result: Vec<Contact> = co.content.iter().cloned().filter(|c| c.get_id() == f).collect();
                    Tables::display(&Tables{header_line: co.headers.to_vec()}, &result);
                },
                SearchMode::Name => {
                    let result: Vec<Contact> = co.content.iter().cloned().filter(|c| c.get_name().contains(&s.keyword)).collect();
                    Tables::display(&Tables{header_line: co.headers.to_vec()}, &result);
                },
                SearchMode::Email => {
                    let result: Vec<Contact> = co.content.iter().cloned().filter(|c| c.get_email().contains(&s.keyword)).collect();
                    Tables::display(&Tables{header_line: co.headers.to_vec()}, &result);
                },
                SearchMode::None => { println!("no search mode specified") },
            }
        },
        Err(_) => println!("no search specified"),
    }
}

pub fn edit_data(co: &mut ContactObject){
    let search_id = get_input("Please type in the ID".to_owned(), &"".to_owned()).unwrap().parse::<f64>().unwrap();
    let result = co.content.iter().cloned().position(|c| c.get_id().eq(&search_id));
    match result {
        Some(index) => {
            let elem = co.content[index].clone();
            Tables::display(&Tables{header_line: co.headers.to_vec()}, &vec![elem.clone()]);
            let name = get_input("Please type in the new name".to_owned(), &elem.get_name()).unwrap();
            let email = get_input("Please type in the new email address".to_owned(), &elem.get_name()).unwrap();
            let new = Contact::new(Some(elem.get_id()), Some(name), Some(email));
            Tables::display(&Tables{header_line: co.headers.to_vec()}, &vec![new.clone()]);
            let _ = std::mem::replace(&mut co.content[index], new);  
        },
        None => println!("The element not found"),
    }
}
