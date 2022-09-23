use std::io;

pub fn user_input() -> Option<String> {
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

pub fn get_bill_amount() -> Option<f64> {
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
