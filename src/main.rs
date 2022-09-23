pub mod mainmenu;

use bill_manager::bill::{Bills};
use bill_manager::menu;
use bill_manager::input;
use bill_manager::mainmenu::MainMenu;


fn run_program() -> Option<()> {

    let mut bills = Bills::new();
    
    loop {
        MainMenu::show();
        let input = input::user_input()?;
        match MainMenu::from_str_input(input.as_str()) {
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

