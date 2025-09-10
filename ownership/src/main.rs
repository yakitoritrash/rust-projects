fn main() {
//    let _s = "hello";
//    let mut s1 = String::from("hello");
//    s1.push_str(", world!");
//    let s2 = s1;
//    let s2 = s1.clone();
//    s1 = String::from("ahoy");
//    println!("{s1} {s2}");

    let mut s = String::from("gello");
    s = string_func(s);
    println!("{s}");
    let x : i32 = 5;
    int_func(x);
    let s2 = String::from("hello");
    let len = calculate_length(&s2);
    println!("{s2} {len}");
}

fn string_func(some_string: String) ->String {
    println!("{some_string}");
    some_string
}

fn int_func(some_integer: i32) {
    println!("{some_integer}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
