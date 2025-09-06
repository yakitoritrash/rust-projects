//    fn main() {
//        let number = 30;
//        if number < 5 {
//            println!("condition was true");
//        } else {
//            println!("condition was false");
//        }
//    }

fn main() {
    let number = 6;
    if number % 4 == 0 {
        println!("the number is divisible by 4");
    } else if number % 3 == 0 {
        println!("the number is divisible by 3");
    } else if number % 2 == 0 {
        println!("the number is divisible by 2");
    } else {
        println!("the number is not divisible by 2, 3, 4");
    }
}
