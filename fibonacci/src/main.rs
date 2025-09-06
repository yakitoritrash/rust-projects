use std::io;

fn main() {
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("not a number");
    let num: usize = num.trim().parse().expect("err");

    if num == 0 {
        return;
    }

    let mut arr = vec![0; num];
    if num > 1 {
        arr[1] = 1;
    }
    for i in 2..num {
        arr[i] = arr[i - 1] + arr[i - 2];
        println!("{}", arr[i]);
    }
}
