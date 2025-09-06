//    fn main() {
//        let number = 30;
//        if number < 5 {
//            println!("condition was true");
//        } else {
//            println!("condition was false");
//        }
//    }

//    fn main() {
//        let number = 6;
//        if number % 4 == 0 {
//            println!("the number is divisible by 4");
//        } else if number % 3 == 0 {
//            println!("the number is divisible by 3");
//        } else if number % 2 == 0 {
//            println!("the number is divisible by 2");
//        } else {
//            println!("the number is not divisible by 2, 3, 4");
//        }
//    }

//    fn main() {
//        loop {
//            println!("again!");
//        }
//    }

//    fn main() {
//        let mut counter = 0;
//        let result = loop {
//            counter += 1;
//            if counter == 10 {
//                break counter * 2;
//            }
//        };
//        println!("result: {result}, counter: {counter}");
//    }

//    fn main() {
//        let mut count = 0;
//        'counting_up: loop {
//            println!("count = {count}");
//            let mut remaining = 10;
//
//            loop {
//                println!("remaining: {remaining}");
//                if remaining == 9 {
//                    break;
//                }
//                if count == 2 {
//                    break 'counting_up;
//                }
//                remaining -= 1;
//            }
//            count += 1;
//        }
//        println!("End count = {count}");
//    }

fn main() {
    let mut number = 3;
    
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!");
}
