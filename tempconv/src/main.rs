use std::io;

//    fn main() {
//        let mut celsius = String::new();
//        io::stdin()
//            .read_line(&mut celsius)
//            .expect("Not a number");
//        let celsius: i64 = celsius.trim().parse().expect("Not a number");
//        let fahrenheit : i64 = ((celsius * 9) / 5) + 32 ;
//        println!("{celsius} {fahrenheit}");
//    }

fn main() {
    let mut fahrenheit = String::new(); 
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Error");
    let fahrenheit: f64 = fahrenheit.trim().parse().expect("Not a number");
    let celsius: f64 = ((fahrenheit - 32.0) * 5.0)/9.0;
    println!("{fahrenheit} {celsius}");
}
