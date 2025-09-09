fn main() {
    //let _s = "hello";
    let mut s1 = String::from("hello");
    s1.push_str(", world!");
    //let s2 = s1;
    s1 = String::from("ahoy");
    println!("{s1}");
}
