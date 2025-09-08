fn main() {
    //let _s = "hello";
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}")
}
