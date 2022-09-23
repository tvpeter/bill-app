use crate::bill::{Bill, Bills};
use crate::input;

    pub fn add_bill(bills: &mut Bills) {
        println!("Bill name: ");
        let name = match input::user_input() {
            Some(input) => input.to_lowercase(),
            None => return,
        };

        println!("Enter amount: ");
        let amount = match input::get_bill_amount() {
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
        let name = match input::user_input() {
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

        let name = match input::user_input() {
            Some(name) => name.to_lowercase(),
            None => return,
        };

        println!("Enter new bill amount");

        let amount = match input::get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };

        if bills.update(&name, amount) {
            println!("bill updated");
        }else {
            println!("bill not found");
        }

    }
