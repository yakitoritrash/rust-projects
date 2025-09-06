use std::io;
//    fn main() {
//        let sum = 5 + 10;
//        let diff = 99.5 - 4.3;
//        let product = 4 * 30;
//        let quotient = 56/7 / 32.2;
//        let truncated = -5 / 3;
//        let remainder = 43 % 5;
//    }

//    fn main() {
//        let _c = 'z';
//        let _z: char = 'Z';
//        let a = 'あ';
//        println!("{a}");
//    }

//    fn main() {
//        let tup: (i32, f64, u8) = (500, 23.7, 1);
//        let (_x, y, _z) = tup;
//        let five_hundred = tup.0;
//        println!("{y} {five_hundred}")
//    }

fn main() {
    let arr: [i32; 5] = [3, 4, 5, 6, 7];
    let _a = [3; 5];
    let _first = arr[0];
    println!("Input an array index: ");
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num: usize = num
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let index = arr[num];
    println!("The value at {num} is {index}");
}
