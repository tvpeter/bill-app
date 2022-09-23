use std::{io, collections::HashMap};

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

fn get_bill_amount() -> Option<f64> {
    loop {
        let input = match user_input() {
            Some(input) => input,
            None => return None,
        };

        if &input == "" {
            return None;
        }

        let parsed_input : Result<f64, _> = input.parse();

        match parsed_input {
            Ok(amount)=> return Some(amount),
            Err(_) => println!("please enter a number"),
        }
    }
}

mod menu {
    use crate::{Bills, user_input, Bill, get_bill_amount};

    pub fn add_bill(bills: &mut Bills) {
        println!("Bill name: ");
        let name = match user_input() {
            Some(input) => input.to_lowercase(),
            None => return,
        };

        println!("Enter amount: ");
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };

        let bill = Bill { name, amount };
        bills.add(bill);
        println!("Bill added");
    }

    pub fn view_bills (bills: &Bills) {
        for bill in bills.get_all() {
            println!("Bill: {:?}, amount: {:?}", bill.name, bill.amount);
        }
    }

    pub fn remove_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
        println!("supply bill name: ");
        let name = match user_input() {
            Some(input) => input.to_lowercase(),
            None => return,
        };

        if bills.remove(name.as_str()) {
            println!("Bill removed successfully");
        }else {
            println!("Bill not found");
        }
    }

    pub fn update_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }

        println!("Enter bill name to update");

        let name = match user_input() {
            Some(name) => name.to_lowercase(),
            None => return,
        };

        println!("Enter new bill amount");

        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };

        if bills.update(&name, amount) {
            println!("bill updated");
        }else {
            println!("bill not found");
        }

    }
}

enum MainMenu {
    AddBill,
    ViewBill,
    RemoveBill,
    UpdateBill,
}

impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            "3" => Some(Self::RemoveBill),
            "4" => Some(Self::UpdateBill),
            _ => None,
        }
    }

    fn show() {
        println!("");
        println!(" == Bill Manager ==");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bill");
        println!("4. Edit Bill");
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
    inner: HashMap<String, Bill>
}

impl Bills {
    fn new() -> Self {
        Self { inner: HashMap::new() }
    }

    fn add (&mut self, bill: Bill) {
        self.inner.insert(bill.name.to_string(), bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }

    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }

    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                true
            },
            None => false,
        }
    }
}

fn run_program() -> Option<()> {
    let mut bills = Bills::new();
    
    loop {
        MainMenu::show();
        let input = user_input()?;
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => menu::add_bill(&mut bills),
            Some(MainMenu::ViewBill) => menu::view_bills(&bills),
            Some(MainMenu::RemoveBill) => menu::remove_bill(&mut bills),
            Some(MainMenu::UpdateBill) => menu::update_bill(&mut bills),
            None => break,
        }
    }
    None
}
fn main() {
    run_program();    

}

