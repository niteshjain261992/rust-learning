fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s2 is {}", s3);
    println!("s1 is {}", s2);
}