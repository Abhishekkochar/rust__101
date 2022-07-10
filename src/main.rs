fn main() {
    let mut x = 5;
    println!("Hello, world! {}", x);
    x = 10 ;
    println!("Hello, world! {}", x);

    const THREE_HOURS_IN_SECONDS:u32= 60 * 60 * 3;
    println!("Const {}", THREE_HOURS_IN_SECONDS);
    let y = another_fn();
    println!("{}", y);
}

fn another_fn() ->i32 {
    //println!("Another Fn");
    5
}