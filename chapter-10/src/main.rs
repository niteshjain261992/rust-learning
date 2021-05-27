fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest();
    println!("The longest string is {}", result);
}

fn longest() -> &'static str {
    let result: &str = "really long string";
    result
}
