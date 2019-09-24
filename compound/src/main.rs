fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = tup;
    println!("The value of y is {}", _y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

    let a = [1, 2, 3, 4, 5, 6];
    let index = 10;
    let _first = a[0];
    let _second = a[1];

    let element = a[index];

    println!("The value of element is: {}", element);
}
