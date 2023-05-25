// crud studdent
// CREATE
// READ
// UPDATE
// DELETE

use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
pub struct Student {
    name: String,
    age: i32,
    phone: String,
    email: String,
}

pub struct Students {
    class: HashMap<String, Student>,
}

impl Students {
    fn new() -> Self {
        Self {
            class: HashMap::new(),
        }
    }

    fn add(&mut self, student: Student) {
        self.class.insert(student.name.to_owned(), student);
    }

    fn view_class(&self) -> Vec<&Student> {
        return self.class.values().collect();
    }

    fn delete(&mut self, key: &str) -> bool {
        return self.class.remove(key).is_some();
    }

    fn update(&mut self, key: &str, student: Student) -> bool {
        return self.class.insert(key.to_owned(), student).is_some();
    }
}

mod manager {
    use crate::{ get_input, get_int, Student, Students };

    pub fn add_student(students: &mut Students) {
        let name = match get_input("Enter your name: ".to_string()) {
            Some(input) => input,
            None => {
                return;
            }
        };
        let age = match get_int("Enter your age: ".to_string()) {
            Some(number) => number,
            None => {
                return;
            }
        };
        let phone = match get_input("Enter your phone: ".to_string()) {
            Some(input) => input,
            None => {
                return;
            }
        };
        let email = match get_input("Enter your email: ".to_string()) {
            Some(input) => input,
            None => {
                return;
            }
        };
        let student = Student {
            name,
            age,
            phone,
            email,
        };
        students.add(student);
    }

    pub fn view_student(students: &Students) {
        for student in students.view_class() {
            println!("{:?}", student);
        }
    }

    pub fn remove_student(students: &mut Students) {
        let name = match get_input("Enter your name: ".to_string()) {
            Some(input) => input,
            None => {
                return;
            }
        };
        if students.delete(&name) {
            println!("Delete success!");
        } else {
            println!("Delete failed!");
        }
    }

    pub fn edit_student(students: &mut Students) {
        let key = match get_input("Enter your name: ".to_string()) {
            Some(input) => input,
            None => {
                return;
            }
        };
        // check key exist in students
        if !students.class.contains_key(&key) {
            println!("Student not found!");
            return;
        }

        let age = match get_int("Enter your age: ".to_string()) {
            Some(input) => input,
            None => {
                return;
            }
        };

        let phone = match get_input("Enter your phone: ".to_string()) {
            Some(input) => input,
            None => {
                return;
            }
        };

        let email = match get_input("Enter your email: ".to_string()) {
            Some(input) => input,
            None => {
                return;
            }
        };

        let student = Student { name: key.clone(), age, phone, email };

        if students.update(&key, student) {
            println!("Update success!");
        } else {
            println!("Update failed!");
        }
    }
}

fn get_input(message: String) -> Option<String> {
    println!("{}", message);
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please input again...!");
    }

    let input = buffer.trim().to_owned();

    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_int(message: String) -> Option<i32> {
    let input = match get_input(message) {
        Some(input) => input,
        None => {
            return None;
        }
    };
    let parsed_input: Result<i32, _> = input.parse();

    match parsed_input {
        Ok(input) => Some(input),
        Err(_) => None,
    }
}

enum Manager {
    AdddStudent,
    ViewStudent,
    EditStudent,
    DeleteStudednt,
}

impl Manager {
    fn show() {
        println!("=======================");
        println!("==== Manager Panel ====");
        println!("1. Add Student");
        println!("2. View Students");
        println!("3. Update Student");
        println!("4. Delete Student");
        println!("=======================");
    }

    fn choice(input: &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::AdddStudent),
            "2" => Some(Manager::ViewStudent),
            "3" => Some(Manager::EditStudent),
            "4" => Some(Manager::DeleteStudednt),
            _ => None,
        }
    }
}

fn main() {
    let mut students = Students::new();

    loop {
        Manager::show();
        let input = get_input("Enter your choice: ".to_string()).expect("Please enter your data: ");
        match Manager::choice(&input) {
            Some(Manager::AdddStudent) => manager::add_student(&mut students),
            Some(Manager::ViewStudent) => manager::view_student(&mut students),
            Some(Manager::EditStudent) => manager::edit_student(&mut students),
            Some(Manager::DeleteStudednt) => manager::remove_student(&mut students),
            None => {
                return;
            }
        }
    }
}
