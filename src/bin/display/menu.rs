use dialoguer::{
    Select,
    theme::ColorfulTheme,
    Input,
};
use console::Term;

use crate::contact::Contact;
use crate::functions::{SearchMode, Search};
extern crate term_table;

use std::vec;


pub enum MenuOption {
    Show,
    Add,
    Search,
    Edit,
    Quit,
}

pub fn menu() -> std::io::Result<MenuOption> {
    let menu_items = vec!["👀 Show", "📕 Add", "🔎 Search", "📝 Edit", "❌ Exit"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&menu_items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;
    
    match selection {
        Some(index) => {
            match index {
                0 => Ok(MenuOption::Show),
                1 => Ok(MenuOption::Add),
                2 => Ok(MenuOption::Search),
                3 => Ok(MenuOption::Edit),
                4 => Ok(MenuOption::Quit),
                _ => todo!(),
            }
        },
        None => panic!("unknown menu item specified!")
    }
}

pub fn get_contact() -> std::io::Result<Contact> {
    let name: String = Input::new()
        .with_prompt("Enter the name: ")
        .interact_text()?;
    
    let email: String = Input::new()
        .with_prompt("Enter the e-mail address: ")
        .interact_text()?;
    
    Ok(Contact::new(None, Some(name), Some(email)))
}

pub fn get_search(fields: Vec<SearchMode>) -> std::io::Result<Search> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&fields)
        .default(1)
        .interact_on_opt(&Term::stderr())?;
    
    let result = match selection {
        Some(index) => {
            let input: String = Input::new().with_prompt("Input").interact_text()?;
            Search{search_mode: fields[index], keyword: input}
        },
        None => Search{search_mode: SearchMode::None, keyword: "".to_owned()},
        
    };
    return Ok(result);
}

pub fn get_input(prompt: String, original: &String) -> std::io::Result<String> {
    let input: String = Input::new()
        .with_prompt(prompt)
        .default(original.into())
        .interact_text()?;
    Ok(input)
}