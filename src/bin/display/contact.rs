
use serde::Deserialize;
use std::fmt::Display;

pub struct ContactObject {
    pub csv_file: String,
    pub headers: Vec<String>,
    pub content: Vec<Contact>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Contact {
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<f64>,
    name: Option<String>,
    email: Option<String>,
}

impl Contact {
    pub fn new(i: Option<f64>, n: Option<String>, e: Option<String>) -> Self {
        Self{id: i, name: n, email: e}
    }

    pub fn set_id(&mut self, id: Option<f64>) {
        self.id = id;
    }

    pub fn get_id(&self) -> f64 {
        let result: f64;
        match &self.id {
            Some(x) => result = *x,
            None => result = 0.0,
        };
        return result
    }

    pub fn get_name(&self) -> String {
        let result: String;
        match &self.name {
            Some(x) => result = x.to_string(),
            None => result = "None".to_owned(),
        };
        return result
    }

    pub fn get_email(&self) -> String {
        let result: String;
        match &self.email {
            Some(x) => result = x.to_string(),
            None => result = "None".to_owned(),
        };
        return result
    }
}

impl Display for Contact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = match &self.id {
            Some(x) => x.to_string(),
            None => "None".to_owned(),
        };
        let name = match &self.name {
            Some(x) => x,
            None => "None",
        };
        let email = match &self.email {
            Some(x) => x,
            None => "None",
        };
        write!(f, "ID: {}, Name: {}, Email: {}", id, name, email)?;
        Ok(())
    }
}