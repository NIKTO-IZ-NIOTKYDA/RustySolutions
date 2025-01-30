/*
    Task:          3
    Level:         1.2
    Author:        https://github.com/NIKTO-IZ-NIOTKYDA
    Task URL:      https://code.mu/en/rust/tasker/stager/1/2/
    Repository:    https://github.com/NIKTO-IZ-NIOTKYDA/RustySolutions
*/

fn main() {
    let num: i32 = 123;

    match String::from(format!("{num}")).chars().collect::<Vec<char>>().first() {
        Some(last_char) => println!("{}", last_char),
        None => panic!("String is empty!")
    }
}
