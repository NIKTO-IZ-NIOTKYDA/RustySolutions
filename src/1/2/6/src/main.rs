/*
    Task:          6
    Level:         1.2
    Author:        https://github.com/NIKTO-IZ-NIOTKYDA
    Task URL:      https://code.mu/en/rust/tasker/stager/1/2/
    Repository:    https://github.com/NIKTO-IZ-NIOTKYDA/RustySolutions
*/

fn main() {
    let num: i32 = 1234;

    println!("{} {} 2",
        num,
        if num % 2 == 0 { "even" } else { "not even" }
    )
}
