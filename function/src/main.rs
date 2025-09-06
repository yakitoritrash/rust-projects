//    fn main() {
//        println!("Hello, world!");
//        another_function(5, 'c');
//    }

fn another_function(x: i32, unit_label: char) {
    println!("value of x is {x}{unit_label}");
}

fn main() {
    let y = {
        let x = 6;
        x
    };
    println!("value of y: {y}");
}
