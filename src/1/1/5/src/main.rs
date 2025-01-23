/*
    Task:          5
    Level:         1.1
    Author:        https://github.com/NIKTO-IZ-NIOTKYDA
    Task URL:      https://code.mu/en/rust/tasker/stager/1/1/
    Repository:    https://github.com/NIKTO-IZ-NIOTKYDA/RustySolutions
*/

fn main() {
    let word: &str = "day";
    
    if word.len() < 2 {
        panic!("The word is less than two characters")
    }

    let two_last_char: Vec<char> = word.chars().rev().take(2).collect();
    let result = if two_last_char[0] == 'ь' { two_last_char[1] } else { two_last_char[1] };

    println!("{result}");
}
