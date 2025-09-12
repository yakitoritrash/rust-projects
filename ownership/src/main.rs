fn main() {
//    let _s = "hello";
//    let mut s1 = String::from("hello");
//    s1.push_str(", world!");
//    let s2 = s1;
//    let s2 = s1.clone();
//    s1 = String::from("ahoy");
//    println!("{s1} {s2}");

//    let mut s = String::from("gello");
//    s = string_func(s);
//    println!("{s}");
//    let x : i32 = 5;
//    int_func(x);
//    let s2 = String::from("hello");
//    let len = calculate_length(&s2);
//    println!("{s2} {len}");

//    let mut s = String::from("hello");
//    //change(&mut s);
//    //{
//        //let r1 = &mut s;
//    //}
//    let r1 = &s;
//    let r2 = &s;
//    println!("{r1} {r2}");
//    let r3 = &mut s;
//    println!("{r3}");

    let reference_to_nothing = no_dangle();
}

//fn dangle() -> &String {
//    let s = String::from("hello");
 //   &s
//}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
//    fn string_func(some_string: String) ->String {
//        println!("{some_string}");
//        some_string
//    }
//
//    fn int_func(some_integer: i32) {
//        println!("{some_integer}");
//    }
//
//    fn calculate_length(s: &String) -> usize {
//        s.len()
//    }

//    fn change(some_string: &mut String) {
//        some_string.push_str(", world!");
//        println!("{some_string}");
//    }
