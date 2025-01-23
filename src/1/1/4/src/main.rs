fn main() {
    let word1: &str = "abc";
    let word2: &str = "ade";

    if word1.len() == 0 || word2.len() == 0 {
        panic!("String is empty!");
    }

    let first_bytes1: u8 = word1.as_bytes()[0];
    let first_bytes2: u8 = word2.as_bytes()[0];

    println!(
        "{}",
        if first_bytes1 == first_bytes2 { "Match" } else { "Not match" }
    );
}
