fn n_th_fibonacci_number(n: i32) -> i32 {
    let mut left = 1;
    let mut right = 1;

    let mut counter = 2;

    while counter <= n {
        counter += 1;
        let num = left + right;
        println!("debug - counter={counter}, num={num}");
        left = right;
        right = num; 
    }
    right
}

fn main() {
    println!("{}", n_th_fibonacci_number(8));
}
