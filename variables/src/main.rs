const MAX_POINTS: u32 = 100_000;

fn bool_f()
{
    let _t = true;
    let _f: bool = false;
}

fn shadow()
{
    let x = 5;
    let x = x + 1;
    let x = x + 2;
    println!("The value of x is {}", x);
}

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    println!("after shadow...");
    shadow();
    println!("after bool...");
    bool_f();
}
