//    fn main() {
//        println!("Hello, world!");
//        another_function(5, 'c');
//    }

fn another_function(x: i32, unit_label: char) {
    println!("value of x is {x}{unit_label}");
}

fn main() {
    let x = five();
    let y = {
        let x = 6;
        x + 1
    };
    let x = plus_one(x);
    println!("value of y: {y}");
    println!("value of x: {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
