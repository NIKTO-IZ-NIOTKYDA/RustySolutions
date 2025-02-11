/*
    Task:          7
    Level:         1.2
    Author:        https://github.com/NIKTO-IZ-NIOTKYDA
    Task URL:      https://code.mu/en/rust/tasker/stager/1/2/
    Repository:    https://github.com/NIKTO-IZ-NIOTKYDA/RustySolutions
*/

fn main() {
    let num: u8 = 10;

    println!("{}{} is a two-digit number.",
        num,
        if num.to_string().len() == 2 { "" } else { " not" }
    )
}
