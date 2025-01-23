/*
    Task:          3
    Level:         1.1
    Author:        https://github.com/NIKTO-IZ-NIOTKYDA
    Task URL:      https://code.mu/ru/rust/tasker/stager/1/1/
    Repository:    https://github.com/NIKTO-IZ-NIOTKYDA/RustySolutions
*/

fn main() {
    let txt: &str = "abcde";

    match txt.chars().collect::<Vec<char>>().last() {
        Some(last_char) => {
            println!("{}", last_char);
        }
        None => panic!("String is empty!")
    }
}
