fn five() -> i32 { 5 }

fn plus_one(x: i32) -> i32 { x + 1 }

fn main() {
    let _x = 5; 

    let y = {
        let x = 3;
        x + 1
    };

    let x = five();

    println!("The value of y is {}", y);
    println!("The value of x is {}", x);
    println!("The value of plus x is {}", plus_one(x));
}
