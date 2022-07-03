fn main() {
    let mut x = 5;
    println!("Hello, world! {}", x);
    x = 10 ;
    println!("Hello, world! {}", x);

    const THREE_HOURS_IN_SECONDS:u32= 60 * 60 * 3;
    println!("Const {}", THREE_HOURS_IN_SECONDS);
    another_fn();
}

fn another_fn() {
    println!("Another Fn");
}