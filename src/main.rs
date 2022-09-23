use std::{collections::HashMap, io};

fn user_input() -> Option<String> {
    let mut buffer = String::new();

    while io::stdin().read_line(&mut buffer).is_err() {
        println!("please enter your data again");
    }

    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

enum MainMenu {
    AddBill,
    ViewBill
}

impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            _ => None,
        }
    }

    fn show() {
        println!("");
        println!(" == Bill Manager ==");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("");
        println!("Enter selection: ");
    }
}

#[derive(Debug, Clone)] 
pub struct Bill {
    name: String,
    amount: f64
}

pub struct Bills {
    inner: Vec<Bill>
}

impl Bills {
    fn new() -> Self {
        Self { inner: vec![] }
    }

    fn add (&mut self, bill: Bill) {
        self.inner.push(bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.iter().collect()
    }
}

fn main() {
    
    loop {
        MainMenu::show();
        let input = user_input().expect("no data entered");
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => (),
            Some(MainMenu::ViewBill) => (),
            None => return,
        }
    }
    


}

