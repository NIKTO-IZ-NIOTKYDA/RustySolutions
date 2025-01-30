/*
    Task:          4
    Level:         1.2
    Author:        https://github.com/NIKTO-IZ-NIOTKYDA
    Task URL:      https://code.mu/en/rust/tasker/stager/1/2/
    Repository:    https://github.com/NIKTO-IZ-NIOTKYDA/RustySolutions
*/

fn main() {
    let num: i32 = 123;

    let num_str = num.to_string();
    let first_num = num_str.chars().next().unwrap();
    let last_num = num_str.chars().last().unwrap();

    println!("{}", char_to_i32(&first_num) + char_to_i32(&last_num));
}

fn char_to_i32(c: &char) -> i32 {
    return *c as i32 - 0x30;
}
