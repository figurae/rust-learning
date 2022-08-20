use std::{
    collections::HashMap,
    io::{self, Write},
};

fn main() {
    let mut company: HashMap<&str, Vec<String>> = HashMap::new();

    company.insert("sales", Vec::new());
    company.insert("marketing", Vec::new());
    company.insert("maintenance", Vec::new());
    company.insert("_unassigned", Vec::new());

    // TODO: maybe do this without passing company all the time?
    routing::welcome(&mut company);
}

mod routing {
    use crate::{actions, command_line_int, command_line_str, messages};
    use std::process::exit;

    pub fn welcome(company: &mut std::collections::HashMap<&str, Vec<String>>) {
        println!("\nhello, agent! what is your command?\n");

        loop {
            println!("{}", messages::welcome());

            let answer = command_line_int();
            match answer {
                1 => edit_employees(company),
                3 => view_database(company),
                0 => exit(0),
                _ => unknown(),
            }
        }
    }

    fn view_database(company: &mut std::collections::HashMap<&str, Vec<String>>) {
        println!("{:?}", company);
    }

    fn edit_employees(company: &mut std::collections::HashMap<&str, Vec<String>>) {
        loop {
            println!("{}", messages::edit_employees());

            let answer = command_line_int();
            match answer {
                1 => add_employee(company),
                2 => move_employee(company),
                3 => remove_employee(company),
                0 => welcome(company),
                _ => todo!(),
            }
        }
    }

    fn remove_employee(company: &mut std::collections::HashMap<&str, Vec<String>>) {
        // TODO: option to print all employees
        println!("select department");

        list_depts(company);
        println!("[0] cancel");

        let dept = command_line_int();

        if dept == 0 {
            return;
        };

        let dept = match dept {
            num if (1..=no_of_depts(company)).contains(&num) => {
                actions::get_department(num, company)
            }
            _ => "_".to_string(),
        };

        list_employees(&dept, company);
        // TODO: make ids

        // println!("removed {} from {}", name, dept);

        todo!();
    }

    fn move_employee(company: &mut std::collections::HashMap<&str, Vec<String>>) {
        println!("select source");

        list_depts(company);
        println!("[0] cancel");
        let source = command_line_int();

        println!("select destination");

        list_depts(company);
        println!("[0] cancel");
        let target = command_line_int();

        move_to(
            // TODO: maybe move this to routing?
            actions::get_department(source, company),
            actions::get_department(target, company),
            company,
        );
        todo!()
    }

    fn move_to(
        source: String,
        target: String,
        company: &mut std::collections::HashMap<&str, Vec<String>>,
    ) {
        let no_of_depts = no_of_depts(company);

        todo!()
    }

    fn add_employee(company: &mut std::collections::HashMap<&str, Vec<String>>) {
        println!("\nenter new employee's name or 0 to go back");

        let name = command_line_str();

        if name == "0" {
            return;
        }

        println!("\nwhere do you want to assign {}?\n", name);

        list_depts(company);
        println!("[0] cancel");

        let target = actions::get_department(command_line_int(), company);

        actions::add_employee(&name, &target, company);
        println!("{} assigned to {}", name, target);
    }

    fn unknown() {
        println!("{}", messages::unknown())
    }

    fn list_depts(company: &mut std::collections::HashMap<&str, Vec<String>>) {
        for (i, dept) in company.keys().enumerate() {
            println!("[{}] {}", i + 1, dept);
        }
    }

    fn list_employees(dept: &str, company: &mut std::collections::HashMap<&str, Vec<String>>) {
        for (i, employee) in company.get_mut(dept).unwrap().iter().enumerate() {
            println!("[{}] {}", i, employee);
        }
    }

    fn no_of_depts(company: &std::collections::HashMap<&str, Vec<String>>) -> u8 {
        company.len().try_into().unwrap()
    }
}

// TODO: try to do this without splitting into two functions
fn command_line_int() -> u8 {
    print!("> ");
    io::stdout().flush().expect("flush error");

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("read_line error");

    match answer.trim().parse() {
        Ok(num) => num,
        Err(_) => u8::MAX,
    }
}

fn command_line_str() -> String {
    print!("> ");
    io::stdout().flush().expect("flush error");

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("read_line error");

    match answer.trim().parse() {
        Ok(str) => str,
        Err(_) => "i am error".to_string(), // TODO: this should be handled better
    }
}

mod actions {
    // TODO: maybe do this with methods/traits/something...
    pub fn add_employee(
        name: &str,
        dept: &str,
        company: &mut std::collections::HashMap<&str, Vec<String>>,
    ) {
        company.get_mut(dept).unwrap().push(name.to_string());
    }

    pub fn remove_employee(
        num: u8,
        dept: &str,
        company: &mut std::collections::HashMap<&str, Vec<String>>,
    ) {
        company.get_mut(dept).unwrap().remove(num as usize);
    }

    pub fn get_department(
        num: u8,
        company: &std::collections::HashMap<&str, Vec<String>>,
    ) -> String {
        company
            .keys()
            .enumerate()
            .nth(num as usize - 1)
            .unwrap()
            .1
            .to_string() // TODO: much ugliness, wow, so refactor
    }
}

mod messages {

    pub fn welcome() -> String {
        format!(
            "{}{}{}{}",
            "[1] edit employees\n", "[2] edit departments\n", "[3] view database\n", "[0] quit"
        )
    }

    pub fn edit_employees() -> String {
        format!(
            "{}{}{}{}",
            "\n[1] add employee\n",
            "[2] move employee\n",
            "[3] remove employee\n",
            "[0] back to main menu"
        )
    }

    pub fn unknown() -> String {
        format!("{}{}", "\nunknown command!\n", "please try again.\n")
    }
}
