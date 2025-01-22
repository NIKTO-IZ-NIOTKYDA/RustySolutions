fn main() {
    let txt: &str = "abcde";

    match txt.chars().collect::<Vec<char>>().last() {
        Some(last_char) => {
            println!("{}", last_char);
        }
        None => panic!("String is empty!")
    }
}
